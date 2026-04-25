use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use image::{ImageReader, GenericImageView};
use rayon::prelude::*;
use walkdir::WalkDir;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        println!("Usage:");
        println!("cargo run -- <input_dir> <output_dir> <max_size>");
        println!("Example:");
        println!("cargo run -- ./images ./thumbs 200");
        return;
    }

    let input_dir = &args[1];
    let output_dir = &args[2];
    let max_size: u32 = args[3].parse().expect("Invalid size");

    fs::create_dir_all(output_dir).expect("❌ Cannot create output folder");

    let images: Vec<PathBuf> = WalkDir::new(input_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .map(|e| e.path().to_path_buf())
        .filter(|p| is_image(p))
        .collect();

    println!("🔄 Generating thumbnails for {} images...", images.len());

    images.par_iter().for_each(|path| {
        if let Err(e) = process_thumbnail(path, input_dir, output_dir, max_size) {
            eprintln!("❌ {:?}: {}", path, e);
        }
    });

    println!("✅ Done");
}

fn process_thumbnail(
    path: &Path,
    input_root: &str,
    output_root: &str,
    max_size: u32,
) -> Result<(), String> {
    let img = ImageReader::open(path)
        .map_err(|e| e.to_string())?
        .decode()
        .map_err(|e| e.to_string())?;

    let thumbnail = create_thumbnail(img, max_size);

    let relative = path.strip_prefix(input_root).unwrap();
    let output_path = Path::new(output_root).join(relative);

    if let Some(parent) = output_path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }

    thumbnail
        .save(&output_path)
        .map_err(|e| e.to_string())?;

    Ok(())
}

fn create_thumbnail(img: image::DynamicImage, max_size: u32) -> image::DynamicImage {
    let (w, h) = img.dimensions();

    let scale = (max_size as f32 / w.max(h) as f32).min(1.0);

    let new_w = (w as f32 * scale) as u32;
    let new_h = (h as f32 * scale) as u32;

    img.resize(new_w, new_h, image::imageops::FilterType::Lanczos3)
}

fn is_image(path: &Path) -> bool {
    match path.extension().and_then(|e| e.to_str()) {
        Some(ext) => matches!(
            ext.to_lowercase().as_str(),
            "png" | "jpg" | "jpeg" | "bmp"
        ),
        None => false,
    }
}