use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Enter first edge of triangle:");
    io::stdin()
        .read_line(&mut input1)
        .expect("Not a valid string");
    let a: f32 = input1.trim().parse().expect("Not a valid number");

    println!("Enter second edge of triangle:");
    io::stdin()
        .read_line(&mut input2)
        .expect("Not a valid string");
    let b: f32 = input2.trim().parse().expect("Not a valid number");

    println!("Enter third edge of triangle:");
    io::stdin()
        .read_line(&mut input3)
        .expect("Not a valid string");
    let c: f32 = input3.trim().parse().expect("Not a valid number");

    // Validate inputs to avoid NaN from invalid triangles or non-positive sides
    if a <= 0.0 || b <= 0.0 || c <= 0.0 {
        println!("Edges must be positive numbers.");
        return;
    }
    if a + b <= c || a + c <= b || b + c <= a {
        println!("Invalid triangle: the sum of any two sides must be greater than the third.");
        return;
    }

    let s: f32 = (a + b + c) / 2.0;
    let mut area_sq: f32 = s * (s - a) * (s - b) * (s - c);
    // Clamp tiny negative due to floating-point rounding
    if area_sq < 0.0 && area_sq > -1e-6 {
        area_sq = 0.0;
    }
    let area: f32 = area_sq.sqrt();
    
    println!("Area of a triangle: {}", area);
}
