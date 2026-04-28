use std::fs::File;
use std::io::Read;
use std::path::Path;

use sha2::{Sha256, Digest};

pub fn sha256_file(path: &Path) -> Option<String> {
    let mut file = File::open(path).ok()?;
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 8192];

    loop {
        let bytes = file.read(&mut buffer).ok()?;
        if bytes == 0 {
            break;
        }
        hasher.update(&buffer[..bytes]);
    }

    Some(hex::encode(hasher.finalize()))
}