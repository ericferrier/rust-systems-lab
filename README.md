# Rust Playground #
A Rust playground for practical CLI tools and fast, build-ready refreshers.


## Systems lab backlog ##

### Starter & Intermediate
- [ ] Word frequency counter
- [ ] Character counter CLI
- [ ] Palindrome checker
- [ ] Simple calculator CLI
- [ ] File line counter
- [ ] Log file analyzer
- [ ] JSON parser CLI
- [ ] File organizer tool
- [ ] Directory tree viewer
- [ ] Simple HTTP request CLI

### Advanced
- [ ] Mini port scanner (beta use only)
- [ ] Basic encryption/decryption tool
- [ ] System resource monitor CLI
- [ ] Lightweight grep clone
- [ ] Config-driven CLI framework

## 🖼️ Image Processing CLI Projects

### Starter & Intermediate

- [ ] Image grayscale converter
- [ ] Image format converter (PNG ↔ JPG)
- [ ] Image resizer CLI
- [ ] Image crop tool (center / coordinates)
- [ ] Image rotation tool (90°, 180°, custom)
- [ ] Image blur / sharpen filter CLI
- [ ] Edge detection (Sobel filter)
- [ ] Brightness / contrast adjuster
- [ ] Batch image processor (folder input → output folder)
- [ ] Thumbnail generator for directories

---

### Advanced Image Processing

- [ ] ASCII art generator from images
- [ ] Simple image compression analyzer (size vs quality tradeoff)
- [ ] Watermark overlay tool
- [ ] Image histogram analyzer

### Statistical Data Learning
- [ ]  Bayesian MNIST Classifier CLI
- [ ]  GNB and Logistic Regression
- [ ]  KNN Classifier
- [ ]  K-Means Clustering
- [ ]  CNN Images classifier


# NOTES #


##   Key Concepts Refresher ##

  * `String` → heap-allocated, owned data
  * `&str` → borrowed slice (no ownership)
  * `split_whitespace()` → returns iterator over words
  * `Vec<&str>` → stores references, avoids copying
  * `max_by_key()` → finds longest word by length
  * Ownership stays with the original `String`


  * `.chars()` → Unicode-safe character iteration
  * `.filter()` → remove whitespace
  * `.lines()` → split by line
  * `String` ownership stays in main
  * No extra allocations (iterator-based)

  * `.is_alphanumeric()` → ignore punctuation/spaces
  * `.to_lowercase()` → case normalization
  * `.rev()` → reverse iterator
  * String transformation pipeline
  * Ownership + new String allocation

  * `split_whitespace()` → simple parsing
  * `parse::<f64>()` → string → number
  * `match` → operator handling
  * `Result<T, E>` → error handling
  * Basic input validation
    Computation happens here
      "+" => Ok(left + right)

  * `File::open()` → open file safely
  * `BufReader` → efficient reading (buffered)
  * `.lines()` → iterator over lines
  * `Result<T, E>` → propagate file errors
  * No full file load into memory


🚀 How to run
```bash
cd words-cli
cargo run -- "Rust is fast and memory safe"
```





