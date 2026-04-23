mod utils;
mod loss;
mod model;
mod train;
mod data;

use std::env;

use utils::one_hot;
use model::LinearModel;
use train::train;
use data::load_mnist_csv;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage:");
        println!("cargo run -- <train.csv> <test.csv>");
        return;
    }

    let train_path = &args[1];
    let test_path = &args[2];

    println!("Loading training data...");
    let (x_train, y_train_labels) = load_mnist_csv(train_path);

    println!("Loading test data...");
    let (x_test, y_test_labels) = load_mnist_csv(test_path);

    let num_classes = 10;

    let y_train = one_hot(&y_train_labels, num_classes);
    let _y_test = one_hot(&y_test_labels, num_classes);

    let input_dim = x_train.shape()[1];

    let mut model = LinearModel::new(input_dim, num_classes);

    println!("Training...");
    train(&mut model, &x_train, &y_train, 20, 0.1);

    let logits = model.forward(&x_test);
    let probs = loss::softmax(&logits);
    let acc = train::accuracy(&probs, &y_test_labels);

    println!("Test accuracy: {:.2}%", acc * 100.0);
    println!("Done.");
}