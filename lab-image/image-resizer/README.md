📏 Resize (keep aspect ratio)
cargo run -- input.jpg output.jpg 800x600

👉 fits inside box (no distortion)

🖼️ Fill exact size (crop if needed)
cargo run -- input.jpg output.jpg 800x600 fill

👉 fills exactly 800×600 (may crop)

🔻 Resize + compress
cargo run -- input.jpg output.jpg 800x600 fit 75

👉 smaller file + resized