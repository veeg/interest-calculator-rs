//! A command line utility to calculate a loans lifespan and the costs associated with that.

use interest_calculator::plot::create_plot;
use interest_calculator::*;

use chrono::offset::Utc;
use chrono::Month;
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

pub fn parse() -> Result<LoanInitialization, String> {
    let opt = Opt::from_args();

    // Sanify how many terms_per_year we can do
    // I think its safe to assume that only a few combinations make sense
    const ALLOWED_TERMS_PER_YEAR: [i32; 6] = [1, 2, 3, 4, 6, 12];
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

    let terms_per_year = match opt.terms_per_year {
        1 => TermsPerYear::One,
        2 => TermsPerYear::Two,
        3 => TermsPerYear::Three,
        4 => TermsPerYear::Four,
        6 => TermsPerYear::Six,
        12 => TermsPerYear::Twelve,
        _ => panic!("cannot be reached"),
    };

    // Day of month for term due
    // TODO: Make this configurable
    let term_due_day = 20;

    let initial = LoanInitialization {
        loan: opt.loan as f64,
        nominal_interest: opt.interest,
        administration_fee: 0.0,
        installment_fee: opt.fee as f64,

        terms: terms,
        terms_per_year,
        due_within_month: MonthlyDueDate::Date(term_due_day),
        // TODO: Calculate this correctly.
        first_installment_month: Month::January,
    };

    Ok(initial)
}

fn main() {
    #[cfg(wasm)]
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    let initial = match parse() {
        Ok(initial) => initial,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    // Get date for start of loan
    // TODO: Make load payout date configurable
    let loan_start_date = Utc::now().naive_utc().date();

    let calculator = InteractiveCalculator::new(loan_start_date, initial);
    let (total, monthly, _daily) = calculator.compute();

    println!("{:#?}", total);
    create_plot(monthly, total).unwrap();
}
