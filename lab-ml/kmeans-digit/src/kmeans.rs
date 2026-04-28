use ndarray::{Array2};
use rand::{seq::SliceRandom, thread_rng};
use rayon::prelude::*;

#[derive(Clone, Copy)]
pub enum Distance {
    Euclidean,
    SquaredEuclidean,
    Manhattan,
}

pub struct KMeans {
    pub k: usize,
    pub centroids: Array2<f64>,
    pub distance: Distance,
    pub inertia_history: Vec<f64>,
}

impl KMeans {
    pub fn new(k: usize, distance: Distance) -> Self {
        Self {
            k,
            centroids: Array2::zeros((k, 1)),
            distance,
            inertia_history: Vec::new(),
        }
    }

    fn dist(&self, a: &[f64], b: &[f64]) -> f64 {
        match self.distance {
            Distance::Euclidean => {
                a.iter().zip(b).map(|(x, y)| (x - y).powi(2)).sum::<f64>().sqrt()
            }
            Distance::SquaredEuclidean => {
                a.iter().zip(b).map(|(x, y)| (x - y).powi(2)).sum()
            }
            Distance::Manhattan => {
                a.iter().zip(b).map(|(x, y)| (x - y).abs()).sum()
            }
        }
    }

    pub fn fit_minibatch(&mut self, x: &Array2<f64>, epochs: usize, batch_size: usize) {
        let mut rng = thread_rng();
        let cols = x.ncols();

        // init centroids
        let mut indices: Vec<usize> = (0..x.nrows()).collect();
        indices.shuffle(&mut rng);

        self.centroids = Array2::zeros((self.k, cols));
        for i in 0..self.k {
            self.centroids.row_mut(i).assign(&x.row(indices[i]));
        }

        for ep in 0..epochs {
            let mut batch_idx: Vec<usize> = (0..x.nrows()).collect();
            batch_idx.shuffle(&mut rng);
            let batch_idx = &batch_idx[..batch_size.min(x.nrows())];

            let mut counts = vec![0usize; self.k];

            // -------------------------
            // PARALLEL ASSIGNMENT STEP
            // -------------------------
            let assignments: Vec<(usize, usize)> = batch_idx
                .par_iter()
                .map(|&i| {
                    let row = x.row(i).to_vec();

                    let mut best = 0;
                    let mut best_dist = f64::MAX;

                    for c in 0..self.k {
                        let centroid = self.centroids.row(c).to_vec();
                        let d = self.dist(&row, &centroid);

                        if d < best_dist {
                            best_dist = d;
                            best = c;
                        }
                    }

                    (i, best)
                })
                .collect();

            // -------------------------
            // UPDATE STEP (sequential for safety)
            // -------------------------
            for (i, cluster) in &assignments {
                counts[*cluster] += 1;

                let row = x.row(*i).to_vec();

                for j in 0..cols {
                    let old = self.centroids[[*cluster, j]];
                    self.centroids[[*cluster, j]] =
                        old + (row[j] - old) / counts[*cluster] as f64;
                }
            }

            // -------------------------
            // INERTIA (PARALLEL)
            // -------------------------
            let inertia: f64 = x
                .outer_iter()
                .par_bridge()
                .map(|row| {
                    let row = row.to_vec();

                    let mut best_dist = f64::MAX;

                    for c in 0..self.k {
                        let centroid = self.centroids.row(c).to_vec();
                        let d = self.dist(&row, &centroid);

                        if d < best_dist {
                            best_dist = d;
                        }
                    }

                    best_dist
                })
                .sum();

            self.inertia_history.push(inertia);

            println!("epoch {} → inertia {:.4}", ep + 1, inertia);
        }
    }

    pub fn predict(&self, x: &Array2<f64>) -> Vec<f64> {
        x.outer_iter()
            .map(|row| {
                let row = row.to_vec();

                let mut best = 0;
                let mut best_dist = f64::MAX;

                for c in 0..self.k {
                    let centroid = self.centroids.row(c).to_vec();
                    let d = self.dist(&row, &centroid);

                    if d < best_dist {
                        best_dist = d;
                        best = c;
                    }
                }

                best as f64
            })
            .collect()
    }
}