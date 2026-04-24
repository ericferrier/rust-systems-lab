use std::env;
use std::fs;

use serde_json::Value;

fn main() {
    let path = get_path();

    let content = fs::read_to_string(&path)
        .expect("Failed to read JSON file");

    let json: Value = serde_json::from_str(&content)
        .expect("Invalid JSON format");

    println!("File: {}\n", path);

    // Pretty print JSON
    println!("--- JSON CONTENT ---");
    println!("{}", serde_json::to_string_pretty(&json).unwrap());

    // Extract fields safely
    println!("\n--- Extracted Fields ---");

    if let Some(app) = json.get("app") {
        println!("App: {}", app);
    }

    if let Some(version) = json.get("version") {
        println!("Version: {}", version);
    }

    if let Some(users) = json.get("users") {
        println!("Users: {}", users);
    }

    if let Some(errors) = json.get("errors") {
        println!("Errors: {}", errors);
    }

    // Count top-level keys
    if let Some(obj) = json.as_object() {
        println!("\nTop-level keys: {}", obj.len());
    }

    // Nested access example
    if let Some(services) = json.get("services") {
        println!("\n--- Services ---");
        if let Some(auth) = services.get("auth") {
            println!("auth: {}", auth);
        }
        if let Some(db) = services.get("db") {
            println!("db: {}", db);
        }
    }
}

fn get_path() -> String {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        args[1].clone()
    } else {
        println!("Usage: cargo run -- data/sample.json");
        std::process::exit(1);
    }
}