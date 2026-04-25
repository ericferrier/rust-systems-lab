use std::env;
use image::{ImageReader, DynamicImage, GenericImageView};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        println!("Usage:");
        println!("cargo run -- <input> <output> <angle>");
        println!();
        println!("Examples:");
        println!("cargo run -- input.jpg output.jpg 90");
        println!("cargo run -- input.jpg output.jpg 180");
        println!("cargo run -- input.jpg output.jpg 270");
        println!("cargo run -- input.jpg output.jpg 45");
        return;
    }

    let input = &args[1];
    let output = &args[2];
    let angle: f32 = args[3].parse().expect("❌ Invalid angle");

    // Load image
    let img = match ImageReader::open(input) {
        Ok(r) => r.decode().expect("❌ Decode failed"),
        Err(e) => {
            eprintln!("❌ Open failed: {}", e);
            return;
        }
    };

    let rotated = rotate_image(img, angle);

    rotated.save(output).expect("❌ Save failed");

    println!("✅ Rotated {}° → {}", angle, output);
}

fn rotate_image(img: DynamicImage, angle: f32) -> DynamicImage {
    match angle as i32 {
        90 => img.rotate90(),
        180 => img.rotate180(),
        270 => img.rotate270(),
        _ => rotate_custom(img, angle),
    }
}

fn rotate_custom(img: DynamicImage, angle_deg: f32) -> DynamicImage {
    use image::{ImageBuffer, Rgba};
    use std::f32::consts::PI;

    let radians = angle_deg * PI / 180.0;

    let (w, h) = img.dimensions();

    let src = img.to_rgba8();
    let mut dst = ImageBuffer::<Rgba<u8>, Vec<u8>>::new(w, h);

    let cx = w as f32 / 2.0;
    let cy = h as f32 / 2.0;

    let cos_a = radians.cos();
    let sin_a = radians.sin();

    for y in 0..h {
        for x in 0..w {
            // translate to center
            let tx = x as f32 - cx;
            let ty = y as f32 - cy;

            // inverse rotation
            let src_x = (tx * cos_a + ty * sin_a + cx).round();
            let src_y = (-tx * sin_a + ty * cos_a + cy).round();

            if src_x >= 0.0 && src_x < w as f32 && src_y >= 0.0 && src_y < h as f32 {
                let px = src.get_pixel(src_x as u32, src_y as u32);
                dst.put_pixel(x, y, *px);
            }
        }
    }

    DynamicImage::ImageRgba8(dst)
}