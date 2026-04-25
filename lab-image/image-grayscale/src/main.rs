use std::env;
use image::{ImageReader, DynamicImage};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage: cargo run -- <input> <output>");
        return;
    }

    let input_path = &args[1];
    let output_path = &args[2];

    // Load image
    let img = match ImageReader::open(input_path) {
        Ok(reader) => match reader.decode() {
            Ok(img) => img,
            Err(e) => {
                eprintln!("❌ Failed to decode image: {}", e);
                return;
            }
        },
        Err(e) => {
            eprintln!("❌ Failed to open image: {}", e);
            return;
        }
    };

    // Convert to grayscale
    let gray = to_grayscale(img);

    // Save output
    if let Err(e) = gray.save(output_path) {
        eprintln!("❌ Failed to save image: {}", e);
        return;
    }

    println!("✅ Grayscale image saved to {}", output_path);
}

fn to_grayscale(img: DynamicImage) -> DynamicImage {
    img.grayscale()
}