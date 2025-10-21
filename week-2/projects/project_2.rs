fn main() {
    // Sales data: (Item, Quantity, Amount)
    let items = ["Toshiba", "Mac", "HP", "Dell", "Acer"];
    let quantities = [2, 1, 3, 3, 1];
    let amounts = [450_000.00, 1_500_000.00, 750_000.00, 2_850_000.00, 250_000.00];

    // Calculate sum of all sales
    let mut sum: f64 = 0.0;
    for &amount in &amounts {
        sum += amount;
    }

    // Calculate average
    let count = amounts.len() as f64;
    let average = sum / count;

    // Display results
    println!("Chief Donatus and Sons Ltd - Sales Report");
    println!("==========================================\n");
    
    println!("{:<5} {:<15} {:<10} {:<15}", "S/N", "Item", "Qty", "Amount");
    println!("{:-<50}", "");
    
    for index in 0..items.len() {
        println!("{:<5} {:<15} {:<10} {:<15.2}", 
                 index + 1, items[index], quantities[index], amounts[index]);
    }
    
    println!("{:-<50}", "");
    println!("\nSum of Sales: ₦{:.2}", sum);
    println!("Average Sales: ₦{:.2}", average);
}
