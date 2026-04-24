pub fn generate_signal(spectrum: &[f32]) -> String {
    let mut max_idx = 0;
    let mut max_val = 0.0;

    for (i, v) in spectrum.iter().enumerate() {
        if *v > max_val {
            max_val = *v;
            max_idx = i;
        }
    }

    // interpret frequency bin
    if max_idx < 5 {
        "HOLD (low frequency trend)".to_string()
    } else if max_idx < 20 {
        "LONG (mid-cycle momentum)".to_string()
    } else {
        "SHORT / NOISE (high frequency)".to_string()
    }
}