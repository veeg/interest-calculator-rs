//! Implement a user-interface application for the interest calculator.

mod widgets;

use self::widgets::summary::Summary;

use chrono::NaiveDate;
use iced::{Sandbox, Element, Text, Container, Column, Length};

/// The GUI application.
pub struct App {
    summary: Summary,
}

#[derive(Debug, Clone)]
pub enum Message {
}

impl Sandbox for App {
    type Message = Message;

    fn new() -> Self {
        Self {
            summary: Summary {
                loan_start_date: Some(NaiveDate::from_ymd(1970, 1, 1)),
                loan_finish_date: None,
                initial_loan: 0,
                total_sum: 0.0,
                loan_cost: 0.0,
            }
        }
    }

    fn title(&self) -> String {
        "Loan Interest Calculator".to_string()
    }

    fn update(&mut self, _event: Message) {
    }

    fn view(&mut self) -> Element<Message> {

        // Left hand column that holds:
        // * Overall total loan statistics
        // * The events modification table
        let content: Element<_> = Column::new()
            .max_width(540)
            .spacing(20)
            .padding(20)
            .push(self.summary.view())
            .push(Text::new("Here should the event modification widget go"))
            .into();

        // Generate the top level view
        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_y()
            .into()
    }
}
