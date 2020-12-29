//! TODO: Process all the things

use crate::cli::State;
use crate::plot::create_plot;

use chrono::{Datelike, Month, NaiveDate};
use num_traits::FromPrimitive;

enum DayAction {
    InstallmentDue,
    ExtraDownpayment(f64),
}

#[derive(Debug)]
struct DailyResult {
    pub date: NaiveDate,
    /// Amount required to payback on this date.
    pub installment: f64,
    /// An additional downpayment on this date.
    pub additional_payment: f64,
    /// Fee related to nominal downpayment
    pub fee: i32,
    /// The amount of interest occuring on this date, not posted.
    pub interest: f64,
    /// The amount of interest that has accumulated to date, not posted to current loan
    pub accumulated_interest: f64,
    /// On this date, the total interest posted to the loan.
    pub posted_interest: f64,
    /// The total remainder of the loan as of date.
    pub current_loan: f64,
}

#[derive(Debug)]
pub struct MonthlyResult {
    pub month: u32,
    pub year: i32,
    pub fee: i32,
    pub interest: f64,
    pub payed_back: f64,
    pub current_loan: f64,
}

#[derive(Debug)]
pub struct TotalResult {
    pub total_cost: f64,
    pub loan: i64,
    pub interest: f64,
    pub fee: i32,
    pub completed_terms: i32,
    pub planned_terms: i32,
}

fn calculate_first_term_due_date(state: &State) -> NaiveDate {
    // Get first term due date
    if state.loan_start_date.day() > state.term_due_day {
        let month = Month::from_u32(state.loan_start_date.month())
            .unwrap()
            .succ();
        let year = if month.number_from_month() == 1 {
            state.loan_start_date.year() + 1
        } else {
            state.loan_start_date.year()
        };

        NaiveDate::from_ymd(year, month.number_from_month(), state.term_due_day)
    } else {
        NaiveDate::from_ymd(
            state.loan_start_date.year(),
            state.loan_start_date.month(),
            state.term_due_day,
        )
    }
}

fn calculate_due_term_dates(state: &State) -> Vec<NaiveDate> {
    let first_term_due_date = calculate_first_term_due_date(&state);

    // Calculate each date we have a due term.
    let month_increase = match state.terms_per_year {
        1 => 12,
        2 => 6,
        3 => 4,
        4 => 3,
        6 => 2,
        12 => 1,
        _ => unreachable!(),
    };
    let mut due_term_dates: Vec<NaiveDate> = Vec::new();
    let mut current_due = first_term_due_date;
    while (due_term_dates.len() as i64) < state.terms.into() {
        let month = ((current_due.month0() + month_increase) % 12) + 1;
        let year = if month < current_due.month() {
            current_due.year() + 1
        } else {
            current_due.year()
        };
        due_term_dates.push(current_due);
        current_due = NaiveDate::from_ymd(year, month, state.term_due_day);
    }

    due_term_dates
}

fn calculate_extra_payment_dates(
    terms: u32,
    amount: f64,
    first_term_date: NaiveDate,
) -> Vec<(NaiveDate, f64)> {
    let mut current_due = first_term_date;
    let mut extra_payment_dates: Vec<(NaiveDate, f64)> = Vec::new();
    for _ in 0..terms {
        let month = ((current_due.month0() + 1) % 12) + 1;
        let year = if month < current_due.month() {
            current_due.year() + 1
        } else {
            current_due.year()
        };
        let day = current_due.day();
        extra_payment_dates.push((current_due, amount));
        current_due = NaiveDate::from_ymd(year, month, day);
    }

    extra_payment_dates
}

fn calculate_annulity_term_payment(state: &State) -> f64 {
    // TEMP: calculate an annulity for our loan
    // C = start capital
    // r = nominal annual interest rate
    // n = number of installments per year
    // N = total number of installments
    //
    // top = C * (r/n)
    // bottom = 1 - (1 + (r/n))^N
    // installment = top / bottom

    let top =
        state.loan as f64 * ((state.effective_interest / 100f64) / state.terms_per_year as f64);
    let power_result = f64::powi(
        1f64 + ((state.effective_interest / 100f64) / state.terms_per_year as f64),
        -state.terms,
    );
    let bottom = 1f64 - power_result;

    top / bottom
}

