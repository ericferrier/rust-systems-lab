use image::open;

pub fn phash(path: &std::path::Path) -> Option<u64> {
    let img = open(path).ok()?.to_luma8();

    let resized = image::imageops::resize(
        &img,
        32,
        32,
        image::imageops::FilterType::Triangle,
    );

    let mut sum = 0u64;
    let mut pixels = Vec::new();

    for p in resized.pixels() {
        let v = p[0] as u64;
        sum += v;
        pixels.push(v);
    }

    let avg = sum / pixels.len() as u64;

    let mut hash: u64 = 0;

    for (i, &v) in pixels.iter().enumerate() {
        if v > avg {
            hash |= 1 << (i % 64);
        }
    }

    Some(hash)
}

pub fn hamming_distance(a: u64, b: u64) -> u32 {
    (a ^ b).count_ones()
}

pub fn similarity_score(a: u64, b: u64) -> f64 {
    let dist = hamming_distance(a, b);
    (1.0 - (dist as f64 / 64.0)) * 100.0
}