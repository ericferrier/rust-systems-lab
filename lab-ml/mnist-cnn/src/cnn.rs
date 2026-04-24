use ndarray::{Array1, Array2, Array3, Array4, Axis, s};

pub struct Conv2D {
    pub weight: Array4<f32>, // (out_channels, in_channels, kernel_h, kernel_w)
    pub bias: Array1<f32>,   // (out_channels)
    pub stride: usize,
}

impl Conv2D {
    pub fn new(in_channels: usize, out_channels: usize, kernel_size: usize, stride: usize) -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let weight = Array4::from_shape_fn((out_channels, in_channels, kernel_size, kernel_size), |_| rng.gen::<f32>() * 0.05);
        let bias = Array1::zeros(out_channels);
        Self { weight, bias, stride }
    }

    // Fast im2col + matmul Conv2D (single image)
    pub fn forward(&self, x: &Array3<f32>) -> Array3<f32> {
        // x: (in_channels, H, W)
        let (in_channels, h, w) = (x.shape()[0], x.shape()[1], x.shape()[2]);
        let out_channels = self.weight.shape()[0];
        let kernel = self.weight.shape()[2];
        let stride = self.stride;
        let oh = (h - kernel) / stride + 1;
        let ow = (w - kernel) / stride + 1;

        // im2col: (oh*ow, in_channels*kernel*kernel)
        let mut cols = Array2::<f32>::zeros((oh * ow, in_channels * kernel * kernel));
        for i in 0..oh {
            for j in 0..ow {
                let mut col = Vec::with_capacity(in_channels * kernel * kernel);
                for ic in 0..in_channels {
                    for ki in 0..kernel {
                        for kj in 0..kernel {
                            let xi = i * stride + ki;
                            let xj = j * stride + kj;
                            col.push(x[[ic, xi, xj]]);
                        }
                    }
                }
                let idx = i * ow + j;
                cols.row_mut(idx).assign(&Array1::from(col));
            }
        }

        // Reshape weights: (out_channels, in_channels*kernel*kernel)
        let weight_view = self.weight.view();
        let w_col = weight_view.to_shape((out_channels, in_channels * kernel * kernel)).unwrap();

        // Output: (oh*ow, out_channels)
        let out2d = cols.dot(&w_col.t());

        // Add bias and reshape to (out_channels, oh, ow)
        let mut out = Array3::<f32>::zeros((out_channels, oh, ow));
        for oc in 0..out_channels {
            for idx in 0..(oh * ow) {
                let i = idx / ow;
                let j = idx % ow;
                out[[oc, i, j]] = out2d[[idx, oc]] + self.bias[oc];
            }
        }
        out
    }
}

pub fn relu(x: &Array3<f32>) -> Array3<f32> {
    x.mapv(|v| v.max(0.0))
}

pub fn maxpool2d(x: &Array3<f32>, pool: usize) -> Array3<f32> {
    let (_c, _h, _w) = (x.shape()[0], x.shape()[1], x.shape()[2]);
    let out_h = x.shape()[1] / pool;
    let out_w = x.shape()[2] / pool;
    let mut out = Array3::zeros((x.shape()[0], out_h, out_w));
    for ch in 0..x.shape()[0] {
        for i in 0..out_h {
            for j in 0..out_w {
                let patch = x.slice(s![ch, i*pool..(i+1)*pool, j*pool..(j+1)*pool]);
                out[[ch, i, j]] = patch.iter().cloned().fold(f32::MIN, f32::max);
            }
        }
    }
    out
}

pub fn flatten(x: &Array3<f32>) -> Array2<f32> {
    let _c = x.shape()[0];
    let _h = x.shape()[1];
    let _w = x.shape()[2];
    let flat: Array1<f32> = Array1::from_iter(x.iter().cloned());
    flat.insert_axis(Axis(0))
}
