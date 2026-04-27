use ndarray::{Array2, Array1};

pub struct LogisticRegression {
    weights: Vec<f64>,
    bias: f64,
}

impl LogisticRegression {
    pub fn new(dim: usize) -> Self {
        Self {
            weights: vec![0.0; dim],
            bias: 0.0,
        }
    }

    pub fn fit(&mut self, x: &Array2<f64>, y: &Array1<f64>, lr: f64, epochs: usize, class_weight: f64) {
        for _ in 0..epochs {
            for i in 0..x.nrows() {
                let row = x.row(i);

                let z: f64 = row
                    .iter()
                    .zip(&self.weights)
                    .map(|(xj, wj)| xj * wj)
                    .sum::<f64>() + self.bias;

                let pred = sigmoid(z);
                let error = pred - y[i];

                // Apply class weighting
                let weight = if y[i] == 1.0 { class_weight } else { 1.0 };

                for j in 0..self.weights.len() {
                    self.weights[j] -= lr * error * row[j] * weight;
                }

                self.bias -= lr * error * weight;
            }
        }
    }

    pub fn predict_proba(&self, x: &Array2<f64>) -> Vec<f64> {
        x.outer_iter()
            .map(|row| {
                let z: f64 = row
                    .iter()
                    .zip(&self.weights)
                    .map(|(xj, wj)| xj * wj)
                    .sum::<f64>() + self.bias;

                sigmoid(z)
            })
            .collect()
    }
}

fn sigmoid(z: f64) -> f64 {
    1.0 / (1.0 + (-z).exp())
}