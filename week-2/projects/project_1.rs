fn main() {
    // Given values
    let principal: f64 = 520_000_000.0; // Principal amount in Naira
    let rate: f64 = 10.0; // Rate of interest per annum (10%)
    let time: f64 = 5.0; // Time period in years

    // Calculate compound amount using formula: A = P[1 + (R/100)]^n
    let amount = principal * (1.0 + (rate / 100.0)).powf(time);

    // Calculate compound interest: CI = A - P
    let compound_interest = amount - principal;

    // Display results
    println!("Principal Amount: N{:.2}", principal);
    println!("Rate of Interest: {}% per annum", rate);
    println!("Time Period: {} years", time);
    println!("\nCompound Amount (A): N{:.2}", amount);
    println!("Compound Interest (CI): N{:.2}", compound_interest);
}