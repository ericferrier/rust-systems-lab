🌫️ Parallel batch resize
cargo run -- ./input ./output --resize 800x800

🌫️ Grayscale + blur
cargo run -- ./input ./output --gray --blur 2

🚀 Full pipeline
cargo run -- ./input ./output --resize 800x800 --gray --blur 1

🧠 What you just built
You now have a real image processing engine:

✔ parallel execution (Rayon)
✔ CLI flag parser
✔ composable pipeline design
✔ recursive batch processing
✔ production-style architecture

⚡ Why Rayon matters
Instead of: 1 image → 1 image → 1 image
You now do:  100 images processed simultaneously across CPU cores
👉 massive speedup (5x–20x depending on CPU)

🔆 Increase brightness
cargo run -- input.jpg output.jpg 30 1.0

📊 Increase contrast
cargo run -- input.jpg output.jpg 0 1.5

⚡ Combined adjustment
cargo run -- input.jpg output.jpg 20 1.3
🧠 What you built

You now implemented:
✔ pixel-level image manipulation
✔ linear transformation of RGB space
✔ real-time image adjustment pipeline
✔ foundation for ML preprocessing normalization

🔥 Why this matters (important connection)

This directly connects to ML pipelines:

Operation	ML equivalent
brightness shift	bias adjustment
contrast scaling	feature scaling
normalization	preprocessing step

👉 This is exactly what CNN pipelines rely on before training