use chrono::Local;
use crate::types::{Note, TodoList, Todo};

#[tauri::command]
pub fn get_time() -> String {
    Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

#[tauri::command]
pub fn create_note() -> Note {
    Note::new()
}

#[tauri::command]
pub fn create_todo_list(title: String) -> TodoList {
    TodoList::new(title)
}

#[tauri::command]
pub fn create_todo(content: String) -> Todo {
    Todo::new(content)
}