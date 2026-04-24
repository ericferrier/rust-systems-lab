use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let dir = get_directory();

    let entries = fs::read_dir(&dir)
        .expect("Failed to read directory");

    for entry in entries {
        let entry = entry.expect("Invalid entry");
        let path = entry.path();

        if path.is_file() {
            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                let folder = map_extension(ext);

                let target_dir = format!("{}/{}", dir, folder);

                fs::create_dir_all(&target_dir)
                    .expect("Failed to create folder");

                let file_name = path.file_name().unwrap();
                let new_path = format!("{}/{}", target_dir, file_name.to_string_lossy());

                fs::rename(&path, &new_path)
                    .expect("Failed to move file");
            }
        }
    }

    println!("Files organized successfully in: {}", dir);
}

fn map_extension(ext: &str) -> &str {
    match ext.to_lowercase().as_str() {
        "png" | "jpg" | "jpeg" | "gif" => "images",
        "pdf" => "pdf",
        "txt" => "text",
        "rs" => "rust",
        "json" => "json",
        "csv" => "csv",
        _ => "other",
    }
}

fn get_directory() -> String {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        args[1].clone()
    } else {
        println!("Usage: cargo run -- <directory>");
        std::process::exit(1);
    }
}