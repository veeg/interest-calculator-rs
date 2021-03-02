//! Represent the widget to modify the initialization state

use crate::{events::LoanInitialization, InteractiveCalculator, LoanEvent};

use chrono::NaiveDate;
use iced::{Column, Element, Length, Row, Space, Text, TextInput};
use std::str::FromStr;

#[derive(Default)]
pub struct EventInitialization {
    administration_fee_state: iced::text_input::State,
    administration_fee_data: String,

    disbursement_date_state: iced::text_input::State,
    disbursement_date_data: String,

    installment_fee_state: iced::text_input::State,
    installment_fee_data: String,

    loan_state: iced::text_input::State,
    loan_data: String,

    interest_state: iced::text_input::State,
    interest_data: String,
}

#[derive(Clone, Debug)]
pub enum WidgetMessage {
    DisbursementDateChanged(String),
    LoanChanged(String),
    InterestChanged(String),
    AdministrationFeeChanged(String),
    InstallmentFeeChanged(String),
}

impl EventInitialization {
    pub fn new(event: &LoanInitialization) -> EventInitialization {
        Self {
            loan_data: event.loan.to_string(),
            interest_data: event.nominal_interest.to_string(),
            administration_fee_data: event.administration_fee.to_string(),
            installment_fee_data: event.installment_fee.to_string(),
            ..Default::default()
        }
    }

    pub fn update(
        &mut self,
        calc: &mut InteractiveCalculator,
        event_index: usize,
        message: WidgetMessage,
    ) {
        match message {
            WidgetMessage::DisbursementDateChanged(data) => {
                self.disbursement_date_data = data;
                // TODO: Validate the error and report to the user
                if let Ok(date) = NaiveDate::from_str(&self.disbursement_date_data) {
                    calc.change_event_date(event_index, date);
                }
            }
            WidgetMessage::LoanChanged(data) => {
                self.loan_data = data;
                let (_, event) = calc.event_index(event_index);
                match event {
                    LoanEvent::Initial(init) => {
                        if let Ok(loan) = f64::from_str(&self.loan_data) {
                            init.loan = loan;
                        }
                    }
                    _ => panic!("resolved to unexpected event"),
                }
            }
            WidgetMessage::InterestChanged(data) => {
                self.interest_data = data;
                let (_, event) = calc.event_index(event_index);
                match event {
                    LoanEvent::Initial(init) => {
                        // Expecting interest to lie within range 0.0 to 100.0
                        // Anything outside 100.0 interest would be ludicrous.
                        if let Ok(interest) = f64::from_str(&self.interest_data) {
                            init.nominal_interest = interest;
                        }
                    }
                    _ => panic!("resolved to unexpected event"),
                }
            }
            WidgetMessage::AdministrationFeeChanged(data) => {
                self.administration_fee_data = data;
                let (_, event) = calc.event_index(event_index);
                match event {
                    LoanEvent::Initial(init) => {
                        // Expecting interest to lie within range 0.0 to 100.0
                        // Anything outside 100.0 interest would be ludicrous.
                        if let Ok(fee) = f64::from_str(&self.administration_fee_data) {
                            init.administration_fee = fee;
                        }
                    }
                    _ => panic!("resolved to unexpected event"),
                }
            }
            WidgetMessage::InstallmentFeeChanged(data) => {
                self.installment_fee_data = data;
                let (_, event) = calc.event_index(event_index);
                match event {
                    LoanEvent::Initial(init) => {
                        // Expecting interest to lie within range 0.0 to 100.0
                        // Anything outside 100.0 interest would be ludicrous.
                        if let Ok(fee) = f64::from_str(&self.installment_fee_data) {
                            init.installment_fee = fee;
                        }
                    }
                    _ => panic!("resolved to unexpected event"),
                }
            }
        }
    }

    pub fn view(&mut self) -> Element<WidgetMessage> {
        Column::new()
            // Render the top title
            .push(Text::new("Loan start").size(30))
            // Render a single row within the column
            .push(
                Row::new()
                    .push(Text::new("Disbursement date:"))
                    .push(Space::with_width(Length::Fill))
                    .push(TextInput::new(
                        &mut self.disbursement_date_state,
                        "yyyy-mm-dd",
                        &self.disbursement_date_data,
                        WidgetMessage::DisbursementDateChanged,
                    )),
            )
            .push(
                Row::new()
                    .push(Text::new("Loan:"))
                    .push(Space::with_width(Length::Fill))
                    .push(TextInput::new(
                        &mut self.loan_state,
                        "",
                        &self.loan_data,
                        WidgetMessage::LoanChanged,
                    )),
            )
            .push(
                Row::new()
                    .push(Text::new("Interest:"))
                    .push(Space::with_width(Length::Fill))
                    .push(TextInput::new(
                        &mut self.interest_state,
                        "",
                        &self.interest_data,
                        WidgetMessage::InterestChanged,
                    )),
            )
            .push(
                Row::new()
                    .push(Text::new("Administration fee:"))
                    .push(Space::with_width(Length::Fill))
                    .push(TextInput::new(
                        &mut self.administration_fee_state,
                        "",
                        &self.administration_fee_data,
                        WidgetMessage::AdministrationFeeChanged,
                    )),
            )
            .push(
                Row::new()
                    .push(Text::new("Installment fee:"))
                    .push(Space::with_width(Length::Fill))
                    .push(TextInput::new(
                        &mut self.installment_fee_state,
                        "",
                        &self.installment_fee_data,
                        WidgetMessage::InstallmentFeeChanged,
                    )),
            )
            // Finalize
            .into()
    }
}
