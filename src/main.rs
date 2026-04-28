use blake3::Hasher;
use std::{collections::HashMap, fs, path::PathBuf};
use walkdir::WalkDir;

// a struct in rust equivalent to class in Java
struct ImageFile {
    path: PathBuf,
    size: u64,
}

impl ImageFile {
    fn new(path: PathBuf, size: u64) -> Self {
        ImageFile { path, size }
    }

    fn display(&self) {
        // &self = borrow, no ownership
        println!("File: {:?}, Size: {:?} byte", self.path, self.size)
    }
}

fn is_image(path: &PathBuf) -> bool {
    let extensions = ["jpg", "jpeg", "png", "gif", "bmp", "webp", "tiff", "heic"];
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| extensions.contains(&ext.to_lowercase().as_str()))
        .unwrap_or(false)
}

fn scan_folder(folder_path: &str) -> Vec<ImageFile> {
    let mut images = Vec::new();

    for entry in WalkDir::new(folder_path)
        .follow_links(false) // not follow symbol link
        .into_iter()
        .filter_map(|e| e.ok())
    // skip permission error
    {
        let path = entry.path().to_path_buf();

        if path.is_file() && is_image(&path) {
            let size = entry.metadata().map(|m| m.len()).unwrap_or(0);

            images.push(ImageFile::new(path, size));
        }
    }
    images
}

// Hash by using file by blake3
fn hash_file(path: &PathBuf) -> Option<String> {
    let bytes = fs::read(path).ok()?; // read file, return file if has errors
    let mut hasher = Hasher::new();
    hasher.update(&bytes);
    Some(hasher.finalize().to_hex().to_string())
}

fn find_dupliactes(images: Vec<ImageFile>) -> HashMap<String, Vec<ImageFile>> {
    let mut hash_map: HashMap<String, Vec<ImageFile>> = HashMap::new();

    for img in images {
        print!("Hashing {:?} ... ", img.path.file_name().unwrap());
        if let Some(hash) = hash_file(&img.path) {
            println!("{}", &hash[..8]); // print first 8 character of hash
            hash_map.entry(hash).or_default().push(img);
        }
    }
    // Only retain those groups has >= 2 files (really duplicate)
    hash_map.retain(|_, files| files.len() > 1);
    hash_map
}

fn main() {
    let folder = "/Users/omori/duplicate-images";

    println!("Scanning ... {}", folder);
    let images = scan_folder(folder);
    print!("Found {} images \n", images.len());

    let duplicates = find_dupliactes(images);

    if duplicates.is_empty() {
        println!("No duplicates found!")
    } else {
        println!("\n=== Found {} duplicate groups ===\n", duplicates.len());
        for (hash, files) in &duplicates {
            println!("Hash: {} ...", &hash[..8]);
            for file in files {
                file.display();
            }
            println!();
        }
    }
}
