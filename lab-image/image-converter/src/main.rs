use std::env;
use std::fs::File;
use std::path::Path;

use image::{
    codecs::jpeg::JpegEncoder,
    ImageReader,
    DynamicImage, ImageFormat,
};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage:");
        println!("cargo run -- <input> <output> [quality:1-100] [resize:WIDTHxHEIGHT]");
        println!("Example:");
        println!("cargo run -- input.png output.jpg 80 800x600");
        return;
    }

    let input_path = &args[1];
    let output_path = &args[2];

    let quality: u8 = args
        .get(3)
        .and_then(|q| q.parse().ok())
        .unwrap_or(85);

    let resize = args.get(4);

    // -----------------------------
    // Load image
    // -----------------------------
    let mut img = match ImageReader::open(input_path) {
        Ok(reader) => reader.decode().expect("❌ Decode failed"),
        Err(e) => {
            eprintln!("❌ Open failed: {}", e);
            return;
        }
    };

    // -----------------------------
    // Resize (BIGGEST size reduction lever)
    // -----------------------------
    if let Some(size) = resize {
        if let Some((w, h)) = parse_resize(size) {
            img = img.resize(w, h, image::imageops::FilterType::Lanczos3);
        }
    }

    // -----------------------------
    // Detect format
    // -----------------------------
    let format = match get_format(output_path) {
        Some(f) => f,
        None => {
            eprintln!("❌ Unsupported output format");
            return;
        }
    };

    // -----------------------------
    // Handle PNG → JPG alpha
    // -----------------------------
    if matches!(format, ImageFormat::Jpeg) {
        img = remove_alpha(img);
    }

    // -----------------------------
    // Save with compression
    // -----------------------------
    match format {
        ImageFormat::Jpeg => save_jpeg(&img, output_path, quality),
        _ => {
            if let Err(e) = img.save(output_path) {
                eprintln!("❌ Save failed: {}", e);
            }
        }
    }

    println!("✅ Converted → {}", output_path);
}

fn save_jpeg(img: &DynamicImage, path: &str, quality: u8) {
    let file = File::create(path).expect("❌ Cannot create file");

    let mut encoder = JpegEncoder::new_with_quality(file, quality);

    encoder
        .encode_image(img)
        .expect("❌ JPEG encoding failed");
}

fn get_format(path: &str) -> Option<ImageFormat> {
    match Path::new(path)
        .extension()
        .and_then(|e| e.to_str())
        .map(|s| s.to_lowercase())
        .as_deref()
    {
        Some("jpg") | Some("jpeg") => Some(ImageFormat::Jpeg),
        Some("png") => Some(ImageFormat::Png),
        _ => None,
    }
}

fn parse_resize(s: &str) -> Option<(u32, u32)> {
    let parts: Vec<&str> = s.split('x').collect();
    if parts.len() != 2 {
        return None;
    }

    let w = parts[0].parse().ok()?;
    let h = parts[1].parse().ok()?;

    Some((w, h))
}

fn remove_alpha(img: DynamicImage) -> DynamicImage {
    let rgba = img.to_rgba8();
    let rgb = image::ImageBuffer::from_fn(rgba.width(), rgba.height(), |x, y| {
        let p = rgba.get_pixel(x, y);
        // Alpha blending with white background
        let alpha = p[3] as f32 / 255.0;
        let r = ((p[0] as f32 * alpha) + (255.0 * (1.0 - alpha))) as u8;
        let g = ((p[1] as f32 * alpha) + (255.0 * (1.0 - alpha))) as u8;
        let b = ((p[2] as f32 * alpha) + (255.0 * (1.0 - alpha))) as u8;
        image::Rgb([r, g, b])
    });
    DynamicImage::ImageRgb8(rgb)
}