fn compute_day_actions(
    state: &State,
    mut due_term_dates: Vec<NaiveDate>,
    mut extra_payments: Vec<Vec<(NaiveDate, f64)>>,
) -> Vec<(NaiveDate, Vec<DayAction>)> {
    let mut action_dates: Vec<(NaiveDate, Vec<DayAction>)> = Vec::new();
    // Loop though each day until we have crossed off the last term
    let mut current = state.loan_start_date.pred();
    loop {
        current = current.succ();

        let mut actions = Vec::new();

        // Check if current date is a term due date
        if current == due_term_dates[0] {
            due_term_dates.remove(0);
            actions.push(DayAction::InstallmentDue);
        }

        for extra in extra_payments.iter_mut() {
            if let Some((extra_date, amount)) = extra.first() {
                if extra_date == &current {
                    actions.push(DayAction::ExtraDownpayment(amount.clone()));
                    extra.remove(0);
                }
            }
        }

        action_dates.push((current.clone(), actions));

        if due_term_dates.len() == 0 {
            break;
        }
    }

    action_dates
}

pub fn process() {
    let state = match crate::cli::parse() {
        Ok(state) => state,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    println!("{:?}", state);

    println!("Starting loan payout date from {}", state.loan_start_date);

    let due_term_dates = calculate_due_term_dates(&state);
    let planned_terms = due_term_dates.len();

    let first_term_due_date = calculate_first_term_due_date(&state);
    println!("First term due {}", first_term_due_date);
    let first_extra_date = NaiveDate::from_ymd(
        first_term_due_date.year(),
        first_term_due_date.month(),
        state.extra_payment_day,
    );

    let extra_payment_dates = calculate_extra_payment_dates(
        state.extra_terms,
        state.extra_amount as f64,
        first_extra_date,
    );
    let action_dates = compute_day_actions(&state, due_term_dates, vec![extra_payment_dates]);

    let term_payment = calculate_annulity_term_payment(&state);
    println!("Term payment: {}", term_payment);

    // Iterate actions_dates to calculate daily_result
    let mut accumulated: f64 = 0.0;
    let mut current_loan: f64 = state.loan as f64;
    let daily_result: Vec<DailyResult> = action_dates
        .into_iter()
        .filter_map(|(date, actions)| {
            if current_loan == 0.0 {
                return None;
            }

            let interest = (current_loan * (state.nominal_interest / 100f64)) / 365f64;
            let mut fee = 0;
            let mut installment = 0.0;
            let mut additional_payment = 0.0;
            let accumulated_interest;
            let mut posted_interest = 0.0;

            accumulated += interest;

            // Iterate actions to calculate DayAction parameters
            for a in actions.iter() {
                match a {
                    DayAction::InstallmentDue => {
                        let loan_after_increase = current_loan + accumulated + state.fee as f64;
                        let current_term_payment = if term_payment > loan_after_increase {
                            loan_after_increase
                        } else {
                            term_payment + state.fee as f64
                        };

                        // Update daily state
                        installment = current_term_payment;
                        fee = state.fee;
                        posted_interest = accumulated;

                        // Update global state
                        accumulated = 0.0;
                        current_loan = loan_after_increase - current_term_payment;
                    }
                    DayAction::ExtraDownpayment(amount) => {
                        additional_payment += amount;
                        current_loan -= amount;
                    }
                }
            }

            accumulated_interest = accumulated;

            let daily = DailyResult {
                date,
                fee,
                installment,
                additional_payment,
                interest,
                accumulated_interest,
                posted_interest,
                current_loan,
            };

            Some(daily)
        })
        .collect();

    let mut installment_sum = 0.0;
    let monthly_result: Vec<MonthlyResult> = daily_result
        .iter()
        .filter_map(|x| {
            installment_sum += x.installment;
            installment_sum += x.additional_payment;
            if x.fee == 0 {
                None
            } else {
                let installment = installment_sum;
                installment_sum = 0.0;
                Some(MonthlyResult {
                    year: x.date.year(),
                    month: x.date.month(),
                    fee: x.fee,
                    interest: x.posted_interest,
                    payed_back: installment,
                    current_loan: x.current_loan,
                })
            }
        })
        .collect();

    let total = TotalResult {
        total_cost: monthly_result.iter().map(|s| s.payed_back).sum(),
        fee: monthly_result.iter().map(|s| s.fee).sum(),
        interest: monthly_result.iter().map(|s| s.interest).sum(),
        loan: state.loan,
        completed_terms: monthly_result.len() as i32,
        planned_terms: planned_terms as i32,
    };

    println!("{:#?}", total);

    create_plot(monthly_result, total).unwrap();
}
