use ndarray::{Array2};
use crate::cnn::{Conv2D, relu, maxpool2d, flatten};

pub struct CNNModel {
    pub conv: Conv2D,
    pub w: Array2<f32>,
    pub b: Array2<f32>,
}

impl CNNModel {
    pub fn new(in_channels: usize, out_channels: usize, kernel: usize, _pool: usize, num_classes: usize) -> Self {
        use rand::Rng;
        let conv = Conv2D::new(in_channels, out_channels, kernel, 1);
        // After conv (no padding): (28 - kernel + 1)
        // After Conv2D (kernel=3, stride=1): (28-3+1)=26, after MaxPool(2): 13
        let flat_dim = out_channels * 13 * 13;
        // For kernel=3, pool=2, out_channels=1: flat_dim = 1*13*13 = 169
        let mut rng = rand::thread_rng();
        let w = Array2::from_shape_fn((flat_dim, num_classes), |_| rng.gen::<f32>() * 0.01);
        let b = Array2::<f32>::zeros((1, num_classes));
        Self { conv, w, b }
    }

    pub fn forward(&self, x: &Array2<f32>) -> Array2<f32> {
        // x: (batch, 28*28)
        let batch = x.nrows();
        let mut out = Array2::zeros((batch, self.w.nrows()));
        for i in 0..batch {
            let img = x.row(i).to_owned().to_shape((1,28,28)).unwrap().to_owned();
            let feat = self.conv.forward(&img);
            let feat = relu(&feat);
            let feat = maxpool2d(&feat, 2);
            let flat = flatten(&feat);
            out.row_mut(i).assign(&flat.row(0));
        }
        out.dot(&self.w) + &self.b
    }
}