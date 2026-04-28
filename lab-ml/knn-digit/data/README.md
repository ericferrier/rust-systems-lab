🔢 Digit Recognizer (Kaggle)
This is the official competition page dataset (MNIST-style handwritten digits).

https://www.kaggle.com/datasets/animatronbot/mnist-digit-recognizer/data

🚀 How to run
cargo run

On this dataset:
KNN will be slow
accuracy usually ~96–97%
performance depends heavily on:
normalization ✔
choice of k ✔
distance metric ✔
🧠 What you learn from this project

This project teaches:
✔ curse of dimensionality (784 features)
✔ why KNN is slow in production
✔ why CNN eventually replaces KNN
✔ importance of scaling
✔ memory + performance tradeoffs in Rust



KNN Accuracy: 0.9706

--- Confusion Matrix ---
       0   1   2   3   4   5   6   7   8   9
 0 | 848   1   0   0   0   1   2   0   0   0
 1 |   0 947   1   1   1   0   0   0   0   0
 2 |   8   3 766   3   2   2   1  10   5   1
 3 |   0   3   4 855   0   7   0   3   4   6
 4 |   0   7   0   0 777   0   5   0   1  21
 5 |   2   0   0  11   0 714   9   0   3   8
 6 |   3   1   0   0   1   1 825   0   0   0
 7 |   0   9   2   0   1   0   0 864   0   8
 8 |   2  17   1   6   3   9   1   2 749  12
 9 |   5   2   2   5   6   1   0  11   0 808
