#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use leaf_note::{storage, serve};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            storage::save_note,
            storage::initialize,
            serve::create_note,
            serve::create_todo_list,
            serve::create_todo,
            serve::get_time
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
