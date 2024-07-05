use std::io;

fn main() {
    println!("Enter a number: ");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let num = input.trim().parse::<u64>().expect("Please enter a number");

    println!("{}", factorial(num))
}

fn factorial(num: u64) -> u64 {
    if num <= 1 {
        return 1;
    }

    num * factorial(num - 1)
}
