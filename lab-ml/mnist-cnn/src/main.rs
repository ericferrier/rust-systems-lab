mod utils;
mod loss;
mod model;
mod train;
mod data;

use std::env;

use utils::one_hot;
mod cnn;
use model::CNNModel;
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

    // CNN: 1 input channel, 8 output channels, 3x3 kernel, 2x2 pool
    let mut model = CNNModel::new(1, 1, 3, 2, num_classes);

    let epochs = 5;
    let lr = 0.1;
    let batch_size = 8;

    println!("Training...");
    train(&mut model, &x_train, &y_train, epochs, lr, batch_size);

    let logits = model.forward(&x_test);
    let probs = loss::softmax(&logits);
    let acc = train::accuracy(&probs, &y_test_labels);

    println!("Test accuracy: {:.2}%", acc * 100.0);
    println!("Done.");
}