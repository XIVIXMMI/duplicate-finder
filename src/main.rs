mod cache;
mod hasher;
mod models;
mod scanner;

use std::sync::Mutex;

fn main() {
    let folder = "/Users/omori/duplicate-images";
    let conn = Mutex::new(cache::init_db());

    println!("Scanning ... {}", folder);
    let images = scanner::scan_folder(folder);
    print!("Found {} images \n", images.len());

    let duplicates = hasher::find_duplicates(images, &conn);

    if duplicates.is_empty() {
        println!("No duplicates found!");
        return;
    }

    println!("\n=== Found {} duplicate groups ===\n", duplicates.len());

    for (hash, files) in &duplicates {
        println!("Hash: {} ...", &hash[..8]);
        let (keep, to_delete) = files.split_first().unwrap();

        println!("Keep: {:?}", keep.path.file_name().unwrap());

        for file in to_delete {
            println!("Delete: {:?}", file.path.file_name().unwrap());
        }

        // confirm before delete
        println!("Move to trash? [Y/N]: ");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        if input.trim() == "y" || input.trim() == "Y" {
            for file in to_delete {
                match trash::delete(&file.path) {
                    Ok(_) => println!("Moved to Trash: {:?}", file.path.file_name().unwrap()),
                    Err(e) => println!("Error: {}", e),
                }
            }
        } else if input.trim() == "N" || input.trim() == "n" {
            println!("Skiped.");
        } else {
            println!("Please enter 'Y' or 'N' ");
        }
        println!();
    }
}
