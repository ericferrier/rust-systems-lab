mod data;
mod fft_engine;
mod signals;
mod backtest;

use fft_engine::FftEngine;
use backtest::Backtester;
use std::{thread, time::Duration};

fn main() {
    let mut engine = FftEngine::new(64);
    let mut backtester = Backtester::new();

    let mut prev_log_price: Option<f32> = None;
    loop {
        let price = match data::fetch_price() {
            Some(p) => p,
            None => {
                eprintln!("Failed to fetch price. Skipping iteration.");
                thread::sleep(Duration::from_secs(2));
                continue;
            }
        };

        let log_price = price.ln();
        if let Some(prev) = prev_log_price {
            let log_return = log_price - prev;
            engine.push(log_return);
        }
        prev_log_price = Some(log_price);

        let spectrum = engine.compute();

        // Find peak frequency bin and its magnitude
        if let Some((peak_idx, peak_val)) = spectrum.iter().enumerate().max_by(|a, b| a.1.partial_cmp(b.1).unwrap()) {
            println!("Peak frequency bin: {} (magnitude: {:.4})", peak_idx, peak_val);
        }

        let signal = signals::generate_signal(&spectrum);

        backtester.step(price, &signal);

        println!("Price: {:.2}", price);
        println!("Signal: {}", signal);
        println!("Portfolio: {:.2}", backtester.value(price));
        println!("-----------------------------");
        thread::sleep(Duration::from_secs(2));
    }
}