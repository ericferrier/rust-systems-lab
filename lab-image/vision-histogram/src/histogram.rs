use image::open;
use std::path::Path;

/// RGB histogram: [R(256), G(256), B(256)]
pub fn compute_rgb_histogram(path: &Path) -> Option<([u32; 256], [u32; 256], [u32; 256])> {
    let img = open(path).ok()?.to_rgb8();

    let mut r_hist = [0u32; 256];
    let mut g_hist = [0u32; 256];
    let mut b_hist = [0u32; 256];

    for pixel in img.pixels() {
        r_hist[pixel[0] as usize] += 1;
        g_hist[pixel[1] as usize] += 1;
        b_hist[pixel[2] as usize] += 1;
    }

    Some((r_hist, g_hist, b_hist))
}

pub fn histogram_similarity(
    a: &[u32; 256],
    b: &[u32; 256],
) -> f64 {
    let mut diff: f64 = 0.0;
    let mut total: f64 = 0.0;

    for i in 0..256 {
        let ai = a[i] as f64;
        let bi = b[i] as f64;

        diff += (ai - bi).abs();
        total += ai + bi;
    }

    if total == 0.0 {
        return 0.0;
    }

    // similarity in %
    (1.0 - (diff / total)) * 100.0
}

pub fn rgb_similarity(
    a: &([u32; 256], [u32; 256], [u32; 256]),
    b: &([u32; 256], [u32; 256], [u32; 256]),
) -> f64 {
    let r = histogram_similarity(&a.0, &b.0);
    let g = histogram_similarity(&a.1, &b.1);
    let b = histogram_similarity(&a.2, &b.2);

    (r + g + b) / 3.0
}