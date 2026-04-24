pub struct Backtester {
    pub cash: f32,
    pub position: f32,
}

impl Backtester {
    pub fn new() -> Self {
        Self {
            cash: 10_000.0,
            position: 0.0,
        }
    }

    pub fn step(&mut self, price: f32, signal: &str) {
        match signal {
            s if s.contains("BUY") => {
                self.position += self.cash / price;
                self.cash = 0.0;
            }
            s if s.contains("SELL") => {
                self.cash += self.position * price;
                self.position = 0.0;
            }
            _ => {}
        }
    }

    pub fn value(&self, price: f32) -> f32 {
        self.cash + self.position * price
    }
}