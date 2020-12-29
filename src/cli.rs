//! Handle command line options and turn it into internal state variable struct

use chrono::offset::Utc;
use chrono::prelude::*;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "interest-calculator")]
struct Opt {
    /// Total sum of the loan.
    #[structopt(long, default_value = "4350000")]
    loan: i64,
    /// Number of terms to pay back the entire loan.
    /// Incompatible with the `years` option.
    #[structopt(short, long)]
    terms: Option<i32>,
    /// Number of years to pay back the entire loan.
    /// Spread out across `terms_per_year` to determine the number of terms to cover the
    /// entire loan.
    /// Incompatible with the `terms` option.
    #[structopt(short, long, conflicts_with("terms"))]
    years: Option<i32>,
    /// Number of terms per year.
    #[structopt(long, default_value = "12")]
    terms_per_year: i32,

    /// Interest over an entire year.
    #[structopt(short, long, default_value = "1.25")]
    interest: f64,
    /// Incurring cost for each term payment.
    #[structopt(short, long, default_value = "45")]
    fee: i32,

    /// The number of terms to perform extra downpayment on
    #[structopt(long, default_value = "0")]
    extra_terms: u32,
    /// The day of the month of a term to perform extra payment on.
    #[structopt(long, default_value = "25")]
    extra_payment_day: u32,
    /// The amount to inject as extra downpayment in a term.
    #[structopt(long, default_value = "6000")]
    extra_amount: i32,
}

#[derive(Debug)]
pub struct State {
    pub loan: i64,
    pub nominal_interest: f64,
    pub effective_interest: f64,
    pub fee: i32,

    pub loan_start_date: NaiveDate,
    pub term_due_day: u32,

    pub terms: i32,
    pub terms_per_year: i32,

    pub extra_terms: u32,
    pub extra_payment_day: u32,
    pub extra_amount: i32,
}

pub fn parse() -> Result<State, String> {
    let opt = Opt::from_args();

    // Sanify how many terms_per_year we can do
    // I think its safe to assume that only a few combinations make sense
    const ALLOWED_TERMS_PER_YEAR: [i32; 5] = [1, 2, 4, 6, 12];
    if !ALLOWED_TERMS_PER_YEAR.contains(&opt.terms_per_year) {
        return Err(format!(
            "error: The argument '--terms-per-year <num>' must be one of {:?}",
            ALLOWED_TERMS_PER_YEAR
        ));
    }

    let terms = match (opt.terms, opt.years) {
        (Some(t), None) => t,
        (None, Some(y)) => y * opt.terms_per_year,
        (None, None) => 30 * opt.terms_per_year,
        (Some(_), Some(_)) => unreachable!(),
    };

    // Get date for start of loan
    // TODO: Make load payout date configurable
    let loan_start_date = Utc::now().naive_utc().date();

    // Day of month for term due
    // TODO: Make this configurable
    let term_due_day = 20;

    // Calculate effective interest rate
    let effective_interest = 1.0 + ((opt.interest / 100.0) / opt.terms_per_year as f64);
    let effective_interest = f64::powi(effective_interest, opt.terms_per_year);
    let effective_interest = effective_interest - 1.0;
    let effective_interest = effective_interest * 100.0;

    println!("effective interest: {}", effective_interest);

    Ok(State {
        loan: opt.loan,
        nominal_interest: opt.interest,
        effective_interest,
        fee: opt.fee,

        loan_start_date,
        term_due_day,

        terms,
        terms_per_year: opt.terms_per_year,

        extra_terms: opt.extra_terms,
        extra_payment_day: opt.extra_payment_day,
        extra_amount: opt.extra_amount,
    })
}
