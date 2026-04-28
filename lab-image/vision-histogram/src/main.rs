mod histogram;

use std::env;
use std::path::Path;

use histogram::{compute_rgb_histogram, rgb_similarity};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage:");
        println!("cargo run -- <image1> <image2>");
        println!("cargo run -- <image>");
        return;
    }

    let img1 = Path::new(&args[1]);

    // SINGLE IMAGE MODE
    if args.len() == 2 {
        if let Some((r_hist, g_hist, b_hist)) = compute_rgb_histogram(img1) {
            println!("Image loaded successfully.");
            let total_pixels: u32 = r_hist.iter().sum();
            let (max_r, max_r_count) = r_hist.iter().enumerate().max_by_key(|&(_, &v)| v).unwrap();
            let (max_g, max_g_count) = g_hist.iter().enumerate().max_by_key(|&(_, &v)| v).unwrap();
            let (max_b, max_b_count) = b_hist.iter().enumerate().max_by_key(|&(_, &v)| v).unwrap();
            println!("\n--- HISTOGRAM REPORT ---");
            println!("Total pixels: {}", total_pixels);
            println!("Most common R value: {} ({} pixels)", max_r, max_r_count);
            println!("Most common G value: {} ({} pixels)", max_g, max_g_count);
            println!("Most common B value: {} ({} pixels)", max_b, max_b_count);
        }
        return;
    }

    // COMPARISON MODE
    let img2 = Path::new(&args[2]);

    let h1 = match compute_rgb_histogram(img1) {
        Some(h) => h,
        None => {
            println!("Failed to load image 1");
            return;
        }
    };

    let h2 = match compute_rgb_histogram(img2) {
        Some(h) => h,
        None => {
            println!("Failed to load image 2");
            return;
        }
    };

    let similarity = rgb_similarity(&h1, &h2);

    println!("\n--- HISTOGRAM SIMILARITY ---");
    println!("Image 1: {:?}", img1);
    println!("Image 2: {:?}", img2);
    println!("Similarity: {:.2}%", similarity);
}