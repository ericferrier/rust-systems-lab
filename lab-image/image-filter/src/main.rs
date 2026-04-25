use std::env;
use image::{ImageReader, DynamicImage};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        println!("Usage:");
        println!("cargo run -- <input> <output> <filter>");
        println!();
        println!("Filters:");
        println!(" blur");
        println!(" sharpen");
        return;
    }

    let input = &args[1];
    let output = &args[2];
    let filter = &args[3].as_str();

    // Load image
    let img = match ImageReader::open(input) {
        Ok(r) => r.decode().expect("❌ Decode failed"),
        Err(e) => {
            eprintln!("❌ Open failed: {}", e);
            return;
        }
    };

    let processed = match *filter {
        "blur" => blur(&img),
        "sharpen" => sharpen(&img),
        _ => {
            eprintln!("❌ Unknown filter: use blur or sharpen");
            return;
        }
    };

    processed.save(output).expect("❌ Save failed");

    println!("✅ {} filter applied → {}", filter, output);
}

fn blur(img: &DynamicImage) -> DynamicImage {
    use image::{ImageBuffer, Rgba};

    let src = img.to_rgba8();
    let (w, h) = src.dimensions();

    let mut dst = ImageBuffer::new(w, h);

    let kernel: [(i32, i32); 9] = [
        (-1, -1), (0, -1), (1, -1),
        (-1,  0), (0,  0), (1,  0),
        (-1,  1), (0,  1), (1,  1),
    ];

    for y in 1..h - 1 {
        for x in 1..w - 1 {
            let mut r = 0u32;
            let mut g = 0u32;
            let mut b = 0u32;

            for (dx, dy) in kernel {
                let px = src.get_pixel((x as i32 + dx) as u32, (y as i32 + dy) as u32);
                r += px[0] as u32;
                g += px[1] as u32;
                b += px[2] as u32;
            }

            dst.put_pixel(
                x,
                y,
                Rgba([
                    (r / 9) as u8,
                    (g / 9) as u8,
                    (b / 9) as u8,
                    255,
                ]),
            );
        }
    }

    DynamicImage::ImageRgba8(dst)
}

fn sharpen(img: &DynamicImage) -> DynamicImage {
    use image::{ImageBuffer, Rgba};

    let src = img.to_rgba8();
    let (w, h) = src.dimensions();

    let mut dst = ImageBuffer::new(w, h);

    let kernel: [(i32, i32, i32); 5] = [
        (0,  0,  5),
        (-1, 0, -1),
        (1,  0, -1),
        (0, -1, -1),
        (0,  1, -1),
    ];

    for y in 1..h - 1 {
        for x in 1..w - 1 {
            let mut r = 0i32;
            let mut g = 0i32;
            let mut b = 0i32;

            for (dx, dy, weight) in kernel {
                let px = src.get_pixel((x as i32 + dx) as u32, (y as i32 + dy) as u32);

                r += px[0] as i32 * weight;
                g += px[1] as i32 * weight;
                b += px[2] as i32 * weight;
            }

            dst.put_pixel(
                x,
                y,
                Rgba([
                    r.clamp(0, 255) as u8,
                    g.clamp(0, 255) as u8,
                    b.clamp(0, 255) as u8,
                    255,
                ]),
            );
        }
    }

    DynamicImage::ImageRgba8(dst)
}