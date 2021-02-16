//! Run the interest calculator as a native GUI library.

use iced::{Settings, Sandbox};
use interest_calculator::gui::App;

fn main() -> iced::Result {
    env_logger::init();

    App::run(Settings::default())
}
