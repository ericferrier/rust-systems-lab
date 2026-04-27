use ndarray::{Array2, Array1};
use csv::ReaderBuilder;

pub fn load_dataset(path: &str) -> (Array2<f64>, Array1<f64>, Array2<f64>, Array1<f64>) {
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_path(path)
        .unwrap();

    let mut x = Vec::new();
    let mut y = Vec::new();

    for result in rdr.records() {
        let record = result.unwrap();

        let mut row = Vec::new();

        for i in 0..30 {
            let v: f64 = record[i].parse().unwrap();
            row.push(v);
        }

        x.push(row);
        y.push(record[30].parse::<f64>().unwrap());
    }

    let split = (x.len() as f64 * 0.8) as usize;

    let x_train = Array2::from_shape_vec(
        (split, 30),
        x[..split].concat()
    ).unwrap();

    let y_train = Array1::from_vec(y[..split].to_vec());

    let x_test = Array2::from_shape_vec(
        (x.len() - split, 30),
        x[split..].concat()
    ).unwrap();

    let y_test = Array1::from_vec(y[split..].to_vec());

    (x_train, y_train, x_test, y_test)
}