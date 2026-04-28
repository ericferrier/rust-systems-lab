use crate::compress::compress_size;
use image::open;
use std::path::Path;

pub fn analyze_image(path: &Path) {
    let img = match open(path) {
        Ok(i) => i,
        Err(_) => return,
    };

    let original_size = std::fs::metadata(path).unwrap().len();

    let qualities = [100, 90, 80, 70, 50, 30, 10];

    println!("\n========================");
    println!("Image: {:?}", path);
    println!("Original size: {:.2} KB", original_size as f64 / 1024.0);

    for q in qualities {
        let size = compress_size(&img, q);
        let ratio = (size as f64 / original_size as f64) * 100.0;

        println!(
            "Q{:>3} → {:>6} KB ({:.1}%)",
            q,
            size as f64 / 1024.0,
            ratio
        );
    }
}