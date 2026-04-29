use blake3::Hasher;
use rayon::prelude::*;
use rusqlite::Connection;
use std::{collections::HashMap, fs, path::PathBuf, sync::Mutex, time::UNIX_EPOCH};
use crate::{cache, models::ImageFile};


// Hash by using file by blake3
fn hash_file(path: &PathBuf) -> Option<String> {
    let bytes = fs::read(path).ok()?; // read file, return file if has errors
    let mut hasher = Hasher::new();
    hasher.update(&bytes);
    Some(hasher.finalize().to_hex().to_string())
}

/**
 * Rust doesn't allow to write on mutiple thread in HashMap at the same time
 * Use Mutex to lock when write and unlock after write more safety in mutiple threads environment
 */
pub fn find_duplicates(
    images: Vec<ImageFile>,
    conn: &Mutex<Connection>,
) -> HashMap<String, Vec<ImageFile>> {
    let hash_map: Mutex<HashMap<String, Vec<ImageFile>>> = Mutex::new(HashMap::new());

    images.into_par_iter().for_each(|img| {
        let path_str = img.path.to_string_lossy().to_string();

        let modified_at = fs::metadata(&img.path)
            .and_then(|m| m.modified())
            .map(|t| t.duration_since(UNIX_EPOCH).unwrap().as_secs())
            .unwrap_or(0);

        let hash = {
            let db = conn.lock().unwrap();
            cache::get_cached_hash(&db, &path_str, modified_at)
        };

        let hash = match hash {
            Some(h) => {
                println!("Cache: {:?} -> {}", img.path.file_name().unwrap(), &h[..8]);
                h
            }
            None => {
                // Cache miss -> hash the real file then save to DB
                match hash_file(&img.path) {
                    Some(h) => {
                        let db = conn.lock().unwrap();
                        cache::save_hash(&db, &path_str, modified_at, &h);
                        println!("Hashed: {:?} -> {}", img.path.file_name().unwrap(), &h[..8]);
                        h
                    }
                    None => return,
                }
            }
        };

        let mut map = hash_map.lock().unwrap();
        map.entry(hash).or_default().push(img);
    });

    let mut result = hash_map.into_inner().unwrap();
    result.retain(|_, files| files.len() > 1);
    result
}
