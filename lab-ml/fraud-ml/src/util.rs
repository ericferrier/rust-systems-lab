use ndarray::{Array2, Array1};
use rand::seq::SliceRandom;
use rand::thread_rng;

/// Shuffle + split dataset into train/test
pub fn train_test_split(
    x: &Array2<f64>,
    y: &Array1<f64>,
    test_ratio: f64,
) -> (Array2<f64>, Array1<f64>, Array2<f64>, Array1<f64>) {

    let n = x.nrows();
    let mut idx: Vec<usize> = (0..n).collect();

    idx.shuffle(&mut thread_rng());

    let test_size = (n as f64 * test_ratio) as usize;

    let train_idx = &idx[test_size..];
    let test_idx = &idx[..test_size];

    let x_train = select_rows(x, train_idx);
    let y_train = select_labels(y, train_idx);

    let x_test = select_rows(x, test_idx);
    let y_test = select_labels(y, test_idx);

    (x_train, y_train, x_test, y_test)
}

pub fn select_rows(x: &Array2<f64>, idx: &[usize]) -> Array2<f64> {
    let mut out = Vec::new();

    for &i in idx {
        for v in x.row(i).iter() {
            out.push(*v);
        }
    }

    Array2::from_shape_vec((idx.len(), x.ncols()), out).unwrap()
}


pub fn select_labels(y: &Array1<f64>, idx: &[usize]) -> Array1<f64> {
    let mut out = Vec::new();

    for &i in idx {
        out.push(y[i]);
    }

    Array1::from_vec(out)
}


pub fn normalize(x: &Array2<f64>) -> Array2<f64> {
    let mut out = x.clone();

    for j in 0..x.ncols() {
        let col = x.column(j);

        let mean = col.mean().unwrap_or(0.0);
        let std = col.std(0.0);

        for i in 0..x.nrows() {
            let val = x[[i, j]];
            out[[i, j]] = (val - mean) / (std + 1e-9);
        }
    }

    out
}


pub fn threshold(probs: &[f64], t: f64) -> Vec<f64> {
    probs.iter()
        .map(|p| if *p > t { 1.0 } else { 0.0 })
        .collect()
}

pub fn sigmoid(z: f64) -> f64 {
    1.0 / (1.0 + (-z).exp())
}

/// Prevent log(0) issues if you later extend metrics
pub fn safe_log(x: f64) -> f64 {
    (x + 1e-12).ln()
}