//! A summary widget for the overall state of the entire loan and downpayment.

use crate::gui::Message;

use chrono::NaiveDate;
use iced::{Length, Row, Space, Column, Text, HorizontalAlignment, Element};

/// This summary widget lays out and renders the table with summary information.
/// Only rendering of text shall be provided. Manipulation of entry is not done through
/// this rendering widget.
///
/// The information included is:
/// * Loan start date
/// * Loan finish date
/// * Initial loan sum
/// * Total sum
/// * Loan cost (The cost of the loan over the period from start to finish)
///
/// Interest and fees are not included here, since they may/will be altered over the course of the
/// loan, and should be retrieved from the timeline widget.
pub struct Summary {
    pub loan_start_date: Option<NaiveDate>,
    pub loan_finish_date: Option<NaiveDate>,
    pub initial_loan: i64,
    pub total_sum: f64,
    pub loan_cost: f64,
}

impl Summary {
    pub fn view(&mut self) -> Element<Message> {
        Column::new()
            // Render the top title
            .push(Text::new("Summary").size(40))
            // Render a single row within the column
            .push(Row::new().push(Text::new("Start date")).push(Space::with_width(Length::Fill)).push(Text::new(self.loan_start_date.clone().map(|d| d.to_string()).unwrap_or("".to_string())).horizontal_alignment(HorizontalAlignment::Right)))
            // Finalize
            .into()
    }
}
