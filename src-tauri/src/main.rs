#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use leaf_note::{storage, serve};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            storage::create_note,
            storage::save_note,
            serve::get_time
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
