use ndarray::Array2;

pub fn one_hot(labels: &[usize], num_classes: usize) -> Array2<f32> {
    let mut y = Array2::<f32>::zeros((labels.len(), num_classes));

    for (i, &label) in labels.iter().enumerate() {
        y[[i, label]] = 1.0;
    }

    y
}