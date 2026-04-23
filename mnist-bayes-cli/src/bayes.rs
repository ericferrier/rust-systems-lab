pub fn gaussian(x: f64, mean: f64, std: f64) -> f64 {
    let eps = 1e-6;
    let std = std + eps;

    let exponent = -((x - mean).powi(2)) / (2.0 * std.powi(2));
    (1.0 / (std * (2.0 * std::f64::consts::PI).sqrt())) * exponent.exp()
}

use std::collections::HashMap;
use crate::model::{ClassModel, NaiveBayes, Stats};

pub fn train(data: &[(Vec<f64>, usize)]) -> NaiveBayes {
    let mut grouped: HashMap<usize, Vec<Vec<f64>>> = HashMap::new();

    for (x, y) in data {
        grouped.entry(*y).or_default().push(x.clone());
    }

    let total = data.len() as f64;
    let mut classes = Vec::new();

    for (label, samples) in grouped {
        let prior = samples.len() as f64 / total;

        let mut features = Vec::new();

        let dim = samples[0].len();

        for i in 0..dim {
            let vals: Vec<f64> = samples.iter().map(|s| s[i]).collect();

            let mean = vals.iter().sum::<f64>() / vals.len() as f64;

            let var = vals.iter()
                .map(|v| (v - mean).powi(2))
                .sum::<f64>() / vals.len() as f64;

            features.push(Stats {
                mean,
                std: var.sqrt(),
            });
        }

        classes.push(ClassModel {
            label,
            prior,
            features,
        });
    }

    NaiveBayes { classes }
}

pub fn predict(model: &NaiveBayes, x: &[f64]) -> usize {
    let mut best_label = 0;
    let mut best_score = f64::MIN;

    for class in &model.classes {
        let mut score = class.prior.ln();

        for i in 0..x.len() {
            let f = &class.features[i];
            score += gaussian(x[i], f.mean, f.std).ln();
        }

        if score > best_score {
            best_score = score;
            best_label = class.label;
        }
    }

    best_label
}