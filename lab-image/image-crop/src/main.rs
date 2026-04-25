use std::env;
use image::{ImageReader, DynamicImage, GenericImageView};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        println!("Usage:");
        println!("cargo run -- <input> <output> <mode> [params]");
        println!();
        println!("Modes:");
        println!(" center WIDTHxHEIGHT");
        println!(" coords X Y WIDTH HEIGHT");
        println!();
        println!("Examples:");
        println!("cargo run -- input.jpg output.jpg center 400x400");
        println!("cargo run -- input.jpg output.jpg coords 100 100 300 300");
        return;
    }

    let input = &args[1];
    let output = &args[2];
    let mode = &args[3];

    // Load image
    let img = match ImageReader::open(input) {
        Ok(r) => r.decode().expect("❌ Decode failed"),
        Err(e) => {
            eprintln!("❌ Open failed: {}", e);
            return;
        }
    };

    let cropped = match mode.as_str() {
        "center" => {
            let (w, h) = parse_size(&args[4]);
            crop_center(&img, w, h)
        }
        "coords" => {
            let x: u32 = args[4].parse().expect("Invalid X");
            let y: u32 = args[5].parse().expect("Invalid Y");
            let w: u32 = args[6].parse().expect("Invalid width");
            let h: u32 = args[7].parse().expect("Invalid height");

            crop_coords(&img, x, y, w, h)
        }
        _ => {
            eprintln!("❌ Unknown mode");
            return;
        }
    };

    cropped.save(output).expect("❌ Save failed");

    println!("✅ Cropped image saved to {}", output);
}

// -----------------------------
// Crop center
// -----------------------------
fn crop_center(img: &DynamicImage, crop_w: u32, crop_h: u32) -> DynamicImage {
    let (img_w, img_h) = img.dimensions();

    let x = (img_w.saturating_sub(crop_w)) / 2;
    let y = (img_h.saturating_sub(crop_h)) / 2;

    img.crop_imm(x, y, crop_w.min(img_w), crop_h.min(img_h))
}

// -----------------------------
// Crop by coordinates
// -----------------------------
fn crop_coords(
    img: &DynamicImage,
    x: u32,
    y: u32,
    w: u32,
    h: u32,
) -> DynamicImage {
    let (img_w, img_h) = img.dimensions();

    let w = w.min(img_w.saturating_sub(x));
    let h = h.min(img_h.saturating_sub(y));

    img.crop_imm(x, y, w, h)
}

// -----------------------------
// Helpers
// -----------------------------
fn parse_size(s: &str) -> (u32, u32) {
    let parts: Vec<&str> = s.split('x').collect();
    if parts.len() != 2 {
        panic!("❌ Invalid size format (use WIDTHxHEIGHT)");
    }

    let w = parts[0].parse().expect("Invalid width");
    let h = parts[1].parse().expect("Invalid height");

    (w, h)
}