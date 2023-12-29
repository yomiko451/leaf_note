#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use leaf_note::{storage, serve};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            storage::save_note,
            storage::load_note,
            storage::delete_note,
            storage::initialize,
            storage::save_todo_list,
            storage::delete_todo_list,
            storage::load_todo_list,
            storage::check_exist,
            serve::create_note,
            serve::create_todo_list,
            serve::create_todo,
            serve::get_time
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
