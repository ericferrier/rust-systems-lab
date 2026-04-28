use ndarray::{Array2};
use rayon::prelude::*;

pub struct KNN {
    pub k: usize,
    pub x_train: Array2<f64>,
    pub y_train: Vec<f64>,
}

impl KNN {
    pub fn new(k: usize, x_train: Array2<f64>, y_train: Vec<f64>) -> Self {
        Self { k, x_train, y_train }
    }

    fn distance(a: &[f64], b: &[f64]) -> f64 {
        a.iter()
            .zip(b.iter())
            .map(|(x, y)| (x - y) * (x - y))
            .sum::<f64>()
            .sqrt()
    }

    pub fn predict(&self, x: &[f64]) -> f64 {
        // Parallel distance computation
        let mut distances: Vec<(f64, f64)> = self
            .x_train
            .outer_iter()
            .enumerate()
            .par_bridge()
            .map(|(i, row)| {
                let row_vec = row.to_vec();
                let d = Self::distance(&row_vec, x);
                (d, self.y_train[i])
            })
            .collect();

        // sort by distance
        distances.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        // majority vote
        let mut counts = vec![0usize; 10];

        for i in 0..self.k {
            let label = distances[i].1 as usize;
            counts[label] += 1;
        }

        counts
            .iter()
            .enumerate()
            .max_by_key(|(_, &c)| c)
            .unwrap()
            .0 as f64
    }
}