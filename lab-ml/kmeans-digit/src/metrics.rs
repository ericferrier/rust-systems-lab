pub fn confusion_matrix(y_true: &[f64], y_pred: &[f64]) -> Vec<Vec<usize>> {
    let max_label = y_true.iter().chain(y_pred.iter())
        .map(|&v| v as usize)
        .max().unwrap_or(0);
    let size = max_label + 1;
    let mut matrix = vec![vec![0usize; size]; size];

    for i in 0..y_true.len() {
        let true_label = y_true[i] as usize;
        let pred_label = y_pred[i] as usize;
        matrix[true_label][pred_label] += 1;
    }

    matrix
}