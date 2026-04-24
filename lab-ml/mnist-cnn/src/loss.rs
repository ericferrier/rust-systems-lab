use ndarray::Array2;

pub fn softmax(x: &Array2<f32>) -> Array2<f32> {
    let mut out = x.clone();

    for mut row in out.outer_iter_mut() {
        let max = row.iter().cloned().fold(f32::MIN, f32::max);

        let mut sum = 0.0;
        for v in row.iter_mut() {
            *v = (*v - max).exp();
            sum += *v;
        }

        for v in row.iter_mut() {
            *v /= sum;
        }
    }

    out
}

pub fn cross_entropy(probs: &Array2<f32>, y: &Array2<f32>) -> f32 {
    let eps = 1e-9;
    let mut loss = 0.0;

    for (p, t) in probs.outer_iter().zip(y.outer_iter()) {
        for (pi, ti) in p.iter().zip(t.iter()) {
            if *ti > 0.0 {
                loss -= ti * (pi + eps).ln();
            }
        }
    }

    loss / probs.nrows() as f32
}