use rustfft::{FftPlanner, num_complex::Complex};

pub struct FftEngine {
    buffer: Vec<f32>,
    window: usize,
    planner: FftPlanner<f32>,
}

impl FftEngine {
    pub fn new(window: usize) -> Self {
        Self {
            buffer: vec![0.0; window],
            window,
            planner: FftPlanner::new(),
        }
    }

    /// Push new log-return value (sliding window)
    pub fn push(&mut self, value: f32) {
        // shift-left is still simple but fine for small windows (64–256)
        self.buffer.rotate_left(1);
        self.buffer[self.window - 1] = value;
    }

    /// Optional: normalize signal (VERY important for finance FFT)
    fn normalize(&self) -> Vec<f32> {
        let mean: f32 = self.buffer.iter().sum::<f32>() / self.window as f32;

        let variance: f32 = self
            .buffer
            .iter()
            .map(|x| (x - mean).powi(2))
            .sum::<f32>()
            / self.window as f32;

        let std = variance.sqrt().max(1e-8);

        self.buffer.iter().map(|x| (x - mean) / std).collect()
    }

    /// FFT computation (returns magnitude spectrum)
    pub fn compute(&mut self) -> Vec<f32> {
        let fft = self.planner.plan_fft_forward(self.window);

        let signal = self.normalize();

        let mut input: Vec<Complex<f32>> = signal
            .into_iter()
            .map(|v| Complex::new(v, 0.0))
            .collect();

        fft.process(&mut input);

        // magnitude spectrum
        input.iter().map(|c| c.norm()).collect()
    }
}