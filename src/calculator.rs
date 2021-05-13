//! Implementation of the public API to consume the calculations.

use crate::events::*;
use crate::reports::*;

use chrono::{Datelike, Month, NaiveDate};
use num_traits::FromPrimitive;
use std::collections::{BTreeMap, VecDeque};

#[derive(Debug)]
pub enum CompoundingStrategy {
    Daily,
    OnInstallment,
    EndOfMonth,
    EndOfYear,
}

#[derive(Debug)]
enum NotableEvents {
    Initialization(f64),
    RepaymentInstallment(f64),
    InterestOnlyInstallment(f64),
    ExtraInstallment(f64),
}

/// The daily result produced by a Calculator.
/// All fields here represent the state on the date of the status report.
#[derive(Debug)]
struct Daily {
    /// The date of this Daily status report.
    pub date: NaiveDate,

    /// The amount of interest accrued on this date.
    pub accrued_interest: f64,
    /// If any interest was compounded into the principal loan, this is represented here.
    pub compounded_interest: f64,
    /// The amount disbursed on the loan.
    pub disbursed: f64,
    /// The total number of fees accrued
    pub fee: f64,
    /// If anything was repayed to the loan, it is reflected in this status.
    pub repayed: f64,
    /// The portion of the repayed status that is an ordrinary repayment portion.
    pub repayment_installment: f64,
    /// The portion of the repayed status that is the accrued interest portion.
    pub interest_installment: f64,
    /// The portion of the repayed status that is due to extraordinary installment.
    pub extra_installment: f64,

    /// Notable events that occurred on this date.
    pub notable_events: Vec<NotableEvents>,
}

#[derive(Debug, Eq, PartialEq)]
enum InstallmentType {
    InterestOnly,
    Repayment,
}

/// The operations that can occur on a single day.
/// Multiple actions can be specified.
#[derive(Debug, Default)]
struct DayActions {
    /// An administration cost to the initialization of the installment loan.
    initialization: Option<(f64, f64)>,
    /// The interest on the outstanding principal loan sum is accumulating into a non-posted
    /// position. This is a daily action.
    interest_accumulating: bool,
    /// The accumulated, non-posted interest is compounded into the principal loan.
    interest_compounding: bool,
    /// Do we have any installment action for this day?
    installment: Option<InstallmentType>,
    /// An additional repayment on the principal loan.
    extra_installments: Vec<f64>,
}

/// This structure is used to hold the current state of a calculation.
#[derive(Debug)]
struct CurrentCalculationState {
    interest_compounding_strategy: CompoundingStrategy,

    /// This is total number of terms we plan to to repay the full loan
    /// where each scheduled installment contains a repayment portion.
    /// Interest-only installments does not count.
    ///
    /// If we hit a refinance event that alter the number of planned repayment terms,
    /// the new sum is calculated based on completed_repayment_terms.
    ///
    /// To get the outstanding planned terms, simply subtract completed_repayment_terms
    /// from planned_repayment_terms.
    planned_repayment_terms: u32,
    /// Total number of scheduled repayment installments.
    completed_repayment_terms: u32,

    /// The original principal loan sum used to calculate the basis of the loan.
    /// This reflects the amount disbursed, including fees, and is used to compute term payments.
    original_outstanding_loan: f64,
    /// This is non-posted interest accumulated.
    accrued_interest: f64,
    accrued_interest_since_last_installment: f64,

    /// The currently valid nominal interest for the outstanding loan.
    current_nominal_interest: f64,
    /// The current principal loan sum needed to be re-payed.
    current_outstanding_loan: f64,
    /// The fee applied when a scheduled re-payment installment is payed.
    current_installment_fee: f64,
    /// Configured number of terms per year for this loan.
    /// This effectively communicates the interval between repayment installments.
    current_terms_per_year: TermsPerYear,
    current_monthly_due_day: MonthlyDueDate,

    /// This is the computed date for our next installment, regardless of type.
    /// Calculated based external date factor, terms per year and monthly due date.
    /// This value is re-calculated on in several day actions or loan events.
    computed_installment_date: NaiveDate,

    /// Computed effective interest. Recomputed when any of its parameters change.
    computed_effective_interest: f64,
    /// The computed term payment.
    computed_term_payment: f64,
}

/// This is an interactive structure used to construct and alter the events
/// within an installment loan calculations.
pub struct InteractiveCalculator {
    /// The set of events, ordered by date.
    /// The first element is guaranteed to be LoanEvent::Initial,
    /// meaning no later element may have a date prior to the LoanEvent::Initial date.
    events: BTreeMap<NaiveDate, Vec<LoanEvent>>,
}

