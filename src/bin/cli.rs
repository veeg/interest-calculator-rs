//! A command line utility to calculate a loands lifespand and the costs associated with that.

use interest_calculator::process::*;

fn main() {
    #[cfg(wasm)]
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    process();
}
