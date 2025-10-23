fn main() {
    let mut count = 0;

    for num in 1..=20 {
        if num > 10 {
            println!("{:?}", num);
            continue;
        }
        count += 1;
    }
    println!("Count of numbers greater than 10 (between 1 and 20) is: {}", count); // Output: 10
}
