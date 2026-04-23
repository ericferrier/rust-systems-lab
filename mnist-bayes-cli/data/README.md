📦 Kaggle MNIST CSV

Go to:

https://www.kaggle.com/datasets/oddrationale/mnist-in-csv

You’ll get:

mnist_train.csv
mnist_test.csv



cargo run -- data/mnist_train.csv data/mnist_test.csv




🧠 What happens when you run it
Rust compiles your project
Loads MNIST CSV files
Extracts features (mean + stddev per image)
Trains Gaussian Naive Bayes model
Runs predictions on test set
Prints accuracy:


🧠 What you built 

🔹 ML pipeline
dataset → features → model → prediction
🔹 Computer vision simplification
raw pixels → statistical features
🔹 Probabilistic modeling
Gaussian likelihoods
priors per class
🔹 Real ML architecture pattern

Same structure used in:

spam filters
OCR systems
anomaly detection
lightweight edge AI

🚀 Why MNIST is perfect here
standardized benchmark
small enough for CLI tools
ideal for Naive Bayes (baseline model)
easy to compare against sklearn results