mod dataset;
mod feature;
mod model;
mod bayes;

use std::{env, process};

use dataset::load_csv;
use feature::extract_features;
use bayes::{train, predict};

fn parse_paths_from_args() -> (String, String) {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => (
            "data/mnist_train.csv".to_string(),
            "data/mnist_test.csv".to_string(),
        ),
        3 => (args[1].clone(), args[2].clone()),
        _ => {
            eprintln!("Usage: cargo run -- <train_csv> <test_csv>");
            process::exit(1);
        }
    }
}

fn build_binary_feature_data(rows: &[(Vec<f64>, usize)]) -> Vec<(Vec<f64>, usize)> {
    rows.iter()
        .filter(|(_, label)| *label == 0 || *label == 1)
        .map(|(pixels, label)| (extract_features(pixels), *label))
        .collect()
}

fn mean(values: &[f64]) -> f64 {
    values.iter().sum::<f64>() / values.len() as f64
}

fn sample_variance(values: &[f64]) -> f64 {
    if values.len() < 2 {
        return 0.0;
    }

    let avg = mean(values);
    values
        .iter()
        .map(|v| {
            let d = v - avg;
            d * d
        })
        .sum::<f64>()
        / (values.len() - 1) as f64
}

fn class_accuracy(test_data: &[(Vec<f64>, usize)], predictions: &[usize], class_label: usize) -> Option<f64> {
    let mut total = 0usize;
    let mut correct = 0usize;

    for ((_, label), pred) in test_data.iter().zip(predictions.iter()) {
        if *label == class_label {
            total += 1;
            if *pred == *label {
                correct += 1;
            }
        }
    }

    if total == 0 {
        None
    } else {
        Some(correct as f64 / total as f64 * 100.0)
    }
}

fn confusion_matrix_01(test_labels: &[usize], predictions: &[usize]) -> [[usize; 2]; 2] {
    let mut cm = [[0usize; 2]; 2];

    for (&actual, &pred) in test_labels.iter().zip(predictions.iter()) {
        if actual <= 1 && pred <= 1 {
            cm[actual][pred] += 1;
        }
    }

    cm
}

fn pixel_to_ascii(v: f64) -> char {
    let ramp = [' ', '.', ':', '-', '=', '+', '*', '#', '%', '@'];
    let clamped = v.clamp(0.0, 1.0);
    let idx = (clamped * (ramp.len() as f64 - 1.0)).round() as usize;
    ramp[idx]
}

fn print_digit_ascii(pixels: &[f64], width: usize, height: usize) {
    if pixels.len() < width * height {
        println!("[preview unavailable: pixel vector too short]");
        return;
    }

    for row in 0..height {
        let start = row * width;
        let end = start + width;
        let line: String = pixels[start..end].iter().map(|&v| pixel_to_ascii(v)).collect();
        println!("{}", line);
    }
}

fn main() {
    let (train_path, test_path) = parse_paths_from_args();

    let train_raw = load_csv(&train_path).unwrap_or_else(|err| {
        eprintln!("Failed to load train CSV '{}': {}", train_path, err);
        process::exit(1);
    });

    let test_raw = load_csv(&test_path).unwrap_or_else(|err| {
        eprintln!("Failed to load test CSV '{}': {}", test_path, err);
        process::exit(1);
    });

    let train_data = build_binary_feature_data(&train_raw);
    let test_data = build_binary_feature_data(&test_raw);
    let test_raw_binary: Vec<(Vec<f64>, usize)> = test_raw
        .iter()
        .filter(|(_, label)| *label == 0 || *label == 1)
        .map(|(pixels, label)| (pixels.clone(), *label))
        .collect();

    if train_data.is_empty() || test_data.is_empty() {
        eprintln!("No usable rows for digits 0/1 were found in the provided datasets.");
        process::exit(1);
    }

    let model = train(&train_data);

    let train0: Vec<&Vec<f64>> = train_data
        .iter()
        .filter(|(_, label)| *label == 0)
        .map(|(features, _)| features)
        .collect();

    let train1: Vec<&Vec<f64>> = train_data
        .iter()
        .filter(|(_, label)| *label == 1)
        .map(|(features, _)| features)
        .collect();

    let digit0_feature1: Vec<f64> = train0.iter().map(|f| f[0]).collect();
    let digit0_feature2: Vec<f64> = train0.iter().map(|f| f[1]).collect();
    let digit1_feature1: Vec<f64> = train1.iter().map(|f| f[0]).collect();
    let digit1_feature2: Vec<f64> = train1.iter().map(|f| f[1]).collect();
    let predictions: Vec<usize> = test_data
        .iter()
        .map(|(features, _)| predict(&model, features))
        .collect();

    let test_labels: Vec<usize> = test_data.iter().map(|(_, label)| *label).collect();

    let correct = test_data
        .iter()
        .zip(predictions.iter())
        .filter(|((_, label), pred)| *label == **pred)
        .count();

    let class0 = class_accuracy(&test_data, &predictions, 0).unwrap_or(0.0);
    let class1 = class_accuracy(&test_data, &predictions, 1).unwrap_or(0.0);

    let accuracy_test0 = class0 / 100.0;
    let accuracy_test1 = class1 / 100.0;

    println!(
        "Mean_of_feature1_for_digit0: {}",
        mean(&digit0_feature1)
    );
    println!(
        "Variance_of_feature1_for_digit0: {}",
        sample_variance(&digit0_feature1)
    );
    println!();
    println!(
        "Mean_of_feature2_for_digit0: {}",
        mean(&digit0_feature2)
    );
    println!(
        "Variance_of_feature2_for_digit0: {}",
        sample_variance(&digit0_feature2)
    );
    println!();
    println!(
        "Mean_of_feature1_for_digit1: {}",
        mean(&digit1_feature1)
    );
    println!(
        "Variance_of_feature1_for_digit1: {}",
        sample_variance(&digit1_feature1)
    );
    println!();
    println!(
        "Mean_of_feature2_for_digit1: {}",
        mean(&digit1_feature2)
    );
    println!(
        "Variance_of_feature2_for_digit1: {}",
        sample_variance(&digit1_feature2)
    );
    println!();
    println!("accuracy test 0 : {}", accuracy_test0);
    println!("accuracy test 1 : {}", accuracy_test1);

    println!("accuracy percentage using test data for class: 0,1");
    println!("({:.2}, {:.2})", class0, class1);

    let cm = confusion_matrix_01(&test_labels, &predictions);
    println!();
    println!("Confusion matrix (rows=actual, cols=predicted)");
    println!("          pred 0    pred 1");
    println!("actual 0   {:>6}    {:>6}", cm[0][0], cm[0][1]);
    println!("actual 1   {:>6}    {:>6}", cm[1][0], cm[1][1]);

    println!();
    println!("Digit visualization preview (ASCII, 28x28)");

    if let Some((pixels0, _)) = test_raw_binary.iter().find(|(_, label)| *label == 0) {
        println!("Sample digit 0:");
        print_digit_ascii(pixels0, 28, 28);
        println!();
    }

    if let Some((pixels1, _)) = test_raw_binary.iter().find(|(_, label)| *label == 1) {
        println!("Sample digit 1:");
        print_digit_ascii(pixels1, 28, 28);
        println!();
    }

    println!(
        "Accuracy: {:.2}%",
        correct as f64 / test_data.len() as f64 * 100.0
    );
}