use std::io;

fn main() {
	println!("Enter a number");
	let mut input1 = String::new();
	io::stdin().read_line(&mut input1).expect("Failed to read input");
	let mut num: i32 = input1.trim().parse().expect("Not a valid integer");

	while num < 10 {
		println!("Inside loop number value is {}", num);
		num += 1;
	}

	println!("\nOutside loop number value is {}", num);
}
