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

#[derive(Debug)]
pub struct TotalResult {
    pub total_cost: f64,
    pub loan: i64,
    pub interest: f64,
    pub fee: i32,
    pub completed_terms: i32,
    pub planned_terms: i32,
}
