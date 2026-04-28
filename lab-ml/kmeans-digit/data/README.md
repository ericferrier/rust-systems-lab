🔢 Digit Recognizer (Kaggle)
This is the official competition page dataset (MNIST-style handwritten digits).

https://www.kaggle.com/datasets/animatronbot/mnist-digit-recognizer/data



🚀 How to run
Manual k:
cargo run -- data/train.csv --k 10 --dist euclidean


cargo run -- data/train.csv --k 10 --dist sqeuclidean

SquaredEuclidean

Auto-k (elbow method):
cargo run -- data/train.csv --auto-k --dist manhattan

cargo run -- data/train.csv --auto-k --dist  sqeuclidean



