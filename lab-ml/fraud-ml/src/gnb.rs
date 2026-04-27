use ndarray::Array2;

pub struct GaussianNB {
    mean0: Vec<f64>,
    var0: Vec<f64>,
    mean1: Vec<f64>,
    var1: Vec<f64>,
    prior1: f64,
}

impl GaussianNB {
    pub fn new() -> Self {
        Self {
            mean0: vec![],
            var0: vec![],
            mean1: vec![],
            var1: vec![],
            prior1: 0.0,
        }
    }

    pub fn fit(&mut self, x: &Array2<f64>, y: &ndarray::Array1<f64>) {
        let cols = x.ncols();

        self.mean0 = vec![0.0; cols];
        self.mean1 = vec![0.0; cols];
        self.var0 = vec![1.0; cols];
        self.var1 = vec![1.0; cols];

        let mut c0 = 0.0;
        let mut c1 = 0.0;

        for i in 0..x.nrows() {
            if y[i] == 0.0 {
                c0 += 1.0;
                for j in 0..cols {
                    self.mean0[j] += x[[i, j]];
                }
            } else {
                c1 += 1.0;
                for j in 0..cols {
                    self.mean1[j] += x[[i, j]];
                }
            }
        }

        self.prior1 = c1 / (c0 + c1);

        for j in 0..cols {
            self.mean0[j] /= c0;
            self.mean1[j] /= c1;
        }
    }

    pub fn predict_proba(&self, x: &Array2<f64>) -> Vec<f64> {
        x.outer_iter()
            .map(|row| {
                let mut p0 = 1.0;
                let mut p1 = 1.0;

                for j in 0..row.len() {
                    let v = row[j];

                    p0 *= gaussian(v, self.mean0[j], self.var0[j]);
                    p1 *= gaussian(v, self.mean1[j], self.var1[j]);
                }

                p1 / (p0 + p1 + 1e-9)
            })
            .collect()
    }
}

fn gaussian(x: f64, mean: f64, var: f64) -> f64 {
    let eps = 1e-9;
    let denom = (2.0 * std::f64::consts::PI * var + eps).sqrt();
    let num = (-((x - mean).powi(2)) / (2.0 * var + eps)).exp();

    num / denom
}