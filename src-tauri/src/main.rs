#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use leaf_note::{storage, serve, spider};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            storage::save_note,
            storage::load_note,
            storage::delete_note,
            storage::check_requirements,
            storage::save_todo_list,
            storage::delete_todo_list,
            storage::load_todo_list,
            storage::load_config,
            storage::update_config,
            storage::create_note,
            storage::create_todo_list,
            storage::create_todo,
            serve::get_time,
            serve::show_main_window,
            spider::get_weather
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
