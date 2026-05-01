use engine::{cache, hasher, scanner};
use std::sync::Mutex;
use tauri::State;

struct AppState {
    conn: Mutex<rusqlite::Connection>,
}

#[derive(serde::Serialize)]
struct DuplicateGroup {
    hash: String,
    files: Vec<String>,
}

// Js call by invoke("scan_duplicates")
#[tauri::command]
fn scan_duplicates(folder: String, state: State<AppState>) -> Vec<DuplicateGroup> {
    let images = scanner::scan_folder(&folder);
    let duplicates = hasher::find_duplicates(images, &state.conn);

    duplicates
        .into_iter()
        .map(|(hash, files)| DuplicateGroup {
            hash: hash[..8].to_string(),
            files: files
                .iter()
                .map(|f| f.path.to_string_lossy().to_string())
                .collect(),
        })
        .collect()
}

// Js call by invoke("delete_file")
#[tauri::command]
fn delete_file(path: String) -> Result<(), String> {
    trash::delete(&path).map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let conn = cache::init_db();

    tauri::Builder::default()
        .manage(AppState {
            conn: Mutex::new(conn),
        })
        .invoke_handler(tauri::generate_handler![scan_duplicates, delete_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
