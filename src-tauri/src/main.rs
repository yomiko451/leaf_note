#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use leaf_note::{storage, serve, spider};
use tauri::Manager;

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
            storage::check_note_exist,
            storage::update_config,
            serve::create_note,
            serve::create_todo_list,
            serve::create_todo,
            serve::get_time,
            serve::show_main_window,
            serve::restart_app,
        ])
        .setup(|app| {
            let main_window = app.get_window("main").unwrap();
            tauri::async_runtime::spawn(async move {
                //TODO: 初始化
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
