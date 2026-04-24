🔐 Encrypt
cargo run -- encrypt data/input.txt data/encrypted.bin mysecretkey
🔓 Decrypt
cargo run -- decrypt data/encrypted.bin data/decrypted.txt mysecretkey
🧪 Example
input.txt:
hello world

After encryption → binary file
After decryption → back to:

hello world

🧠 Takeaway
File I/O (fs::read, fs::write)
CLI arguments
Byte-level operations
Simple symmetric encryption concept
Modular Rust (mod crypto)