mod watermark;

use watermark::add_text_like_watermark;

use std::env;
use std::fs;
use std::path::Path;

use image::open;
use walkdir::WalkDir;


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage:");
        println!("cargo run -- <input_folder> <output_folder>");
        return;
    }

    let input = &args[1];
    let output = Path::new(&args[2]);

    fs::create_dir_all(output).unwrap();

    println!("Processing folder: {}\n", input);

    for entry in WalkDir::new(input)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();

        if !is_image(path) {
            continue;
        }

        let img = match open(path) {
            Ok(i) => i,
            Err(_) => continue,
        };

        let watermarked = add_text_like_watermark(img, "WATERMARK");

        let file_name = path.file_name().unwrap();
        let out_path = output.join(file_name);

        watermarked.save(out_path).unwrap();

        println!("Watermarked: {:?}", path);
    }
}

fn is_image(path: &Path) -> bool {
    match path.extension().and_then(|e| e.to_str()) {
        Some(ext) => matches!(
            ext.to_lowercase().as_str(),
            "png" | "jpg" | "jpeg" | "webp"
        ),
        None => false,
    }
}