use ndarray::Array2;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn load_mnist_csv(path: &str) -> (Array2<f32>, Vec<usize>) {
    let file = File::open(path).expect("Cannot open file");
    let reader = BufReader::new(file);

    let mut data: Vec<f32> = Vec::new();
    let mut labels: Vec<usize> = Vec::new();

    let mut rows = 0;
    let mut lines = reader.lines();

    // Skip header row
    lines.next();

    for line in lines {
        let line = line.unwrap();
        let mut parts = line.split(',');

        // First value = label
        let label: usize = parts.next().unwrap().parse().unwrap();
        labels.push(label);

        // Remaining = pixels
        for p in parts {
            let val: f32 = p.parse::<f32>().unwrap() / 255.0; // normalize
            data.push(val);
        }

        rows += 1;
    }

    let cols = data.len() / rows;

    let x = Array2::from_shape_vec((rows, cols), data).unwrap();

    (x, labels)
}