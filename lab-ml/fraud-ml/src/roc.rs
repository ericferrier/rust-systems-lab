
pub fn compute_auc(y_true: &[f64], y_prob: &[f64]) -> f64 {
    let mut pairs: Vec<(f64, f64)> = y_prob.iter()
        .cloned()
        .zip(y_true.iter().cloned())
        .collect();

    // sort by probability descending
    pairs.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());

    let mut tp = 0.0;
    let mut fp = 0.0;
    let pos = y_true.iter().filter(|&&x| x == 1.0).count() as f64;
    let neg = y_true.len() as f64 - pos;

    let mut prev_fp = 0.0;
    let mut prev_tp = 0.0;
    let mut auc = 0.0;

    for (_, label) in pairs {
        if label == 1.0 {
            tp += 1.0;
        } else {
            fp += 1.0;
            auc += (fp - prev_fp) * (tp + prev_tp) / 2.0;
        }
        prev_fp = fp;
        prev_tp = tp;
    }

    auc / (pos * neg + 1e-12)
}
