use interest_calculator::{
    InteractiveCalculator, LoanInitialization, MonthlyDueDate, TermsPerYear,
};

use chrono::{Month, NaiveDate};

#[test]
fn interactive_calculator_initial_event() {
    let initial = LoanInitialization {
        loan: 1000.0,
        nominal_interest: 1.0,
        administration_fee: 0.0,
        installment_fee: 0.0,

        terms: 12,
        terms_per_year: TermsPerYear::Twelve,
        due_within_month: MonthlyDueDate::First,
        first_installment_month: Month::February,
    };

    let loan_start_date = NaiveDate::from_ymd(2021, 1, 10);
    let calculator = InteractiveCalculator::new(loan_start_date.clone(), initial);

    let (summary, _, _) = calculator.compute();

    println!("{:#?}", summary);

    assert_eq!(summary.total_loan, 1000.0);
    assert!(summary.total_interest > 10.0);
    assert_eq!(summary.start_date, loan_start_date);
    assert_eq!(summary.end_date, NaiveDate::from_ymd(2022, 1, 1));
}
