mod scanner;
mod phash;
mod report;
mod hash;

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::collections::HashMap;

use scanner::scan_folder;
use phash::similarity_score;
use hash::sha256_file;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage:");
        println!("cargo run -- <folder> --mode similar --threshold 85");
        println!("cargo run -- <folder> --mode exact");
        return;
    }

    let folder = &args[1];

    let mut mode = "similar";
    let mut threshold: f64 = 85.0;

    for i in 0..args.len() {
        if args[i] == "--mode" {
            mode = &args[i + 1];
        }
        if args[i] == "--threshold" {
            threshold = args[i + 1].parse().unwrap_or(85.0);
        }
    }

    println!("Scanning folder...\n");

    let entries = scan_folder(folder);

    // Analyze all images for similarity (report only)
    report::analyze(&entries, threshold);

    let output_dir = Path::new("output/duplicates");
    fs::create_dir_all(&output_dir).unwrap();

    match mode {
        // -------------------------
        // 🎯 EXACT DUPLICATES (SHA256)
        // -------------------------
        "exact" => {
            let mut seen: HashMap<String, PathBuf> = HashMap::new();
            let mut moved = 0;

            for entry in entries {
                let hash = match sha256_file(&entry.path) {
                    Some(h) => h,
                    None => continue,
                };

                if let Some(original) = seen.get(&hash) {
                    println!(
                        "EXACT DUPLICATE:\n  {:?}\n  original: {:?}\n",
                        entry.path, original
                    );

                    move_file(&entry.path, output_dir);
                    moved += 1;
                } else {
                    seen.insert(hash, entry.path.clone());
                }
            }

            println!("\n--- EXACT REPORT ---");
            println!("Files scanned: {}", seen.len());
            println!("Duplicates moved: {}", moved);
        }

        // -------------------------
        // 🧠 PERCEPTUAL SIMILARITY
        // -------------------------
        "similar" => {
            let mut moved = 0;

            for i in 0..entries.len() {
                for j in i + 1..entries.len() {
                    let a = &entries[i];
                    let b = &entries[j];

                    let score = similarity_score(a.hash, b.hash);

                    if score >= threshold {
                        println!(
                            "SIMILAR ({}%):\n  {:?}\n",
                            score as u32,
                            b.path
                        );

                        move_file(&b.path, output_dir);
                        moved += 1;
                    }
                }
            }

            println!("\n--- SIMILAR REPORT ---");
            println!("Images scanned: {}", entries.len());
            println!("Moved duplicates: {}", moved);
            println!("Threshold: {}%", threshold);
        }

        _ => println!("Unknown mode"),
    }
}

fn move_file(path: &PathBuf, output_dir: &Path) {
    if let Some(file_name) = path.file_name() {
        let new_path = output_dir.join(file_name);
        let _ = fs::rename(path, new_path);
    }
}