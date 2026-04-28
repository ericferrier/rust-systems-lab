🚀 How to run

cargo run -- ./images --mode exact

🔍 Similar image detection (default)
cargo run -- ./images --mode similar --threshold 85
🎯 More strict similarity
cargo run -- ./images --mode similar --threshold 98



📁 Output structure
output/
 └── duplicates/
      ├── img1.jpg
      ├── img2.png