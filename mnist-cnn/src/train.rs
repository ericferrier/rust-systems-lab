use ndarray::{Array2, Axis};
use rand::seq::SliceRandom;

use crate::loss::{cross_entropy, softmax};
use crate::model::LinearModel;

pub fn train(
    model: &mut LinearModel,
    x: &Array2<f32>,
    y: &Array2<f32>,
    epochs: usize,
    lr: f32,
    batch_size: usize,
) {
    let n = x.nrows();
    let mut rng = rand::thread_rng();
    let mut indices: Vec<usize> = (0..n).collect();

    for epoch in 0..epochs {
        indices.shuffle(&mut rng);

        let mut epoch_loss_sum = 0.0f32;

        for batch_start in (0..n).step_by(batch_size) {
            let batch_end = (batch_start + batch_size).min(n);
            let batch_indices = &indices[batch_start..batch_end];

            let x_batch = x.select(Axis(0), batch_indices);
            let y_batch = y.select(Axis(0), batch_indices);

            let logits = model.forward(&x_batch);
            let probs = softmax(&logits);
            let loss = cross_entropy(&probs, &y_batch);

            let batch_n = x_batch.nrows() as f32;
            // Keep exactly one normalization by batch size.
            let d_logits = (&probs - &y_batch) / batch_n;
            let d_w = x_batch.t().dot(&d_logits);
            let d_b = d_logits.sum_axis(Axis(0));

            model.w -= &(lr * &d_w);
            model.b -= &(lr * &d_b.insert_axis(Axis(0)));

            epoch_loss_sum += loss * batch_n;
        }

        let epoch_loss = epoch_loss_sum / n as f32;
        println!("Epoch {}/{} - loss: {:.4}", epoch + 1, epochs, epoch_loss);
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