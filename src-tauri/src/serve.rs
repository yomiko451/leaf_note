use chrono::Local;
use tauri::{Manager, Window};

#[tauri::command]
pub fn get_time() -> String {
    Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

#[tauri::command]
pub async fn show_main_window(window: Window) {
    let main_window = window.get_window("main").unwrap();
    main_window.show().unwrap();
}


