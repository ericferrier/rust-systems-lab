use std::env;
use std::io::{self, Write};

fn main() {
    let input = get_input();

    let normalized: String = input
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_lowercase().to_string())
        .collect();

    let reversed: String = normalized.chars().rev().collect();

    let is_palindrome = normalized == reversed;

    println!("Input: {}", input);
    println!("Palindrome: {}", is_palindrome);
}

fn get_input() -> String {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        args[1..].join(" ")
    } else {
        print!("Enter text: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim_end().to_string()
    }
}