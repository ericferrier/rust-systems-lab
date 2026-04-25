▶️ Usage

cargo run -- input.jpg output.png

🧠 What you just built

You implemented:
✔ grayscale conversion
✔ gradient computation
✔ convolution kernels
✔ edge magnitude extraction

🔥 What Sobel is really doing
It detects intensity changes:

Direction	Meaning
Gx	vertical edges
Gy	horizontal edges
magnitude	edge strength

🧠 Why this matters (important)
This is exactly what CNNs start with:
Sobel = hand-crafted filters
CNN = learned filters


🧠 “manual first layer of a CNN in Rust”