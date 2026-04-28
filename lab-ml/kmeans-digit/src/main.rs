mod utils;
mod kmeans;
mod metrics;

use std::env;
use utils::load_csv;
use kmeans::{KMeans, Distance};
use crate::metrics::confusion_matrix;

mod elbow;
use elbow::find_best_k;



fn parse_distance(s: &str) -> Distance {
    match s {
        "manhattan" => Distance::Manhattan,
        "sqeuclidean" => Distance::SquaredEuclidean,
        _ => Distance::Euclidean,
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage:");
        println!("cargo run -- data/train.csv --k 10 --dist euclidean");
        return;
    }

    let mut auto_k = false;
    for i in 0..args.len() {
        if args[i] == "--auto-k" {
            auto_k = true;
        }
    }

    let path = &args[1];
    let mut k = 10;
    let mut dist = Distance::Euclidean;
    for i in 0..args.len() {
        if args[i] == "--k" {
            k = args[i + 1].parse().unwrap();
        }
        if args[i] == "--dist" {
            dist = parse_distance(&args[i + 1]);
        }
    }

    println!("Loading dataset...");
    let (x, y) = load_csv(path);

    if auto_k {
        let (best_k, inertias) = find_best_k(&x, 12, 10, 512, dist);
        println!("\nSuggested optimal k: {}", best_k);
        println!("\nElbow curve:");
        for (i, val) in inertias.iter().enumerate() {
            println!("k={} → {:.4}", i + 2, val);
        }
        println!("\nTraining final model with k={}...", best_k);
        let mut model = KMeans::new(best_k, dist);
        model.fit_minibatch(&x, 20, 512);
        let preds = model.predict(&x);
        let matrix = confusion_matrix(y.as_slice().unwrap(), &preds);
        print_confusion(&matrix);
    } else {
        println!("Training KMeans...");
        let mut model = KMeans::new(k, dist);
        model.fit_minibatch(&x, 20, 512);
        println!("\nInertia history:");
        for (i, val) in model.inertia_history.iter().enumerate() {
            println!("epoch {} → {:.4}", i + 1, val);
        }
        let preds = model.predict(&x);
        let matrix = confusion_matrix(y.as_slice().unwrap(), &preds);
        print_confusion(&matrix);
    }
}


pub fn print_confusion(matrix: &Vec<Vec<usize>>) {
    let size = matrix.len();
    println!("\n--- Confusion Matrix ---");

    print!("    ");
    for i in 0..size {
        print!("{:4}", i);
    }
    println!();

    for i in 0..size {
        print!("{:2} |", i);
        for j in 0..size {
            print!("{:4}", matrix[i][j]);
        }
        println!();
    }
}