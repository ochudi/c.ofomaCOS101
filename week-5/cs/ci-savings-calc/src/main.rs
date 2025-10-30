use std::io;

fn main() {
    loop {
        // Read principal amount
        println!("Enter the principal amount (P):");
        let mut p_input = String::new();
        io::stdin()
            .read_line(&mut p_input)
            .expect("Failed to read principal amount.");
        let p: f64 = p_input
            .trim()
            .parse()
            .expect("Please enter a valid number for principal (P).");

        // Read interest rate
        println!("Enter the annual interest rate (R) in %:");
        let mut r_input = String::new();
        io::stdin()
            .read_line(&mut r_input)
            .expect("Failed to read interest rate.");
        let r: f64 = r_input
            .trim()
            .parse()
            .expect("Please enter a valid number for rate (R).");

        // Read time
        println!("Enter the time (T) in years:");
        let mut t_input = String::new();
        io::stdin()
            .read_line(&mut t_input)
            .expect("Failed to read time value.");
        let t: f64 = t_input
            .trim()
            .parse()
            .expect("Please enter a valid number for time (T).");

        // Compute total amount and compound interest
        let amount = p * (1.0 + r / 100.0).powf(t);
        let interest = amount - p;

        // Output results
        println!("-----------------------------");
        println!("Total amount after {} years: {:.2}", t, amount);
        println!("Compound interest earned: {:.2}", interest);
        println!("-----------------------------");

        // Ask if user wants to continue
        println!("Do you want to calculate for another customer? (y/n)");
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read user choice.");
        let choice = choice.trim().to_lowercase();
        if choice == "n" {
            println!("Goodbye!");
            break;
        }
    }
}
