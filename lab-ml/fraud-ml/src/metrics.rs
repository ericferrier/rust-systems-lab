use crate::roc::compute_auc;


pub fn evaluate(
    y_true: &[f64],
    y_pred: &[f64],
    y_prob: &[f64],
) {
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

    let precision = tp / (tp + fp + 1e-12);
    let recall = tp / (tp + fn_ + 1e-12);
    let f1 = 2.0 * (precision * recall) / (precision + recall + 1e-12);

    let accuracy = (tp + tn) / (tp + tn + fp + fn_ + 1e-12);

    println!("--- CONFUSION MATRIX ---");
    println!("TP: {:.0} FP: {:.0}", tp, fp);
    println!("FN: {:.0} TN: {:.0}", fn_, tn);

    println!("\n--- FRAUD METRICS (IMPORTANT) ---");
    println!("Accuracy:  {:.4} (IGNORE THIS)", accuracy);
    println!("Precision: {:.4}", precision);
    println!("Recall:    {:.4}  ← FRAUD DETECTION RATE", recall);
    println!("F1 Score:  {:.4}", f1);

    // ROC-AUC placeholder call (if you already implemented roc.rs)
    let auc = compute_auc(y_true, y_prob);
    println!("ROC-AUC:   {:.4}", auc);
}

