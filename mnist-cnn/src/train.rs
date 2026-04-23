use ndarray::{Array2, Axis};
use crate::model::LinearModel;
use crate::loss::{softmax, cross_entropy};

pub fn train(model: &mut LinearModel, x: &Array2<f32>, y: &Array2<f32>, epochs: usize, lr: f32) {
    let n = x.nrows() as f32;

    for epoch in 0..epochs {
        let logits = model.forward(x);
        let probs = softmax(&logits);
        let loss = cross_entropy(&probs, y);

        // Gradient of softmax + cross-entropy combined: (probs - y) / n
        let d_logits = (&probs - y) / n;

        // Gradient w.r.t. weights: X^T @ d_logits
        let d_w = x.t().dot(&d_logits);

        // Gradient w.r.t. bias: sum over samples
        let d_b = d_logits.sum_axis(Axis(0));

        model.w -= &(lr * &d_w);
        model.b -= &(lr * &d_b.insert_axis(Axis(0)));

        println!("Epoch {}/{} - loss: {:.4}", epoch + 1, epochs, loss);
    }
}

pub fn accuracy(probs: &Array2<f32>, labels: &[usize]) -> f32 {
    let mut correct = 0;

    for (i, row) in probs.outer_iter().enumerate() {
        let pred = row
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .unwrap()
            .0;

        if pred == labels[i] {
            correct += 1;
        }
    }

    correct as f32 / labels.len() as f32
}