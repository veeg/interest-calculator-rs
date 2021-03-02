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

    let summary = calculator.compute();
    assert!(summary.is_ok());
    let summary = summary.unwrap();

    assert_eq!(summary.total_loan, 1000.0);
    assert!(summary.total_interest > 1.0);
    assert_eq!(summary.disbursement_date, loan_start_date);
    assert_eq!(summary.end_date, NaiveDate::from_ymd(2022, 1, 1));
}

#[test]
fn outstanding_loan_is_negative() {
    let initial = LoanInitialization {
        loan: -560.5,
        nominal_interest: 1.0,
        administration_fee: 0.0,
        installment_fee: 0.0,
        terms: 12,
        terms_per_year: TermsPerYear::Twelve,
        due_within_month: MonthlyDueDate::First,
        first_installment_month: Month::February,
    };

    let loan_start_date = NaiveDate::from_ymd(2021, 1, 10);
    let calculator = InteractiveCalculator::new(loan_start_date, initial);

    let res = calculator.compute();
    assert!(res.is_err());
    assert_eq!(
        "expecting non-zero positive value for outstanding loan in initial loan event",
        &res.unwrap_err()
    );
}

#[test]
fn outstanding_loan_is_zero() {
    let initial = LoanInitialization {
        loan: 0.0,
        nominal_interest: 1.0,
        administration_fee: 0.0,
        installment_fee: 0.0,
        terms: 12,
        terms_per_year: TermsPerYear::Twelve,
        due_within_month: MonthlyDueDate::First,
        first_installment_month: Month::February,
    };

    let loan_start_date = NaiveDate::from_ymd(2021, 1, 10);
    let calculator = InteractiveCalculator::new(loan_start_date, initial);

    let res = calculator.compute();
    assert!(res.is_err());
    assert_eq!(
        "expecting non-zero positive value for outstanding loan in initial loan event",
        &res.unwrap_err()
    );
}

#[test]
fn interest_is_zero() {
    let initial = LoanInitialization {
        loan: 1000.0,
        nominal_interest: 0.0,
        administration_fee: 0.0,
        installment_fee: 0.0,
        terms: 12,
        terms_per_year: TermsPerYear::Twelve,
        due_within_month: MonthlyDueDate::First,
        first_installment_month: Month::February,
    };

    let loan_start_date = NaiveDate::from_ymd(2021, 1, 10);
    let calculator = InteractiveCalculator::new(loan_start_date, initial);

    let res = calculator.compute();
    assert!(res.is_err());
    assert_eq!(
        "expecting non-zero positive value for nominal interest in initial loan event",
        &res.unwrap_err()
    );
}

#[test]
fn interest_is_negative() {
    let initial = LoanInitialization {
        loan: 1000.0,
        nominal_interest: -1.0,
        administration_fee: 0.0,
        installment_fee: 0.0,
        terms: 12,
        terms_per_year: TermsPerYear::Twelve,
        due_within_month: MonthlyDueDate::First,
        first_installment_month: Month::February,
    };

    let loan_start_date = NaiveDate::from_ymd(2021, 1, 10);
    let calculator = InteractiveCalculator::new(loan_start_date, initial);

    let res = calculator.compute();
    assert!(res.is_err());
    assert_eq!(
        "expecting non-zero positive value for nominal interest in initial loan event",
        &res.unwrap_err()
    );
}
