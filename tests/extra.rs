use interest_calculator::{
    InteractiveCalculator, LoanExtraInstallment, LoanInitialization, MonthlyDueDate, TermsPerYear,
};

use chrono::{Month, NaiveDate};

#[test]
fn interactive_calculator_extra_installment() {
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
    let mut calculator = InteractiveCalculator::new(loan_start_date.clone(), initial);

    let extra_event = LoanExtraInstallment { amount: 100.0 };
    let extra_date = NaiveDate::from_ymd(2021, 2, 20);
    calculator
        .add_event_extra_single(extra_date, extra_event)
        .unwrap();

    let summary = calculator.compute();
    assert!(summary.is_ok());
    let summary = summary.unwrap();

    // Loan is payed back earlier due to extra installment
    assert_eq!(summary.total_extra_installment, 100.0);
    assert_eq!(summary.end_date, NaiveDate::from_ymd(2021, 12, 1));
}