impl InteractiveCalculator {
    /// Construct a new InteractiveCalculator with the initial loan event.
    pub fn new(date: NaiveDate, initial: LoanInitialization) -> Self {
        let mut map = BTreeMap::new();
        map.insert(date, vec![LoanEvent::Initial(initial)]);
        InteractiveCalculator { events: map }
    }

    /// Add an extra installment event to the calculator.
    pub fn add_event_extra_single(
        &mut self,
        date: NaiveDate,
        extra: LoanExtraInstallment,
    ) -> Result<(), String> {
        // Simply convert it to a recurring one with count 1, and delegate to other method.
        let extra = LoanRecurringExtraInstallments {
            amount: extra.amount,
            count: std::num::NonZeroU32::new(1).unwrap(),
            recurring_interval: RecurringInterval::Monthly,
        };

        self.add_event_extra_recurring(date, extra)
    }

    /// Add an extra installment event to the calculator.
    // TODO: Make this public once we've implemented it properly
    fn add_event_extra_recurring(
        &mut self,
        date: NaiveDate,
        extra: LoanRecurringExtraInstallments,
    ) -> Result<(), String> {
        // TODO: Sanity check date
        self.events
            .entry(date)
            .and_modify(|e| e.push(LoanEvent::Extra(extra.clone())))
            .or_insert(vec![LoanEvent::Extra(extra)]);
        Ok(())
    }

    /// Compute the installment loan result for the lifetime of the loan based on current events.
    pub fn compute(&self) -> Result<TotalResult, String> {
        let mut events_iter = self.events.iter();

        // SAFETY(unwrap): events vector always contains 1 element.
        let (payout_date, initial) = events_iter.next().unwrap();
        if initial.len() > 1 {
            return Err(
                "Unexpected amount of events on loan initialization, expected one".to_string(),
            );
        }
        // SAFETY(unwrap): guarded to contain one item.
        let initial = initial.first().unwrap();

        let initial = initial.initial();
        if initial.loan <= 0.0 {
            return Err(
                "expecting non-zero positive value for outstanding loan in initial loan event"
                    .to_string(),
            );
        }
        if initial.nominal_interest <= 0.0 {
            return Err(
                "expecting non-zero positive value for nominal interest in initial loan event"
                    .to_string(),
            );
        }

        let mut state = initial_computing_state(payout_date, initial);

        // Calculate future actions based on initial
        let mut daily_actions = compute_actions_on_disbursement(
            initial.loan,
            initial.administration_fee,
            &payout_date,
            &state,
        );

        // We can now consume future events as their date approaches.
        let mut potential_events = events_iter.next();

        let mut dailys = Vec::new();
        for current_date in payout_date.iter_days() {
            // Handle events that may alter the daily_actions
            match potential_events {
                Some((event_date, next_events)) if event_date == &current_date => {
                    // A day can have multiple events
                    for day_event in next_events.iter() {
                        match day_event {
                            LoanEvent::Extra(schedule) => {
                                // Handle the extra payment
                                // TODO: Support more than single day extra payment
                                if let Some((action_date, action)) = daily_actions.get_mut(0) {
                                    debug_assert!(action_date == event_date);
                                    action.extra_installments.push(schedule.amount)
                                }
                            }
                            _ => {}
                        }
                    }

                    potential_events = events_iter.next();
                }
                Some((event_date, _)) if event_date < &current_date => {
                    return Err("event_date is in the past".to_string());
                }
                Some(_) => {}
                None => {}
            }

            // Retrieve this days actions.
            let actions = match Self::fetch_date_action(current_date, &mut daily_actions) {
                Ok(Some(a)) => a,
                Ok(None) => continue,
                Err(e) => {
                    // We currently panic on this action.
                    // This is ONLY due a an inconsistency in the day actions implementation.
                    dbg!(current_date, &state);
                    return Err(e);
                }
            };

            // Process this days actions.
            let (daily, finished) = Self::process_day_action(&mut state, current_date, actions);
            dailys.push(daily);

            if finished {
                break;
            }
        }

        return Ok(TotalResult {
            total_cost: dailys.iter().map(|x| x.repayed).sum(),
            total_loan: dailys.iter().map(|x| x.disbursed).sum(),
            total_repayment_installment: dailys.iter().map(|x| x.repayment_installment).sum(),
            total_extra_installment: dailys.iter().map(|x| x.extra_installment).sum(),
            total_interest: dailys.iter().map(|x| x.compounded_interest).sum(),
            total_fee: dailys.iter().map(|x| x.fee).sum(),

            disbursement_date: dailys.first().unwrap().date.clone(),
            first_installment_date: dailys
                .iter()
                .find(|x| x.repayment_installment > 0.0)
                .map_or(NaiveDate::from_ymd(1970, 1, 1), |x| x.date.clone()),
            end_date: dailys.last().unwrap().date.clone(),
            planned_terms: state.planned_repayment_terms as i32,
            completed_terms: state.completed_repayment_terms as i32,
        });
    }

