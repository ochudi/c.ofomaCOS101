use std::io::{self, Write};

fn read_score(prompt: &str) -> Option<f64> {
    print!("{}", prompt);
    io::stdout().flush().ok()?;
    let mut s = String::new();
    io::stdin().read_line(&mut s).ok()?;
    let val: f64 = match s.trim().parse() {
        Ok(v) => v,
        Err(_) => {
            eprintln!("Invalid numeric input for score.");
            return None;
        }
    };
    if val < 0.0 || val > 100.0 {
        eprintln!("Invalid score: {}. Scores must be between 0 and 100.", val);
        return None;
    }
    Some(val)
}

fn main() {
    // Read name
    print!("Enter student name: ");
    io::stdout().flush().ok();
    let mut name = String::new();
    if io::stdin().read_line(&mut name).is_err() {
        eprintln!("Failed to read name.");
        return;
    }
    let name = name.trim();
    if name.is_empty() {
        eprintln!("Name cannot be empty.");
        return;
    }

    // Read three scores
    let s1 = match read_score("Enter score 1: ") { Some(v) => v, None => return };
    let s2 = match read_score("Enter score 2: ") { Some(v) => v, None => return };
    let s3 = match read_score("Enter score 3: ") { Some(v) => v, None => return };

    // Calculate average
    let avg = (s1 + s2 + s3) / 3.0;
    let avg_str = format!("{:.2}", avg);

    // Determine grade using if..else if..else
    let grade = if avg >= 70.0 && avg <= 100.0 {
        "A"
    } else if avg >= 60.0 {
        "B"
    } else if avg >= 50.0 {
        "C"
    } else if avg >= 45.0 {
        "D"
    } else {
        "F"
    };

    // Output
    println!("Student: {}", name);
    println!("Average: {}", avg_str);
    println!("Grade: {}", grade);
}
