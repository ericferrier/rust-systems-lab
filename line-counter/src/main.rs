use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() {
    let path = get_file_path();

    match count_lines(&path) {
        Ok(count) => println!("Lines: {}", count),
        Err(e) => println!("Error: {}", e),
    }
}

fn get_file_path() -> String {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        args[1].clone()
    } else {
        println!("Usage: cargo run -- <file_path>");
        std::process::exit(1);
    }
}

fn count_lines(path: &str) -> io::Result<usize> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    Ok(reader.lines().count())
}