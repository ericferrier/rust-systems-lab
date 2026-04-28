use crate::phash::similarity_score;
use crate::scanner::ImageEntry;

pub fn analyze(entries: &[ImageEntry], threshold: f64) {
    println!("Analyzing {} images...\n", entries.len());

    let mut duplicates = 0;

    for i in 0..entries.len() {
        for j in i + 1..entries.len() {
            let a = &entries[i];
            let b = &entries[j];

            let score = similarity_score(a.hash, b.hash);

            if score >= threshold {
                duplicates += 1;

                println!(
                    "SIMILAR ({}%):\n  {:?}\n  {:?}\n",
                    score as u32, a.path, b.path
                );
            }
        }
    }

    println!("\n--- REPORT ---");
    println!("Images scanned: {}", entries.len());
    println!("Similar pairs: {}", duplicates);
    println!("Threshold: {}%", threshold);
}