# Interest Calculator

This project is a simple CLI utility that allows you to calculate the total loan costs
for the lifetime of the loan.

It allows to customize:
* Number of terms/years the loan shall be fully payed back.
* Number of terms per year: Either 1, 2, 3, 4, 6, or 12.
* Extra downpayments for an extended period of time.
 * This allows one to see the impact of changing the monthly downpayment for an extended or the entire period of the loan.

 As a bonus, a simple plot is generated to show each term payment and loan progress.

## Web

To build the web based application, you must have the `wasm-bindgen-cli` utility installed.

* `cargo build --target wasm32-unknown-unknown`
* `wasm-bindgen target/wasm32-unknown-unknown/debug/gui.wasm --out-dir web --web`
