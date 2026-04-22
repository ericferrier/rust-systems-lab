use std::env;
use std::io::{self, Write};

fn main() {
    let input = get_input();

    let char_count = input.chars().count();
    let char_no_spaces = input.chars().filter(|c| !c.is_whitespace()).count();
    let line_count = input.lines().count();

    println!("Input: {}", input);
    println!("Total characters: {}", char_count);
    println!("Characters (no spaces): {}", char_no_spaces);
    println!("Lines: {}", line_count);
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