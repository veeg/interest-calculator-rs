//! An interest calculator with with daily status report.
//!
//! This interest calculator supports the following features:
//! * Add an arbitrary number of extra downpayments
//! * Get a daily/monthly/yearly state report over the coarse of the loan.

mod cli;
mod plot;
mod process;

pub use plot::*;
pub use process::*;

fn main() {
    #[cfg(wasm)]
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    process();
}
