use ndarray::{Array2};
use rand::Rng;

pub struct LinearModel {
    pub w: Array2<f32>,
    pub b: Array2<f32>,
}

impl LinearModel {
    pub fn new(input_dim: usize, num_classes: usize) -> Self {
        let mut rng = rand::thread_rng();

        let w = Array2::from_shape_fn((input_dim, num_classes), |_| {
            rng.gen::<f32>() * 0.01
        });

        let b = Array2::<f32>::zeros((1, num_classes));

        Self { w, b }
    }

    pub fn forward(&self, x: &Array2<f32>) -> Array2<f32> {
        x.dot(&self.w) + &self.b
    }
}