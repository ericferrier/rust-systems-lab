use crate::kmeans::{KMeans, Distance};
use ndarray::Array2;

pub fn find_best_k(
    x: &Array2<f64>,
    max_k: usize,
    epochs: usize,
    batch_size: usize,
    distance: Distance,
) -> (usize, Vec<f64>) {
    let mut inertias = Vec::new();

    println!("Running elbow method...");

    for k in 2..=max_k {
        let mut model = KMeans::new(k, distance);
        model.fit_minibatch(x, epochs, batch_size);

        let final_inertia = *model.inertia_history.last().unwrap();
        println!("k={} → inertia {:.4}", k, final_inertia);

        inertias.push(final_inertia);
    }

    let best_k = detect_elbow(&inertias);

    (best_k + 2, inertias) // +2 because we started at k=2
}

fn detect_elbow(inertias: &[f64]) -> usize {
    let mut best_k = 0;
    let mut max_curvature = 0.0;

    for i in 1..inertias.len() - 1 {
        let prev = inertias[i - 1];
        let curr = inertias[i];
        let next = inertias[i + 1];

        // second derivative approximation
        let curvature = (prev - curr) - (curr - next);

        if curvature > max_curvature {
            max_curvature = curvature;
            best_k = i;
        }
    }

    best_k
}
