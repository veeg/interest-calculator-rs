//! Implementation of the public API to consume the calculations.

use crate::events::*;
use crate::reports::*;

use chrono::NaiveDate;

/// This is an interactive structure used to construct and alter the events
/// within an installment loan calculations.
pub struct InteractiveCalculator {
    /// The set of events, ordered by date.
    /// The first element is guaranteed to be LoanEvent::Initial,
    /// meaning no later element may have a date prior to the LoanEvent::Initial date.
    events: Vec<(NaiveDate, LoanEvent)>,
}

impl InteractiveCalculator {
    /// Construct a new InteractiveCalculator with the initial loan event.
    pub fn new(date: NaiveDate, initial: LoanInitialization) -> Self {
        InteractiveCalculator {
            events: vec![(date, LoanEvent::Initial(initial))],
        }
    }

    /// Alter the loan payout day used registered for the LoanEvent::Initial.
    ///
    /// This will modify all subsequent event dates by the same days offset
    /// to correct the timeline previously established.
    pub fn change_initial_payout_date(&mut self, _date: NaiveDate) {
        todo!("implement me");
    }

    /// Compute the installment loan result for the lifetime of the loan based on current events.
    pub fn compute(&self) -> (TotalResult, Vec<MonthlyResult>, Vec<DailyResult>) {
        // This is the bread and butter of computing the result of the installment loan.
        // We have to distill the loan down into a current state basis, which we evaluate
        // for each day we progress. We alter the current state based on events, and continue
        // iterating until we no longer have any more installments.

        // TODO: Replace this with the plan outlined above.
        return self.legacy_implementation();
    }

    fn legacy_implementation(&self) -> (TotalResult, Vec<MonthlyResult>, Vec<DailyResult>) {
        let (loan_start_date, initial) = self.events.first().unwrap();
        let initial = initial.initial();

        let terms_per_year = initial.terms_per_year.to_u32() as i32;

        // Calculate effective interest rate
        let effective_interest = 1.0 + ((initial.nominal_interest / 100.0) / terms_per_year as f64);
        let effective_interest = f64::powi(effective_interest, terms_per_year);
        let effective_interest = effective_interest - 1.0;
        let effective_interest = effective_interest * 100.0;

        let state = crate::process::State {
            loan: initial.loan as i64,
            nominal_interest: initial.nominal_interest,
            effective_interest,
            fee: initial.installment_fee as i32,

            loan_start_date: loan_start_date.clone(),
            term_due_day: initial.due_within_month.to_u32(),

            terms: initial.terms,
            terms_per_year,

            extra_terms: 0,
            extra_payment_day: 1,
            extra_amount: 0,
        };

        return crate::process::process(state);
    }
}
