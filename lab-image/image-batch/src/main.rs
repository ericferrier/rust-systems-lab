use std::env;
use std::fs;
use std::path::Path;

use image::{ImageReader, DynamicImage};
use walkdir::WalkDir;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage:");
        println!("cargo run -- <input_folder> <output_folder>");
        return;
    }

    let input_dir = &args[1];
    let output_dir = &args[2];

    fs::create_dir_all(output_dir).expect("❌ Cannot create output folder");

    let mut count = 0;

    for entry in WalkDir::new(input_dir)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();

        if path.is_file() && is_image(path) {
            match process_image(path, input_dir, output_dir) {
                Ok(_) => count += 1,
                Err(e) => eprintln!("❌ Error processing {:?}: {}", path, e),
            }
        }
    }

    println!("✅ Processed {} images", count);
}


fn process_image(
    path: &Path,
    input_root: &str,
    output_root: &str,
) -> Result<(), String> {
    let img = ImageReader::open(path)
        .map_err(|e| e.to_string())?
        .decode()
        .map_err(|e| e.to_string())?;

    // 🔧 PIPELINE STEP (you can plug your tools here)
    let processed = pipeline(img);

    let relative = path.strip_prefix(input_root).unwrap();
    let output_path = Path::new(output_root).join(relative);

    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }

    processed
        .save(&output_path)
        .map_err(|e| e.to_string())?;

    Ok(())
}

fn pipeline(img: DynamicImage) -> DynamicImage {
    // Example pipeline (you can customize freely)

    let resized = img.resize(800, 800, image::imageops::FilterType::Lanczos3);
    let gray = resized.grayscale();

    gray
}

fn is_image(path: &Path) -> bool {
    match path.extension().and_then(|e| e.to_str()) {
        Some(ext) => {
            matches!(
                ext.to_lowercase().as_str(),
                "png" | "jpg" | "jpeg" | "bmp" | "tiff"
            )
        }
        None => false,
    }
}