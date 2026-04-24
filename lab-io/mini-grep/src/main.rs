use colored::*;
use regex::Regex;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use walkdir::WalkDir;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage:");
        eprintln!("cargo run -- <pattern> <path> [--recursive]");
        std::process::exit(1);
    }

    let pattern = &args[1];
    let path = &args[2];
    let recursive = args.get(3).map_or(false, |a| a == "--recursive");

    let regex = Regex::new(pattern).unwrap_or_else(|err| {
        eprintln!("Invalid regex: {}", err);
        std::process::exit(1);
    });

    if recursive {
        search_dir(path, &regex);
    } else {
        let meta = std::fs::metadata(path).unwrap_or_else(|err| {
            eprintln!("Error accessing {}: {}", path, err);
            std::process::exit(1);
        });
        if meta.is_dir() {
            eprintln!("Error: {} is a directory. Use --recursive to search directories.", path);
            std::process::exit(1);
        }
        search_file(path, &regex);
    }
}


fn search_file(path: &str, regex: &Regex) {
    let file = File::open(path).unwrap_or_else(|err| {
        eprintln!("Error opening file {}: {}", path, err);
        std::process::exit(1);
    });

    let reader = BufReader::new(file);

    for (line_num, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        if regex.is_match(&line) {
            print_match(path, line_num + 1, &line, regex);
        }
    }
}


fn search_dir(path: &str, regex: &Regex) {
    for entry in WalkDir::new(path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        let file_path = entry.path().display().to_string();
        search_file(&file_path, regex);
    }
}

fn print_match(file: &str, line_num: usize, line: &str, regex: &Regex) {
    let mut highlighted = line.to_string();

    // Replace matches with colored version
    for mat in regex.find_iter(line) {
        let matched_text = mat.as_str();
        let colored = matched_text.red().bold().to_string();
        highlighted = highlighted.replace(matched_text, &colored);
    }

    println!(
        "{}:{}: {}",
        file.blue(),
        line_num.to_string().yellow(),
        highlighted
    );
}