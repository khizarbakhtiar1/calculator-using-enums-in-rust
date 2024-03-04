use std::io::{self, Write};
use std::str::FromStr;

enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

fn calculate(op: Operation) -> f64 {
    match op {
        Operation::Add(a, b) => a + b,
        Operation::Subtract(a, b) => a - b,
        Operation::Multiply(a, b) => a * b,
        Operation::Divide(a, b) => a / b,
    }
}

fn main() {
    let mut input = String::new();
    print!("Enter the first number: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let num1: f64 = f64::from_str(input.trim()).unwrap();

    let mut input = String::new();
    print!("Enter the operation (+, -, *, /): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let operation = input.trim();

    let mut input = String::new();
    print!("Enter the second number: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let num2: f64 = f64::from_str(input.trim()).unwrap();

    let op = match operation {
        "+" => Operation::Add(num1, num2),
        "-" => Operation::Subtract(num1, num2),
        "*" => Operation::Multiply(num1, num2),
        "/" => Operation::Divide(num1, num2),
        _ => panic!("Unknown operation"),
    };

    let result = calculate(op);
    println!("The result is: {}", result);
}
