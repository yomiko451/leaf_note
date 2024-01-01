use chrono::Local;
use crate::types::{Note, TodoList, Todo};
use crate::storage;
use tauri::{AppHandle, Manager};

#[tauri::command]
pub fn get_time() -> String {
    Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

#[tauri::command]
pub fn create_note(app_handle: AppHandle) -> Note {
    let note = Note::new();
    storage::save_note(app_handle, note.clone());
    note
}

#[tauri::command]
pub fn create_todo_list(app_handle: AppHandle, title: String) -> TodoList {
    let todo_list = TodoList::new(title);
    storage::save_todo_list(app_handle, todo_list.clone());
    todo_list
}

#[tauri::command]
pub fn create_todo(content: String) -> Todo {
    Todo::new(content)
}

#[tauri::command]
pub async fn show_main_window(window: tauri::Window) {
    let main_window = window.get_window("main").unwrap();
    main_window.show().unwrap();
}


