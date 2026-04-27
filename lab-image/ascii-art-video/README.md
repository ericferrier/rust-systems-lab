🎥 Default playback
cargo run -- video.mp4

⚙️ Custom width
cargo run -- video.mp4 --width 80

⚡ Faster playback
cargo run -- video.mp4 --width 120 --fps 30

🧠 What you just built
This is now a terminal video renderer pipeline:

✔ MP4 decoding (FFmpeg bindings)
✔ frame extraction
✔ grayscale conversion
✔ ASCII quantization
✔ real-time rendering loop
✔ FPS control system

🔥 Why this is powerful

You basically built:

🎬 “Netflix in ASCII mode”

Used in:
terminal dashboards
retro system monitoring
embedded systems (low GPU environments)
ML debugging visualization
hacker-style UI tools

Note:
ffmpeg-sys-next crate requires the native FFmpeg libraries (like libavutil)