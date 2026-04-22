use std::env;
use std::io::{self, Write};

fn main() {
    let input = get_input();

    match calculate(&input) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}

fn get_input() -> String {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        args[1..].join(" ")
    } else {
        print!("Enter expression (e.g. 2 + 3): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    }
}

fn calculate(input: &str) -> Result<f64, String> {
    let parts: Vec<&str> = input.split_whitespace().collect();

    if parts.len() != 3 {
        return Err("Invalid format. Use: number operator number".into());
    }

    let left: f64 = parts[0].parse().map_err(|_| "Invalid number")?;
    let operator = parts[1];
    let right: f64 = parts[2].parse().map_err(|_| "Invalid number")?;

    match operator {
        "+" => Ok(left + right),
        "-" => Ok(left - right),
        "*" => Ok(left * right),
        "/" => {
            if right == 0.0 {
                Err("Division by zero".into())
            } else {
                Ok(left / right)
            }
        }
        _ => Err("Unsupported operator".into()),
    }
}