    /// Returns Ok(Some(...)) if actions array has actions for this date.
    /// Returns Ok(None) if there are no actions for this date
    /// Return Err(...) on fatal error (logic break)
    fn fetch_date_action(
        date: NaiveDate,
        actions: &mut VecDeque<(NaiveDate, DayActions)>,
    ) -> Result<Option<DayActions>, String> {
        // TODO: Improve this data structure - we can do better
        // Fetch this days actions
        match actions.front() {
            Some((action_date, _)) => {
                if action_date > &date {
                    // Date in the future. Continue with none
                    return Ok(None);
                }
            }
            None => {
                // The actions set is empty
                // This is effectively a condition where we cannot terminate
                return Err(
                    "no more daily actions, yet we have not terminated computation".to_string(),
                );
            }
        }

        if let Some((action_date, a)) = actions.pop_front() {
            if action_date < date {
                return Err(
                    "action events where in the past - this means we skipped some events"
                        .to_string(),
                );
            }
            return Ok(Some(a));
        };

        unreachable!("pop_front should be guarded");
    }

    /// Process a single day of actions.
    /// This will calculator state
    fn process_day_action(
        state: &mut CurrentCalculationState,
        date: NaiveDate,
        actions: DayActions,
    ) -> (Daily, bool) {
        // This it the daily parameters
        let mut finished = false;
        let mut daily_accrued_interest = 0.0;
        let mut daily_repayed = 0.0;
        let mut daily_interest_installment = 0.0;
        let mut daily_repayment_installment = 0.0;
        let mut daily_extra_installment = 0.0;
        let mut daily_compounded_interest = 0.0;
        let mut daily_fees = 0.0;
        let mut daily_disbursed = 0.0;

        let mut notable = Vec::new();

        if let Some((amount, fee)) = actions.initialization {
            state.current_outstanding_loan += amount + fee;
            state.original_outstanding_loan = state.current_outstanding_loan;
            daily_fees += fee;
            daily_disbursed += amount;

            state.computed_effective_interest = effective_interest(
                state.current_nominal_interest,
                state.current_terms_per_year.to_u32(),
            );
            state.computed_term_payment = annuity_term_payment(
                state.original_outstanding_loan,
                state.computed_effective_interest,
                state.current_terms_per_year.to_u32(),
                state.planned_repayment_terms - state.completed_repayment_terms,
            );
            notable.push(NotableEvents::Initialization(amount));
        }

        // Accumulate interest on outstanding principal loan.
        if actions.interest_accumulating {
            // We currently implement interest accumulation by daily increment.
            daily_accrued_interest = (state.current_outstanding_loan
                * (state.current_nominal_interest / 100f64))
                / 365f64;
            state.accrued_interest += daily_accrued_interest;
            state.accrued_interest_since_last_installment += daily_accrued_interest;
        }

        // If any extra installments have been scheduled on this day, we need to account
        // for it.
        //
        // TODO: Check if the extra installment would brind the outstanding loan to zero.
        // Could be useful to do this check after installment check as well.
        for extra in actions.extra_installments.iter() {
            daily_extra_installment += extra;
            daily_repayed += extra;
            state.current_outstanding_loan -= extra;

            notable.push(NotableEvents::ExtraInstallment(*extra));
        }

        // Check if we should post the accrued interest to the loan
        if actions.interest_compounding {
            daily_compounded_interest = state.accrued_interest;
            state.accrued_interest = 0.0;
            state.current_outstanding_loan += daily_compounded_interest;
        }

        // Installment on repayment - this includes repayment of an interest portion.
        match actions.installment {
            Some(InstallmentType::Repayment) => {
                // TODO(serial loans): Once we support serial loans, this term payment
                // must include the computed accrued interest
                let term_payment = state.computed_term_payment;

                // Check if the current outstanding loan, including non-posted interest,
                // could be fulfilled by a complete term payment.
                let total = state.current_outstanding_loan
                    + state.current_installment_fee
                    + state.accrued_interest;
                let payment = if term_payment > total {
                    finished = true;
                    total
                } else {
                    term_payment + state.current_installment_fee
                };

                // Update some daily metrics.
                daily_interest_installment = state.accrued_interest_since_last_installment;
                daily_fees += state.current_installment_fee;
                daily_repayed += payment;

                state.accrued_interest_since_last_installment = 0.0;
                state.current_outstanding_loan += state.current_installment_fee;

                // Special-case the last installment. If we have NOT compounded the interest
                // on this day, we may have non-posted interest we should include in the final
                // payment.
                // We do NOT update the daily_interest_installment with the accrued_interest,
                // since this is already taken into account by
                // accrued_interest_since_last_installment.
                if payment == total && state.accrued_interest > 0.0 {
                    daily_compounded_interest += state.accrued_interest;
                    state.current_outstanding_loan += state.accrued_interest;
                    state.accrued_interest = 0.0;
                }

                daily_repayment_installment = payment - daily_interest_installment;
                state.current_outstanding_loan -= payment;
                state.completed_repayment_terms += 1;
                notable.push(NotableEvents::RepaymentInstallment(payment));

                // TODO: We have some corner cases where our calculations are off
                // with very low numbers. This has manifested itself as negative
                // current_outstanding_loan. We workaround this issue now
                // so we dont end up with a panic, and look into addressing this
                // situation later on.
                if state.current_outstanding_loan < 0.0 {
                    finished = true;
                }
            }
            Some(InstallmentType::InterestOnly) => {
                // We only process a interest installment if we have not processed a repayment.
                daily_interest_installment =
                    state.accrued_interest_since_last_installment + daily_accrued_interest;
                state.accrued_interest_since_last_installment = 0.0;
                daily_repayed += daily_interest_installment;
                notable.push(NotableEvents::InterestOnlyInstallment(
                    daily_interest_installment,
                ));
            }
            None => {}
        }

        let daily = Daily {
            date,
            accrued_interest: daily_accrued_interest,
            compounded_interest: daily_compounded_interest,
            disbursed: daily_disbursed,
            fee: daily_fees,
            repayed: daily_repayed,
            repayment_installment: daily_repayment_installment,
            interest_installment: daily_interest_installment,
            extra_installment: daily_extra_installment,

            notable_events: notable,
        };

        (daily, finished)
    }
}

