use ndarray::{Array2, Array1};
use csv::ReaderBuilder;

pub fn load_csv(path: &str) -> (Array2<f64>, Array1<f64>) {
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_path(path)
        .unwrap();

    let mut x = Vec::new();
    let mut y = Vec::new();

    for result in rdr.records() {
        let record = result.unwrap();

        let label: f64 = record[0].parse().unwrap();
        y.push(label);

        for i in 1..record.len() {
            let val: f64 = record[i].parse().unwrap();
            x.push(val / 255.0);
        }
    }

    let rows = y.len();
    let cols = x.len() / rows;

    (
        Array2::from_shape_vec((rows, cols), x).unwrap(),
        Array1::from_vec(y),
    )
}