use chrono::prelude::*;
use chrono::offset::Utc;
use structopt::StructOpt;

use num_traits::cast::FromPrimitive;

#[derive(Debug, StructOpt)]
#[structopt(name = "interest-calculator")]
struct Opt {
    /// Total sum of the loan.
    #[structopt(default_value = "4350000")]
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
    interest: f32,
    /// Incurring cost for each term payment.
    #[structopt(short, long, default_value = "45")]
    fee: i32,
}

fn main() {
    let opt = Opt::from_args();

    // Sanify how many terms_per_year we can do
    // I think its safe to assume that only a few combinations make sense
    const ALLOWED_TERMS_PER_YEAR: [i32; 5] = [1, 2, 4, 6, 12];
    if !ALLOWED_TERMS_PER_YEAR.contains(&opt.terms_per_year) {
        eprintln!("error: The argument '--terms-per-year <num>' must be one of {:?}", ALLOWED_TERMS_PER_YEAR);
        return;
    }

    let terms = match (opt.terms, opt.years) {
        (Some(t), None) => t,
        (None, Some(y)) => y * opt.terms_per_year,
        (None, None) => 30 * opt.terms_per_year,
        (Some(_), Some(_)) => unreachable!(),
    };

    println!("{:?}", opt);
    println!("Number of terms: {}", terms);

    // Get the next month and the year that belongs to
    let current_time = Utc::now();
    let month = Month::from_u32(current_time.month()).unwrap().succ();
    let year = if month.number_from_month() == 1 {
        current_time.year() + 1
    } else {
        current_time.year()
    };

    println!("Starting calculation from {} {}", month.name(), year);
}
