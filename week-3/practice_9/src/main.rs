fn main() {
    // By default, variables are immutable: you can't reassign them.
    let course = "COS-101";
    println!("course is {}", course);
    // course = "COS-102"; // ERROR: cannot assign to immutable variable

    // Make a variable mutable with `mut` to allow reassignment.
    let mut fees: i32 = 25_000;
    println!("fees is {}", fees);
    fees = 35_000;
    println!("fees changed to {}", fees);

    // Shadowing: reuse the same name to create a new (still immutable) binding.
    let level = 100;
    let level = level + 1; // new binding, old one is not mutated
    println!("level after shadowing is {}", level);

    // Constants are always immutable and require a type.
    const REG_FEE: i32 = 10_000;
    println!("registration fee is {}", REG_FEE);
}
