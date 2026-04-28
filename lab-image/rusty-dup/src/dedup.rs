fn is_image(path: &Path) -> bool {
    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        matches!(ext.to_lowercase().as_str(), "jpg" | "jpeg" | "png" | "gif" | "bmp" | "tiff" | "webp")
    } else {
        false
    }
}

fn hash_file(path: &Path) -> String {
    let mut file = fs::File::open(path).unwrap();
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 4096];
    loop {
        let n = file.read(&mut buffer).unwrap();
        if n == 0 { break; }
        hasher.update(&buffer[..n]);
    }
    format!("{:x}", hasher.finalize())
}

fn move_to_duplicates(path: &Path, dup_folder: &Path) {
    if let Some(filename) = path.file_name() {
        let dest = dup_folder.join(filename);
        let _ = fs::rename(path, dest);
    }
}
use std::collections::HashMap;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};

use walkdir::WalkDir;
use sha2::{Sha256, Digest};

pub struct Report {
    pub total_files: usize,
    pub duplicates: usize,
    pub bytes_saved: u64,
}

pub fn process_duplicates(input: &str, output: &str) -> Report {
    let mut seen: HashMap<String, PathBuf> = HashMap::new();

    let dup_folder = Path::new(output).join("duplicates");
    fs::create_dir_all(&dup_folder).unwrap();

    let mut report = Report {
        total_files: 0,
        duplicates: 0,
        bytes_saved: 0,
    };

    for entry in WalkDir::new(input)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();

        if !is_image(path) {
            continue;
        }

        report.total_files += 1;

        let file_size = fs::metadata(path).unwrap().len();

        let hash = hash_file(path);

        if let Some(original) = seen.get(&hash) {
            println!("Duplicate: {:?} (original: {:?})", path, original);

            report.duplicates += 1;
            report.bytes_saved += file_size;

            move_to_duplicates(path, &dup_folder);
        } else {
            seen.insert(hash, path.to_path_buf());
        }
    }

    report
}