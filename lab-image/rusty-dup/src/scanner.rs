use rayon::prelude::*;
use std::path::{PathBuf};
use walkdir::WalkDir;

use crate::phash::phash;

#[derive(Clone)]
pub struct ImageEntry {
    pub path: PathBuf,
    pub hash: u64,
}

pub fn scan_folder(root: &str) -> Vec<ImageEntry> {
    let entries: Vec<_> = WalkDir::new(root)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .collect();

    entries
        .par_iter()
        .filter_map(|entry| {
            let path = entry.path();

            if !is_image(path) {
                return None;
            }

            let hash = phash(path)?;

            Some(ImageEntry {
                path: path.to_path_buf(),
                hash,
            })
        })
        .collect()
}

fn is_image(path: &std::path::Path) -> bool {
    match path.extension().and_then(|e| e.to_str()) {
        Some(ext) => matches!(
            ext.to_lowercase().as_str(),
            "png" | "jpg" | "jpeg" | "webp"
        ),
        None => false,
    }
}