//! Implement a user-interface application for the interest calculator.

mod widgets;

use self::widgets::{event_initialization::EventInitialization, summary::Summary};
use crate::{InteractiveCalculator, LoanInitialization, MonthlyDueDate, TermsPerYear};

use iced::{Column, Container, Element, Length, Sandbox};

/// The GUI application.
pub struct App {
    summary: Summary,
    calculator: InteractiveCalculator,
    event_initialization: EventInitialization,
}

#[derive(Debug, Clone)]
pub enum Message {
    EventInitialization(widgets::event_initialization::WidgetMessage),
}

impl Sandbox for App {
    type Message = Message;

    fn new() -> Self {
        // We default to today
        let loan_start_date = chrono::Utc::today().naive_utc();
        let first_installment_month = crate::calculator::future_month(&loan_start_date, 1);

        let initial = LoanInitialization {
            loan: 1000000.0,
            nominal_interest: 1.25,
            administration_fee: 0.0,
            installment_fee: 45.0,

            terms: 12,
            terms_per_year: TermsPerYear::Twelve,
            due_within_month: MonthlyDueDate::First,
            first_installment_month,
        };

        let event_initialization = EventInitialization::new(&initial);
        let mut s = Self {
            summary: Summary::default(),
            calculator: InteractiveCalculator::new(loan_start_date.clone(), initial),
            event_initialization,
        };

        // Cheeky initialization
        s.summary.update(s.calculator.compute());

        s
    }

    fn title(&self) -> String {
        "Installment Loan Calculator".to_string()
    }

    fn update(&mut self, event: Message) {
        match event {
            Message::EventInitialization(m) => {
                self.event_initialization.update(&mut self.calculator, 0, m)
            }
        }

        self.summary.update(self.calculator.compute());
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
            .push(
                self.event_initialization
                    .view()
                    .map(Message::EventInitialization),
            )
            .into();

        // Generate the top level view
        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_y()
            .into()
    }
}
