//! Run the interest calculator as a native GUI library.

use iced::{Sandbox, Settings};
use interest_calculator::gui::App;

fn main() -> iced::Result {
    #[cfg(target_arch = "wasm32")]
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    env_logger::init();

    App::run(Settings::default())
}