fn initial_computing_state(
    payout_date: &NaiveDate,
    initial: &LoanInitialization,
) -> CurrentCalculationState {
    let computed_installment_date = installment_date_from_target_month(
        payout_date,
        initial.due_within_month,
        initial.first_installment_month,
    );

    CurrentCalculationState {
        interest_compounding_strategy: CompoundingStrategy::OnInstallment,
        planned_repayment_terms: initial.terms,
        completed_repayment_terms: 0,
        original_outstanding_loan: 0.0,

        accrued_interest: 0.0,
        accrued_interest_since_last_installment: 0.0,

        current_nominal_interest: initial.nominal_interest,
        current_outstanding_loan: 0.0,
        current_installment_fee: initial.installment_fee,
        current_terms_per_year: initial.terms_per_year,
        current_monthly_due_day: initial.due_within_month,

        computed_installment_date,
        computed_effective_interest: 0.0,
        computed_term_payment: 0.0,
    }
}

/// Compute the future set of DayActions for the remainder of this loan.
///
/// NOTE: The disbursement that happeans on the first day is the ONLY
/// action that scheduled for this date.
fn compute_actions_on_disbursement(
    amount: f64,
    fee: f64,
    current_date: &NaiveDate,
    state: &CurrentCalculationState,
) -> VecDeque<(NaiveDate, DayActions)> {
    // Set of return actions
    let mut all_actions = {
        let a = DayActions {
            initialization: Some((amount, fee)),
            ..Default::default()
        };
        let mut v = VecDeque::new();
        v.push_front((current_date.clone(), a));
        v
    };

    let mut completed_repayments = 0;
    let mut skip_installments = 0;
    let mut next_installment_date = state.computed_installment_date;

    for date in current_date.succ().iter_days() {
        if completed_repayments == state.planned_repayment_terms {
            break;
        }

        let mut actions = DayActions {
            initialization: None,
            interest_accumulating: true,
            interest_compounding: false,
            installment: None,
            extra_installments: Vec::new(),
        };

        // Check for if we have any installment type for today
        actions.installment = if next_installment_date == date && skip_installments > 0 {
            skip_installments -= 1;
            Some(InstallmentType::InterestOnly)
        } else if next_installment_date == date {
            completed_repayments += 1;
            Some(InstallmentType::Repayment)
        } else {
            None
        };

        actions.interest_compounding = match state.interest_compounding_strategy {
            CompoundingStrategy::EndOfYear => {
                if date.succ().year() != date.year() {
                    true
                } else {
                    false
                }
            }
            CompoundingStrategy::EndOfMonth => {
                if date.succ().month() != date.month() {
                    true
                } else {
                    false
                }
            }
            CompoundingStrategy::OnInstallment => {
                if date == next_installment_date {
                    true
                } else {
                    false
                }
            }
            CompoundingStrategy::Daily => true,
        };

        // Update next_installment_date, if needed
        if date == next_installment_date {
            next_installment_date = installment_date_from_interval(
                &date,
                state.current_monthly_due_day,
                state.current_terms_per_year,
            );
        }

        all_actions.push_back((date, actions));
    }

    all_actions
}

