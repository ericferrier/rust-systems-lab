## S&P 500 Futures Tick Data (SP)

Data Source: [Kaggle - S&P 500 Futures Tick Data](https://www.kaggle.com/datasets/finnhub/sp-500-futures-tick-data-sp)

### CSV Format
Each row: `date,time,price,volume`
Example:
```
2020-01-02,09:30:00,3250.25,10
```

---

## Program Instructions

This project reads S&P 500 futures tick data from `data/SP.csv` and performs real-time frequency analysis on price movements using FFT (Fast Fourier Transform).

**What the program does:**
- Reads prices sequentially from the CSV file.
- Computes log-returns and maintains a sliding window for FFT.
- Runs FFT to detect dominant frequencies in price changes.
- Prints the peak frequency bin and its magnitude.
- Generates trading signals:
	- `LONG` (mid-cycle momentum)
	- `SHORT` (high frequency/noise)
	- `HOLD` (low frequency trend)
- Simulates a simple backtest portfolio based on these signals.

**How to run:**
```
cargo run
```

**Requirements:**
- Rust
- `data/SP.csv` file in the correct format

---
For more details on the data, see the Kaggle link above.

