use std::io;

fn main() {
    loop {
        println!("\n===========================================");
        println!("  PAN-ATLANTIC UNIVERSITY COMPUTER STORE");
        println!("---------------------------------------------");
        println!("Code | Item      | Price (₦)");
        println!("  L  | Laptop    | 550,000");
        println!("  M  | Monitor   | 120,000");
        println!("  K  | Keyboard  | 15,000");
        println!("  H  | Headset   | 25,000");
        println!("------------------------------------");

        // Read item code
        println!("Enter item code (L, M, K, H):");
        let mut code = String::new();
        io::stdin()
            .read_line(&mut code)
            .expect("Failed to read item code.");
        let code = code.trim().to_uppercase();

        // Match the item code to its price
        let price = match code.as_str() {
            "L" => 550_000.0,
            "M" => 120_000.0,
            "K" => 15_000.0,
            "H" => 25_000.0,
            _ => {
                println!("Invalid item code. Please try again.\n");
                continue;
            }
        };

        // Read quantity
        println!("Enter quantity:");
        let mut qty_input = String::new();
        io::stdin()
            .read_line(&mut qty_input)
            .expect("Failed to read quantity.");
        let quantity: f64 = qty_input
            .trim()
            .parse()
            .expect("Please enter a valid number for quantity.");

        // Calculate total
        let total = price * quantity;

        // Apply discount if applicable
        let final_amount = if total > 500_000.0 {
            let discount = total * 0.07;
            println!("A 7% discount of ₦{:.2} has been applied.", discount);
            total - discount
        } else {
            total
        };

        // Output results
        println!("------------------------------------");
        println!("Item code: {}", code);
        println!("Quantity: {}", quantity);
        println!("Total cost: ₦{:.2}", total);
        println!("Final amount payable: ₦{:.2}", final_amount);
        println!("------------------------------------");

        // Ask if user wants to continue
        println!("Do you want to enter another order? (y/n)");
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read choice.");
        if choice.trim().to_lowercase() == "n" {
            println!("Thank you for using the system. Goodbye!");
            break;
        }
    }
}
