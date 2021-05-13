//! This module encapsulates the API used to interact with the library.

use chrono::Month;

/// Each variant of a LoanEvent details the various events that can occur
/// for the lifetime of the loan.
#[derive(Debug)]
pub enum LoanEvent {
    /// The initial loan event - This is the point where the loan is constructed.
    Initial(LoanInitialization),
    /// An interest change is scheduled - at this point in time the interest will be
    /// altered to a different rate. All subsequent calculations will be redone.
    InterestChange(LoanInterestChange),
    /// We initiate a transfer between banks - this entails settling the current
    /// interest and establishing new calculations based the new set of values.
    ///
    /// A BankTransfer will null out any extra scheduled payments plan.
    BankTransfer(LoanTransfer),
    /// We inject new capital into the loan.
    Refinance(LoanRefinance),
    /// Extra scheduled installments for a period of time.
    Extra(LoanRecurringExtraInstallments),
    /// We schedule a installment freeze, only interest will be owed.
    RepaymentFreeze(LoanRepaymentFreeze),
}

impl LoanEvent {
    pub(crate) fn initial(&self) -> &LoanInitialization {
        match &*self {
            LoanEvent::Initial(d) => &d,
            _ => panic!("attempted to access LoanEvent::Initial that was not initial"),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum MonthlyDueDate {
    /// The 1st of the month.
    First,
    /// The 15th of the month.
    Mid,
    /// The end, be it 28, 29, 30 or 31.
    End,
    /// The date could be anywhere in the range 1-31.
    Date(u32),
}

impl MonthlyDueDate {
    pub fn to_u32(&self) -> u32 {
        match &*self {
            MonthlyDueDate::First => 1,
            MonthlyDueDate::Mid => 15,
            MonthlyDueDate::End => 31,
            MonthlyDueDate::Date(d) => *d,
        }
    }
}

/// Indicate how many installments per year a loan is is configured to have.
#[derive(Clone, Copy, Debug)]
pub enum TermsPerYear {
    One,
    Two,
    Three,
    Four,
    Six,
    Twelve,
}

impl TermsPerYear {
    pub fn to_u32(&self) -> u32 {
        match &*self {
            TermsPerYear::One => 1,
            TermsPerYear::Two => 2,
            TermsPerYear::Three => 3,
            TermsPerYear::Four => 4,
            TermsPerYear::Six => 6,
            TermsPerYear::Twelve => 12,
        }
    }
}

/// The initial state of a loan.
#[derive(Clone, Debug)]
pub struct LoanInitialization {
    /// The total loan sum.
    pub loan: f64,
    /// Nominal interest advertised by the loan provider.
    pub nominal_interest: f64,
    /// When issuing a loan, some banks will charge an administration fee for
    /// issuing the loan. This can be added to the loan sum and will be part of the
    /// calculations.
    pub administration_fee: f64,
    /// Each installment that includes a re-payment may include an additional fee.
    /// This fee is not included for interest installments, only re-payment installments.
    pub installment_fee: f64,

    /// The number of terms this loan should be downpayed over.
    pub terms: u32,
    /// The number of terms per year.
    pub terms_per_year: TermsPerYear,
    /// The time of month, if within a term month, a installment is due.
    pub due_within_month: MonthlyDueDate,

    /// This is first month after payout_date that an installment is due.
    /// The date within this month is calculated based on due_within_month.
    pub first_installment_month: Month,
}

/// An event to describe the transfer of a loan from one bank to another.
/// Terms and installment dates will be transferred from the last bank.
#[derive(Clone, Debug)]
pub struct LoanTransfer {
    /// When issuing a loan, some banks will charge an administration fee for
    /// issuing the loan. This can be added to the loan sum and will be part of the
    /// calculations.
    pub administration_fee: f64,
}

/// An event to describe an interest change on a loan.
#[derive(Clone, Debug)]
pub struct LoanInterestChange {
    /// This is the new interest on the loan, effective from date.
    pub nominal_interest: f64,
}

/// An event to describe an refinacing action.
#[derive(Clone, Debug)]
pub struct LoanRefinance {
    /// The total to increase the loan by.
    pub loan_increase: f64,
    /// Any administration fee related to the refinancing appropriation will be added
    /// to the total sum of the loan.
    pub administration_fee: f64,
}

/// A recurring interval selection within a year.
#[derive(Clone, Debug)]
pub enum RecurringInterval {
    /// Every week.
    Weekly,
    /// Every two weeks.
    Biweekly,
    /// Every month
    Monthly,
    /// Every two months, six times a year.
    Bimonthly,
    /// Every three months, four times a year.
    Quarerly,
    /// Every four months, three times a year.
    Triannually,
    /// Every six months, two times a year.
    Biannually,
    /// Every year.
    Anually,
}

/// An event to schedule a set of extra payments on the loan.
#[derive(Clone, Debug)]
pub struct LoanRecurringExtraInstallments {
    /// The amount per extra payments
    pub amount: f64,
    /// The number of extra payment events.
    /// This MAY be 1, where the recurring interval will have no effect.
    /// You may rather use LoanExtraInstallment event for a one-off extra installment.
    pub count: std::num::NonZeroU32,
    /// The interval to issue any additional extra payments outside one.
    pub recurring_interval: RecurringInterval,
}

/// An event to add a single extra installment.
#[derive(Clone, Debug)]
pub struct LoanExtraInstallment {
    /// The amount per extra payments
    pub amount: f64,
}

/// An event that freezes the current repayment installments.
/// Only interest installments must be made.
#[derive(Clone, Debug)]
pub struct LoanRepaymentFreeze {
    /// The number of repayment installment freezes.
    pub count: std::num::NonZeroU32,
}
