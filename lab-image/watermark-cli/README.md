🚀 How to run
📁 Batch watermark folder
cargo run -- ./input_images ./output_images

📊 Output
input_images/
 ├── a.jpg
 ├── b.png

output_images/
 ├── a.jpg   (watermarked)
 ├── b.png   (watermarked)
🧠 What this version does

✔ loads images
✔ applies watermark overlay
✔ processes entire folder recursively
✔ saves to output folder
✔ zero unsafe code
✔ zero heavy dependencies