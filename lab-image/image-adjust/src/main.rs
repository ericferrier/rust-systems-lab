use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use image::{io::Reader as ImageReader, DynamicImage};
use rayon::prelude::*;
use walkdir::WalkDir;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage:");
        println!("cargo run -- <input_dir> <output_dir> [--resize WxH] [--gray] [--blur N]");
        return;
    }

    let input_dir = &args[1];
    let output_dir = &args[2];

    let config = parse_args(&args);

    fs::create_dir_all(output_dir).expect("❌ Cannot create output directory");

    // Collect image paths
    let images: Vec<PathBuf> = WalkDir::new(input_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .map(|e| e.path().to_path_buf())
        .filter(|p| is_image(p))
        .collect();

    println!("🔄 Processing {} images in parallel...", images.len());

    images.par_iter().for_each(|path| {
        if let Err(e) = process_image(path, input_dir, output_dir, &config) {
            eprintln!("❌ Error {:?}: {}", path, e);
        }
    });

    println!("✅ Done");
}


#[derive(Debug)]
struct Config {
    resize: Option<(u32, u32)>,
    grayscale: bool,
    blur: Option<u32>,
}

fn parse_args(args: &[String]) -> Config {
    let mut config = Config {
        resize: None,
        grayscale: false,
        blur: None,
    };

    let mut i = 3;

    while i < args.len() {
        match args[i].as_str() {
            "--resize" => {
                if i + 1 < args.len() {
                    config.resize = parse_size(&args[i + 1]);
                    i += 1;
                }
            }
            "--gray" => {
                config.grayscale = true;
            }
            "--blur" => {
                if i + 1 < args.len() {
                    config.blur = args[i + 1].parse().ok();
                    i += 1;
                }
            }
            _ => {}
        }
        i += 1;
    }

    config
}

fn process_image(
    path: &Path,
    input_root: &str,
    output_root: &str,
    config: &Config,
) -> Result<(), String> {
    let img = ImageReader::open(path)
        .map_err(|e| e.to_string())?
        .decode()
        .map_err(|e| e.to_string())?;

    let mut img = apply_pipeline(img, config);

    let relative = path.strip_prefix(input_root).unwrap();
    let output_path = Path::new(output_root).join(relative);

    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }

    img.save(&output_path).map_err(|e| e.to_string())?;

    Ok(())
}

fn apply_pipeline(mut img: DynamicImage, config: &Config) -> DynamicImage {
    // Resize
    if let Some((w, h)) = config.resize {
        img = img.resize(w, h, image::imageops::FilterType::Lanczos3);
    }

    // Grayscale
    if config.grayscale {
        img = img.grayscale();
    }

    // Blur (simple box blur approximation)
    if let Some(level) = config.blur {
        for _ in 0..level {
            img = img.blur(1.0);
        }
    }

    img
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

fn parse_size(s: &str) -> Option<(u32, u32)> {
    let parts: Vec<&str> = s.split('x').collect();
    if parts.len() != 2 {
        return None;
    }

    Some((parts[0].parse().ok()?, parts[1].parse().ok()?))
}