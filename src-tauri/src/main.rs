#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use leaf_note::storage;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            storage::create_note
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
