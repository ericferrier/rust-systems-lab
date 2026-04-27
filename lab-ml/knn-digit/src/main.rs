mod utils;
mod knn;
mod metrics;

use utils::load_csv;
use knn::KNN;
use metrics::accuracy, confusion_matrix, print_confusion;

fn main() {
    let (x, y) = load_csv("data/train.csv");

    let split = (x.nrows() as f64 * 0.8) as usize;

    let x_train = x.slice(ndarray::s![0..split, ..]).to_owned();
    let y_train = y.slice(ndarray::s![0..split]).to_vec();

    let x_test = x.slice(ndarray::s![split.., ..]).to_owned();
    let y_test = y.slice(ndarray::s![split..]).to_vec();

    let knn = KNN::new(3, x_train, y_train);

    let mut preds = Vec::new();

    for i in 0..x_test.nrows() {
        let row = x_test.row(i).to_vec();
        preds.push(knn.predict(&row));
    }

    let acc = accuracy(&y_test, &preds);

    println!("KNN Accuracy: {:.4}", acc);

    let matrix = confusion_matrix(&y_test, &preds);

    print_confusion(&matrix);
}