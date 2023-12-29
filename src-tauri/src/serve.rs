use chrono::Local;
use crate::types::{Note, TodoList, Todo};
use crate::storage;
use tauri::AppHandle;

#[tauri::command]
pub fn get_time() -> String {
    Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

#[tauri::command]
pub fn create_note() -> Note {
    Note::new()
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

