# 🦀 Rust Playground

A collection of **practical Rust CLI tools, system utilities, and experimental data projects**.

Built  for:

* fast prototyping
* systems programming practice
* ML / signal processing experiments
* real-world CLI tooling patterns

---

# 🧪 lab-io Systems Lab Backlog

## 🟢 Starter & Intermediate CLI Tools

* [ ] Word frequency counter
* [ ] Character counter CLI
* [ ] Palindrome checker
* [ ] Simple calculator CLI
* [ ] File line counter
* [ ] Log file analyzer
* [ ] JSON parser CLI
* [ ] File organizer tool
* [ ] Directory tree viewer
* [ ] Simple HTTP request CLI

---

## 🟠 Advanced System Tools

* [ ] Mini port scanner *(beta / educational use only)*
* [ ] AES encryption / decryption tool
* [ ] System resource monitor CLI
* [ ] Lightweight grep clone (regex + recursive search)
* [ ] Config-driven CLI framework

---

# 🖼️ lab-image — Image Processing CLI Tools

## 🟢 Starter & Intermediate

* [ ] Grayscale image converter
* [ ] Image format converter (PNG ↔ JPG)
* [ ] Image resizer CLI
* [ ] Image crop tool (center / coordinates)
* [ ] Image rotation tool (90°, 180°, custom)
* [ ] Blur / sharpen filter CLI
* [ ] Edge detection (Sobel filter)
* [ ] Brightness / contrast adjuster
* [ ] Batch image processor (folder → output pipeline)
* [ ] Thumbnail generator

---

## 🟠 Advanced Image Processing

* [ ] ASCII art generator from images
* [ ] Duplicated images Archiver 
* [ ] Image histogram analyzer
* [ ] Watermark overlay tool 
* [ ] Compression analysis tool (size vs quality tradeoff)

---

# 🧠 lab-ml — Data & Statistical Learning CLI

* [ ] Bayesian MNIST classifier CLI
* [ ] Gaussian Naive Bayes + Logistic Regression comparison
* [ ] KNN classifier (to test)
* [ ] K-Means clustering tool (to do)
* [ ] CNN from scratch (no PyTorch / Keras)
* [ ] Peak frequency detector (FFT-based analysis)
* [ ] S&P 500 FFT cycle analyzer (log returns)
* [ ] Rolling FFT backtesting engine

---

# 🧠 lab-solid — Design Patterns

* [ ] Traits instead of interfaces
* [ ] Generics instead of DI containers
* [ ] Explicit wiring instead of reflection
* [ ] Constructor injection

 Rust achieves IoC through:
    Constructor injection
    Trait-based abstraction
    Explicit dependency passing


# ⚙️ Key Concepts Refresher

## 🦀 Rust fundamentals

* `String` → heap-allocated owned data
* `&str` → borrowed string slice
* Ownership prevents data races and memory bugs
* `Vec<T>` → dynamic heap collection
* Iterators → zero-cost abstraction patterns

---

## 🔤 Text processing

* `.split_whitespace()` → word tokenization
* `.chars()` → Unicode-safe iteration
* `.lines()` → file line streaming
* `.to_lowercase()` → normalization
* `.filter()` → iterator filtering
* `.max_by_key()` → selection logic

---

## 🧮 Error handling

* `Result<T, E>` → explicit failure handling
* `Option<T>` → optional values instead of null
* `match` → exhaustive control flow

---

## 📂 File & IO

* `File::open()` → safe file access
* `BufReader` → efficient streaming reads
* `.lines()` → iterator-based file parsing
* Avoids loading entire file into memory

---

## 🧠 Data / ML utilities

* Feature extraction via statistics
* Mean / variance normalization
* Naive Bayes assumptions (Gaussian distributions)
* Log returns for financial time series
* FFT for frequency domain analysis

---

# 🚀 Example Usage

## Word counter CLI

```bash
cargo run -- "Rust is fast and memory safe"
```

---

## File-based tools

```bash
cargo run -- input.txt
```

---

## Grep clone

```bash
cargo run -- "TODO" ./src --recursive
```

---

## FFT / Market analyzer

```bash
cargo run -- data/sp500.csv
```

---

# 🧭 Design Philosophy

This repo follows:

> **small tools → real systems → composable utilities**

Each project is:

* independent
* CLI-driven
* minimal dependencies
* production-style Rust patterns


# Performance & Library Usage

This project does not utilize third-party Rust crates for machine learning operations. For more advanced models, please refer to:

    XGBoost bindings: xgboost-rust, quickgrove
    Tree-based algorithms: linfa-trees, smelt-ml, sklears-tree

Performance benchmarks and runtime characteristics are specific to:
    Hardware: Apple Mac mini (Apple Silicon M-series)
    Memory: Unified memory architecture
    OS: macOS (latest stable)

Important Notes:
    Not Cross-Platform Certified: Results observed on Mac mini hardware may differ on Linux, Windows, or Intel-based systems due to variations in CPU architecture, memory bandwidth, and instruction set optimizations.

    Determinism Not Guaranteed: Due to potential non-deterministic behavior in tree-based algorithms (feature ordering, parallel thread scheduling, floating-point operations), results may vary between runs, systems, or when using different Rust compiler versions.