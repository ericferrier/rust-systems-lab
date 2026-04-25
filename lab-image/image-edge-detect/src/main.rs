use std::env;
use image::{io::Reader as ImageReader, DynamicImage};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage:");
        println!("cargo run -- <input> <output>");
        return;
    }

    let input = &args[1];
    let output = &args[2];

    // Load image
    let img = match ImageReader::open(input) {
        Ok(r) => r.decode().expect("❌ Decode failed"),
        Err(e) => {
            eprintln!("❌ Open failed: {}", e);
            return;
        }
    };

    let edges = sobel(&img);

    edges.save(output).expect("❌ Save failed");

    println!("✅ Edge detection complete → {}", output);
}

fn sobel(img: &DynamicImage) -> DynamicImage {
    use image::{ImageBuffer, Luma};

    let gray = img.to_luma8();
    let (w, h) = gray.dimensions();

    let mut out = ImageBuffer::<Luma<u8>, Vec<u8>>::new(w, h);

    let gx_kernel: [[i32; 3]; 3] = [
        [-1, 0, 1],
        [-2, 0, 2],
        [-1, 0, 1],
    ];

    let gy_kernel: [[i32; 3]; 3] = [
        [-1, -2, -1],
        [0,  0,  0],
        [1,  2,  1],
    ];

    for y in 1..h - 1 {
        for x in 1..w - 1 {
            let mut gx = 0i32;
            let mut gy = 0i32;

            for ky in 0..3 {
                for kx in 0..3 {
                    let pixel = gray.get_pixel((x + kx - 1), (y + ky - 1))[0] as i32;

                    gx += pixel * gx_kernel[ky as usize][kx as usize];
                    gy += pixel * gy_kernel[ky as usize][kx as usize];
                }
            }

            let magnitude = ((gx * gx + gy * gy) as f32).sqrt();

            let edge_value = magnitude.clamp(0.0, 255.0) as u8;

            out.put_pixel(x, y, Luma([edge_value]));
        }
    }

    DynamicImage::ImageLuma8(out)
}