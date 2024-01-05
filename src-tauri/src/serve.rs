use chrono::Local;
use tauri::{Manager, Window, LogicalSize};

#[tauri::command]
pub fn get_time() -> String {
    Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

#[tauri::command]
pub async fn show_main_window(window: Window) {
    let main_window = window.get_window("main").unwrap();
    main_window.show().unwrap();
}

#[tauri::command]
pub fn resize_main_window(window: Window, ui_scale: usize) {
    match ui_scale {
        0 => {
            window.set_size(LogicalSize::new(400.0, 300.0)).unwrap();
        },
        1 => {
            window.set_size(LogicalSize::new(800.0, 600.0)).unwrap();
        },
        2 => {
            window.set_size(LogicalSize::new(1200.0, 900.0)).unwrap();
        },
        _ => ()
    }
}


