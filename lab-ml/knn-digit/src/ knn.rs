use rayon::prelude::*;

impl KNN {
    fn distance(a: &[f64], b: &[f64]) -> f64 {
        a.iter()
            .zip(b.iter())
            .map(|(x, y)| (x - y) * (x - y))
            .sum::<f64>()
            .sqrt()
    }

    pub fn predict(&self, x: &[f64]) -> f64 {
        let mut distances: Vec<(f64, f64)> = self
            .x_train
            .outer_iter()
            .enumerate()
            .par_bridge() // 🔥 parallel iterator
            .map(|(i, row)| {
                let d = Self::distance(&row.to_vec(), x);
                (d, self.y_train[i])
            })
            .collect();

        distances.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        let mut counts = vec![0; 10];

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