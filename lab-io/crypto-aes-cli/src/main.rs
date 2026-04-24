mod crypto;

use std::env;
use std::fs;

use crypto::xor_cipher;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 5 {
        println!("Usage:");
        println!("cargo run -- <encrypt|decrypt> <input> <output> <key>");
        return;
    }

    let mode = &args[1];
    let input_file = &args[2];
    let output_file = &args[3];
    let key = args[4].as_bytes();

    if key.is_empty() {
        println!("Key cannot be empty");
        return;
    }

    let data = fs::read(input_file).expect("Failed to read input file");

    let result = xor_cipher(&data, key);

    fs::write(output_file, result).expect("Failed to write output file");

    println!("Done: {}", mode);
}