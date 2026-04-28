mod compress;
mod analyzer;

use std::env;
use std::path::Path;
use walkdir::WalkDir;

use analyzer::analyze_image;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage:");
        println!("cargo run -- <image_or_folder>");
        return;
    }

    let input = Path::new(&args[1]);

    if input.is_file() {
        analyze_image(input);
    } else {
        for entry in WalkDir::new(input)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| e.file_type().is_file())
        {
            let path = entry.path();

            if is_image(path) {
                analyze_image(path);
            }
        }
    }
}

fn is_image(path: &Path) -> bool {
    match path.extension().and_then(|e| e.to_str()) {
        Some(ext) => matches!(
            ext.to_lowercase().as_str(),
            "png" | "jpg" | "jpeg"
        ),
        None => false,
    }
}