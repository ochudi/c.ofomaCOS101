use std::io;

fn main() {
    // Ask for the student's name
    println!("Enter student name:");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read name");
    let name = name.trim(); // remove extra spaces or newline

    if name.is_empty() {
        println!("Name cannot be empty.");
        return;
    }

    // Function to read a single score
    fn get_score(number: u8) -> f64 {
        loop {
            println!("Enter score {} (0–100):", number);
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read input");

            // Try to change the input into a number
            match input.trim().parse::<f64>() {
                Ok(value) if value >= 0.0 && value <= 100.0 => return value,
                _ => println!("Please enter a valid number between 0 and 100."),
            }
        }
    }

    // Get three scores
    let score1 = get_score(1);
    let score2 = get_score(2);
    let score3 = get_score(3);

    // Calculate average
    let average = (score1 + score2 + score3) / 3.0;

    // Decide the grade
    let grade = if average >= 70.0 {
        "A"
    } else if average >= 60.0 {
        "B"
    } else if average >= 50.0 {
        "C"
    } else if average >= 45.0 {
        "D"
    } else {
        "F"
    };

    // Show result
    println!("\nStudent: {}", name);
    println!("Average: {:.2}", average);
    println!("Grade: {}", grade);
}