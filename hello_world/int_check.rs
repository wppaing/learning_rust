use std::io;

fn main() {
    let mut int_type = String::new();
    let mut int = String::new();

    println!("Enter integer type (u8, u16, u32, u64, i8, i16, i32, i64):");
    io::stdin()
        .read_line(&mut int_type)
        .expect("Failed to read input");

    println!("Enter value:");
    io::stdin()
        .read_line(&mut int)
        .expect("Failed to read input");

    let int_type = int_type.trim();
    let int = int.trim();

    match int_type {
        "u8" => match int.parse::<u8>() {
            Ok(value) => println!("Value: {} is valid for type {}", value, int_type),
            Err(_) => println!("Invalid value for type u8"),
        },
        "u16" => match int.parse::<u16>() {
            Ok(value) => println!("Value {} is valid for type {}", value, int_type),
            Err(_) => println!("Invalid value for type u16"),
        },
        "u32" => match int.parse::<u32>() {
            Ok(value) => println!("Value: {} is valid for type {}", value, int_type),
            Err(_) => println!("Invalid value for type u32"),
        },
        "u64" => match int.parse::<u64>() {
            Ok(value) => println!("Value {} is valid for type {}", value, int_type),
            Err(_) => println!("Invalid value for type u64"),
        },
        "i8" => match int.parse::<i8>() {
            Ok(value) => println!("Value: {} is valid for type {}", value, int_type),
            Err(_) => println!("Invalid value for type i8"),
        },
        "i16" => match int.parse::<i16>() {
            Ok(value) => println!("Value {} is valid for type {}", value, int_type),
            Err(_) => println!("Invalid value for type i16"),
        },
        "i32" => match int.parse::<i32>() {
            Ok(value) => println!("Value: {} is valid for type {}", value, int_type),
            Err(_) => println!("Invalid value for type i32"),
        },
        "i64" => match int.parse::<i64>() {
            Ok(value) => println!("Value {} is valid for type {}", value, int_type),
            Err(_) => println!("Invalid value for type i64"),
        },
        _ => println!("Invalid rust integer type"),
    }
}
