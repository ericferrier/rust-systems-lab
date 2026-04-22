use std::env;

fn main() {
    let url = get_url();

    match send_request(&url) {
        Ok(body) => {
            println!("URL: {}", url);
            println!("--- Response Body ---");
            println!("{}", body);
        }
        Err(e) => {
            println!("Request failed: {}", e);
        }
    }
}

fn send_request(url: &str) -> Result<String, reqwest::Error> {
    let response = reqwest::blocking::get(url)?;

    println!("Status: {}", response.status());

    let body = response.text()?;
    Ok(body)
}

fn get_url() -> String {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        args[1].clone()
    } else {
        println!("Usage: cargo run -- https://quopa.io");
        std::process::exit(1);
    }
}