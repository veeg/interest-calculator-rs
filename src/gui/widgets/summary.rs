//! A summary widget for the overall state of the entire loan and downpayment.

use crate::gui::Message;
use crate::reports::TotalResult;

use iced::{Color, Column, Element, HorizontalAlignment, Length, Row, Space, Text};

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
#[derive(Default)]
pub struct Summary {
    disbursement_date: String,
    finalized_date: String,
    // Formatted as: completed/planned
    terms: String,
    total_cost: String,
    total_loan: String,
    total_interest: String,
    total_fee: String,

    error: String,
}

impl Summary {
    pub fn update(&mut self, total: Result<TotalResult, String>) {
        match total {
            Ok(t) => {
                self.disbursement_date = t.disbursement_date.to_string();
                self.finalized_date = t.disbursement_date.to_string();
                self.terms = format!("{}/{}", t.completed_terms, t.planned_terms);
                self.total_cost = format!("{:.2}", t.total_cost);
                self.total_loan = format!("{:.2}", t.total_loan);
                self.total_interest = format!("{:.2}", t.total_interest);
                self.total_fee = format!("{:.2}", t.total_fee);

                self.error.clear();
            }
            Err(e) => {
                self.disbursement_date = String::new();
                self.finalized_date = String::new();
                self.terms = String::new();
                self.total_cost = String::new();
                self.total_loan = String::new();
                self.total_interest = String::new();
                self.total_fee = String::new();

                self.error = e;
            }
        }
    }

    pub fn view(&mut self) -> Element<Message> {
        Column::new()
            // Render the top title
            .push(Text::new("Summary").size(40))
            .push(Text::new(&self.error).color(Color::from_rgb(0.76, 0.094, 0.027)))
            // Render a single row within the column
            .push(
                Row::new()
                    .push(Text::new("Disbursement date:"))
                    .push(Space::with_width(Length::Fill))
                    .push(
                        Text::new(&self.disbursement_date)
                            .horizontal_alignment(HorizontalAlignment::Right),
                    ),
            )
            .push(
                Row::new()
                    .push(Text::new("Finalized date:"))
                    .push(Space::with_width(Length::Fill))
                    .push(
                        Text::new(&self.finalized_date)
                            .horizontal_alignment(HorizontalAlignment::Right),
                    ),
            )
            .push(
                Row::new()
                    .push(Text::new("Terms (completed/planned):"))
                    .push(Space::with_width(Length::Fill))
                    .push(Text::new(&self.terms).horizontal_alignment(HorizontalAlignment::Right)),
            )
            .push(
                Row::new()
                    .push(Text::new("Principal loan:"))
                    .push(Space::with_width(Length::Fill))
                    .push(
                        Text::new(&self.total_loan)
                            .horizontal_alignment(HorizontalAlignment::Right),
                    ),
            )
            .push(
                Row::new()
                    .push(Text::new("Cost:"))
                    .push(Space::with_width(Length::Fill))
                    .push(
                        Text::new(&self.total_cost)
                            .horizontal_alignment(HorizontalAlignment::Right),
                    ),
            )
            .push(
                Row::new()
                    .push(Text::new("Interest:"))
                    .push(Space::with_width(Length::Fill))
                    .push(
                        Text::new(&self.total_interest)
                            .horizontal_alignment(HorizontalAlignment::Right),
                    ),
            )
            .push(
                Row::new()
                    .push(Text::new("Fee:"))
                    .push(Space::with_width(Length::Fill))
                    .push(
                        Text::new(&self.total_fee).horizontal_alignment(HorizontalAlignment::Right),
                    ),
            )
            // Finalize
            .into()
    }
}
