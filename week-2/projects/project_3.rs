
// A = P * [1 - (R/100)]^n
// P = 510,000; R = 5% per annum; n = 3 years

fn main() {
    let principal: f64 = 510_000.0;
    let rate_percent: f64 = 5.0;
    let years: i32 = 3;

    let factor = 1.0 - rate_percent / 100.0;
    let amount = principal * factor.powi(years);

    println!(
        "Value of the TV after {} years at {}% depreciation per annum is N{:.2}",
        years, rate_percent, amount
    );
}