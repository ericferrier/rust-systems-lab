use std::env;
use std::io::{self, Write};

fn main() {
    // 1. Get input (argument or stdin)
    let input = get_input();

    // 2. Process sentence
    let words: Vec<&str> = input.split_whitespace().collect();
    let word_count = words.len();

    let longest_word = words
        .iter()
        .max_by_key(|word| word.len())
        .unwrap_or(&"");

    // 3. Output results
    println!("Sentence: {}", input);
    println!("Word count: {}", word_count);
    println!("Longest word: {}", longest_word);
}

fn get_input() -> String {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        // Use CLI argument
        args[1..].join(" ")
    } else {
        // Ask user for input
        print!("Enter a sentence: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    }
}