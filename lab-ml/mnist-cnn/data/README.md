📦 Kaggle MNIST CSV

Go to:

https://www.kaggle.com/datasets/oddrationale/mnist-in-csv

You’ll get:

mnist_train.csv
mnist_test.csv



cargo run -- data/mnist_train.csv data/mnist_test.csv



Loading training data...
Loading test data...
Training...
Epoch 1/20 - loss: 2.3053
Epoch 2/20 - loss: 2.1990
Epoch 3/20 - loss: 2.1024
Epoch 4/20 - loss: 2.0133
Epoch 5/20 - loss: 1.9308
Epoch 6/20 - loss: 1.8542
Epoch 7/20 - loss: 1.7833
Epoch 8/20 - loss: 1.7176
Epoch 9/20 - loss: 1.6566
Epoch 10/20 - loss: 1.6002
Epoch 11/20 - loss: 1.5478
Epoch 12/20 - loss: 1.4992
Epoch 13/20 - loss: 1.4541
Epoch 14/20 - loss: 1.4121
Epoch 15/20 - loss: 1.3731
Epoch 16/20 - loss: 1.3367
Epoch 17/20 - loss: 1.3027
Epoch 18/20 - loss: 1.2710
Epoch 19/20 - loss: 1.2413
Epoch 20/20 - loss: 1.2134
Test accuracy: 81.76%
Done.


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