//! Installment Loan Calculator
//!
//! This library implements a series of calculations on the progess
//! of an installment loan, where multiple types of events may influence
//! the loan over its lifetime.

mod calculator;
mod events;
pub mod gui;
pub mod plot;
pub mod reports;

pub use calculator::{CompoundingStrategy, InteractiveCalculator};
pub use events::*;
