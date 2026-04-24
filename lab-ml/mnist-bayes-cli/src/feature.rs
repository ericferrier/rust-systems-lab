pub fn extract_features(pixels: &[f64]) -> Vec<f64> {
    let mean = pixels.iter().sum::<f64>() / pixels.len() as f64;

    let variance = pixels.iter()
        .map(|v| (v - mean).powi(2))
        .sum::<f64>() / pixels.len() as f64;

    let stddev = variance.sqrt();

    vec![mean, stddev]
}