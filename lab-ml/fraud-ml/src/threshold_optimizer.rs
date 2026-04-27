pub fn find_best_threshold(
    y_true: &[f64],
    y_prob: &[f64],
    steps: usize,
) -> (f64, f64) {
    let mut best_threshold = 0.5;
    let mut best_f1 = 0.0;

    for i in 0..steps {
        let threshold = i as f64 / steps as f64;

        let preds: Vec<f64> = y_prob
            .iter()
            .map(|p| if *p >= threshold { 1.0 } else { 0.0 })
            .collect();

        let (tp, fp, _tn, fn_) = confusion(y_true, &preds);

        let precision = tp / (tp + fp + 1e-12);
        let recall = tp / (tp + fn_ + 1e-12);

        let f1 = 2.0 * (precision * recall) / (precision + recall + 1e-12);

        if f1 > best_f1 {
            best_f1 = f1;
            best_threshold = threshold;
        }
    }

    (best_threshold, best_f1)
}

pub fn confusion(
    y_true: &[f64],
    y_pred: &[f64],
) -> (f64, f64, f64, f64) {
    let mut tp = 0.0;
    let mut fp = 0.0;
    let mut tn = 0.0;
    let mut fn_ = 0.0;

    for i in 0..y_true.len() {
        match (y_true[i], y_pred[i]) {
            (1.0, 1.0) => tp += 1.0,
            (0.0, 1.0) => fp += 1.0,
            (0.0, 0.0) => tn += 1.0,
            (1.0, 0.0) => fn_ += 1.0,
            _ => {}
        }
    }

    (tp, fp, tn, fn_)
}