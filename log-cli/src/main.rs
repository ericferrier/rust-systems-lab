use std::env;
use std::fs;

fn main() {
    let path = get_path();

    let content = fs::read_to_string(&path)
        .expect("Failed to read log file");

    let mut info = 0;
    let mut warn = 0;
    let mut error = 0;
    let mut debug = 0;

    let mut error_lines: Vec<&str> = Vec::new();

    for line in content.lines() {
        if line.starts_with("INFO") {
            info += 1;
        } else if line.starts_with("WARN") {
            warn += 1;
        } else if line.starts_with("ERROR") {
            error += 1;
            error_lines.push(line);
        } else if line.starts_with("DEBUG") {
            debug += 1;
        }
    }

    let total = info + warn + error + debug;

    println!("File: {}", path);
    println!("Total lines: {}", total);
    println!("INFO: {}", info);
    println!("WARN: {}", warn);
    println!("ERROR: {}", error);
    println!("DEBUG: {}", debug);

    println!("\n--- Error Logs ---");
    for e in error_lines {
        println!("{}", e);
    }

    println!("\n--- Health Summary ---");
    if error > 0 {
        println!("System Status: DEGRADED");
    } else {
        println!("System Status: OK");
    }
}

fn get_path() -> String {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        args[1].clone()
    } else {
        println!("Usage: cargo run -- data/sample.log");
        std::process::exit(1);
    }
}