use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    let path = get_path();
    let root = Path::new(&path);

    println!("{}", root.display());

    if root.is_dir() {
        print_tree(root, 0);
    } else {
        println!("Not a directory");
    }
}

fn print_tree(path: &Path, depth: usize) {
    let entries = match fs::read_dir(path) {
        Ok(e) => e,
        Err(_) => return,
    };

    let mut items: Vec<PathBuf> = entries
        .filter_map(|e| e.ok().map(|e| e.path()))
        .collect();

    // Optional: sort for stable output
    items.sort();

    for item in items {
        let name = item.file_name().unwrap().to_string_lossy();

        for _ in 0..depth {
            print!("│   ");
        }

        if item.is_dir() {
            println!("├── {}", name);
            print_tree(&item, depth + 1);
        } else {
            println!("├── {}", name);
        }
    }
}

fn get_path() -> String {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        args[1].clone()
    } else {
        ".".to_string() // default: current directory
    }
}