use std::path::PathBuf;
use walkdir::WalkDir;
use crate::models::ImageFile;

fn is_image(path: &PathBuf) -> bool {
    let extensions = ["jpg", "jpeg", "png", "gif", "bmp", "webp", "tiff", "heic"];
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| extensions.contains(&ext.to_lowercase().as_str()))
        .unwrap_or(false)
}

pub fn scan_folder(folder_path: &str) -> Vec<ImageFile> {
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