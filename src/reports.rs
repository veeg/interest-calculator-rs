//! Models related to the reports of calculations

use chrono::NaiveDate;

#[derive(Debug)]
pub struct DailyResult {
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
