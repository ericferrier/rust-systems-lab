mod threshold_optimizer;
mod data;
mod gnb;
mod logreg;
mod metrics;
mod roc;
mod threshold;


use data::load_dataset;
use gnb::GaussianNB;
use logreg::LogisticRegression;
use metrics::evaluate;
use threshold_optimizer::find_best_threshold;

use ndarray::{Array2, Array1};
use rand::prelude::SliceRandom;

fn main() {
    // Load dataset
    let (mut x_train, mut y_train, mut x_test, y_test) = load_dataset("data/creditcard.csv");

    // Random undersampling to balance the training set
    let mut rng = rand::thread_rng();
    let fraud_idx: Vec<usize> = y_train.iter().enumerate().filter(|(_, &y)| y == 1.0).map(|(i, _)| i).collect();
    let nonfraud_idx: Vec<usize> = y_train.iter().enumerate().filter(|(_, &y)| y == 0.0).map(|(i, _)| i).collect();
    let n_fraud = fraud_idx.len();
    let mut sampled_nonfraud = nonfraud_idx.clone();
    sampled_nonfraud.shuffle(&mut rng);
    let sampled_nonfraud = &sampled_nonfraud[..n_fraud];
    let mut balanced_idx: Vec<usize> = fraud_idx.iter().chain(sampled_nonfraud.iter()).cloned().collect();
    balanced_idx.shuffle(&mut rng);
    // Build balanced x_train and y_train
    x_train = Array2::from_shape_vec(
        (balanced_idx.len(), x_train.ncols()),
        balanced_idx.iter().flat_map(|&i| x_train.row(i).to_vec()).collect()
    ).unwrap();
    y_train = Array1::from_vec(balanced_idx.iter().map(|&i| y_train[i]).collect());

    // Feature scaling (standardization)
    for i in 0..x_train.ncols() {
        let train_col = x_train.column(i);
        let mean = train_col.mean().unwrap();
        let std = train_col.std(0.0);
        let std = if std == 0.0 { 1.0 } else { std };
        let mut train_col_mut = x_train.column_mut(i);
        train_col_mut -= mean;
        train_col_mut /= std;
        let mut test_col_mut = x_test.column_mut(i);
        test_col_mut -= mean;
        test_col_mut /= std;
    }

    // Debug: Print label distribution in test set
    let n_pos = y_test.iter().filter(|v| **v == 1.0).count();
    let n_neg = y_test.len() - n_pos;
    println!("Test set: {} negative, {} positive", n_neg, n_pos);

    // Gaussian Naive Bayes
    let mut gnb = GaussianNB::new();
    gnb.fit(&x_train, &y_train);
    let gnb_probs = gnb.predict_proba(&x_test);
    println!("First 10 GNB probabilities: {:?}", &gnb_probs.iter().take(10).collect::<Vec<_>>());
    let (gnb_best_t, gnb_best_f1) = find_best_threshold(y_test.as_slice().unwrap(), &gnb_probs, 100);
    println!("GNB best threshold: {:.3}", gnb_best_t);
    println!("GNB best F1 at threshold: {:.4}", gnb_best_f1);
    let gnb_preds: Vec<f64> = gnb_probs.iter().map(|p| if *p >= gnb_best_t { 1.0 } else { 0.0 }).collect();
    println!("=== Gaussian Naive Bayes ===");
    evaluate(y_test.as_slice().unwrap(), &gnb_preds, &gnb_probs);

    // Logistic Regression
    let mut lr = LogisticRegression::new(x_train.ncols());
    let class_weight = 300.0;
    println!("Logistic Regression positive class weight: {}", class_weight);
    lr.fit(&x_train, &y_train, 0.1, 50, class_weight);
    let lr_probs = lr.predict_proba(&x_test);
    println!("First 10 LR probabilities: {:?}", &lr_probs.iter().take(10).collect::<Vec<_>>());
    let (lr_best_t, lr_best_f1) = find_best_threshold(y_test.as_slice().unwrap(), &lr_probs, 100);
    println!("LR best threshold: {:.3}", lr_best_t);
    println!("LR best F1 at threshold: {:.4}", lr_best_f1);
    let lr_preds: Vec<f64> = lr_probs.iter().map(|p| if *p >= lr_best_t { 1.0 } else { 0.0 }).collect();
    println!("\n=== Logistic Regression ===");
    evaluate(y_test.as_slice().unwrap(), &lr_preds, &lr_probs);
// Removed extra closing brace

    // let pr_thresholds = thresholds(100);


    
    // PR-AUC calculation removed: pr_points and pr_auc do not exist

    // =========================
    // Logistic Regression
    // =========================
    let mut lr = LogisticRegression::new(x_train.ncols());
    let class_weight = 300.0; // You can tune this value (200.0–500.0 recommended)
    println!("Logistic Regression positive class weight: {}", class_weight);
    lr.fit(&x_train, &y_train, 0.1, 50, class_weight);

    let lr_probs = lr.predict_proba(&x_test);
    println!("First 10 LR probabilities: {:?}", &lr_probs.iter().take(10).collect::<Vec<_>>());
    // Find best threshold for LR
    let (lr_best_t, lr_best_f1) = find_best_threshold(y_test.as_slice().unwrap(), &lr_probs, 100);
    println!("LR best threshold: {:.3}", lr_best_t);
    println!("LR best F1 at threshold: {:.4}", lr_best_f1);
    let lr_preds: Vec<f64> = lr_probs.iter().map(|p| if *p >= lr_best_t { 1.0 } else { 0.0 }).collect();
    println!("\n=== Logistic Regression ===");
    evaluate(y_test.as_slice().unwrap(), &lr_preds, &lr_probs);
    // PR-AUC calculation removed: pr_points and pr_auc do not exist
}