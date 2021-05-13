//! Models related to the reports of calculations

use chrono::NaiveDate;

/// This report includes the total computation of an installment loan.
#[derive(Debug)]
pub struct TotalResult {
    /// The total cost of this loan, including the principal loan sum.
    /// This field is a sum of the other totals in this result structure.
    pub total_cost: f64,
    /// The total principal loan sum, aggergated over any additional refinancing.
    pub total_loan: f64,
    /// The total sum of the disbursed loan has been repayed in nominal repayments
    /// according to the valid repayment plan at any one time.
    pub total_repayment_installment: f64,
    /// The total sum of the disbursed loan that has been repayed in extra installments.
    pub total_extra_installment: f64,
    /// The total sum of interest payed on the loan over its duration.
    pub total_interest: f64,
    /// The total sum of fees associated with the loan repayment plan.
    pub total_fee: f64,

    /// The date this loan was disbursed.
    pub disbursement_date: NaiveDate,
    /// First date of a regular, scheduled repayment installment.
    pub first_installment_date: NaiveDate,
    /// The date this loan was completely payed back.
    pub end_date: NaiveDate,
    /// The number of total planned terms as of initial loan, transfer or refinance situation.
    pub planned_terms: i32,
    /// The number of total planned terms as of initial loan, transfer or refinance situation.
    pub completed_terms: i32,
}