/// Calculate the next installment date based on:
/// - current date
/// - desired due day of month
/// - term interval
///
/// The calculation will start with the term interval offset from current date.
fn installment_date_from_interval(
    current: &NaiveDate,
    due: MonthlyDueDate,
    interval: TermsPerYear,
) -> NaiveDate {
    let increase: u32 = match interval {
        TermsPerYear::One => 12,
        TermsPerYear::Two => 6,
        TermsPerYear::Three => 4,
        TermsPerYear::Four => 3,
        TermsPerYear::Six => 2,
        TermsPerYear::Twelve => 1,
    };
    let month = future_month(current, increase);
    return installment_date_from_target_month(current, due, month);
}

/// Calculate the future month based on num month increments from provided date.
pub(crate) fn future_month(date: &NaiveDate, increase: u32) -> Month {
    // Calculate the increased target month
    let month = (date.month0() + increase) % 12;
    // SAFETY(unwrap): Calculation above places it in 0-11 range, whilst this method takes 1-12
    // range. Thus, the +1 brings the allowed range into bounds.
    Month::from_u32(month + 1).unwrap()
}

/// Calculate the installment date based on:
/// - current date
/// - desired month
/// - day within month
fn installment_date_from_target_month(
    current: &NaiveDate,
    due: MonthlyDueDate,
    target_month: Month,
) -> NaiveDate {
    let mut year = current.year();
    if target_month.number_from_month() < current.month() {
        year += 1;
    }

    // If we are within the same month, we must assess if the target month is a full
    // year in advance.
    let mut day = due.to_u32();
    if target_month.number_from_month() == current.month() && current.day() >= day {
        year += 1;
    }

    // Attempt to fully reconstruct a valid date.
    loop {
        if let Some(valid) = NaiveDate::from_ymd_opt(year, target_month.number_from_month(), day) {
            break valid;
        }
        day -= 1;
    }
}

fn effective_interest(nominal_interest: f64, compounding_terms: u32) -> f64 {
    let effective = 1.0 + ((nominal_interest / 100.0) / compounding_terms as f64);
    let effective = f64::powi(effective, compounding_terms as i32);

    (effective - 1.0) * 100.0
}

fn annuity_term_payment(
    principal: f64,
    effective_interest: f64,
    terms_per_year: u32,
    total_terms: u32,
) -> f64 {
    // C = principal loan
    // r = effective interest rate
    // n = number of installments per year
    // N = total number of installments
    //
    // top = C * (r/n)
    // bottom = 1 - (1 + (r/n))^N
    // installment = top / bottom

    let top = principal * ((effective_interest / 100f64) / terms_per_year as f64);
    let power_result = f64::powi(
        1f64 + ((effective_interest / 100f64) / terms_per_year as f64),
        -(total_terms as i32),
    );
    let bottom = 1f64 - power_result;

    top / bottom
}

#[cfg(test)]
mod tests {
    use super::{installment_date_from_target_month, MonthlyDueDate};
    use chrono::{Month, NaiveDate};

    #[test]
    fn installment_date_from_target_month_next_month() {
        let today = NaiveDate::from_ymd(2021, 1, 10);
        let monthly_due_day = MonthlyDueDate::First;
        let target_month = Month::February;

        let result = installment_date_from_target_month(&today, monthly_due_day, target_month);
        assert_eq!(NaiveDate::from_ymd(2021, 2, 1), result);
    }
}
