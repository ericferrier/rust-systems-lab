use std::env;
use image::{ImageReader, imageops::FilterType};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        println!("Usage:");
        println!("cargo run -- <input> <output> <WIDTHxHEIGHT> [mode:fit|fill] [quality]");
        println!();
        println!("Examples:");
        println!("cargo run -- input.jpg output.jpg 800x600");
        println!("cargo run -- input.jpg output.jpg 800x600 fit");
        println!("cargo run -- input.jpg output.jpg 800x600 fill 80");
        return;
    }

    let input = &args[1];
    let output = &args[2];
    let (width, height) = parse_size(&args[3]);

    let mode = args.get(4).map(|s| s.as_str()).unwrap_or("fit");
    let quality: u8 = args.get(5).and_then(|q| q.parse().ok()).unwrap_or(85);

    // -----------------------------
    // Load image
    // -----------------------------
    let img = match ImageReader::open(input) {
        Ok(r) => r.decode().expect("❌ Decode failed"),
        Err(e) => {
            eprintln!("❌ Open failed: {}", e);
            return;
        }
    };

    // -----------------------------
    // Resize
    // -----------------------------
    let resized = match mode {
        "fill" => img.resize_to_fill(width, height, FilterType::Lanczos3),
        _ => img.resize(width, height, FilterType::Lanczos3),
    };

    // -----------------------------
    // Save (with optional compression)
    // -----------------------------
    if output.ends_with(".jpg") || output.ends_with(".jpeg") {
        save_jpeg(&resized, output, quality);
    } else {
        resized.save(output).expect("❌ Save failed");
    }

    println!("✅ Resized image saved to {}", output);
}

// -----------------------------
// Helpers
// -----------------------------
fn parse_size(s: &str) -> (u32, u32) {
    let parts: Vec<&str> = s.split('x').collect();
    if parts.len() != 2 {
        panic!("❌ Invalid size format. Use WIDTHxHEIGHT");
    }

    let w = parts[0].parse().expect("Invalid width");
    let h = parts[1].parse().expect("Invalid height");

    (w, h)
}

fn save_jpeg(img: &image::DynamicImage, path: &str, quality: u8) {
    use std::fs::File;
    use image::codecs::jpeg::JpegEncoder;

    let file = File::create(path).expect("❌ Cannot create file");
    let mut encoder = JpegEncoder::new_with_quality(file, quality);

    encoder.encode_image(img).expect("❌ Encoding failed");
}