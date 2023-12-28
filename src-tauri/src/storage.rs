use tauri::AppHandle;
use std::fs;
use crate::types::{Note, TodoList, Todo};

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

#[tauri::command]
pub fn save_note(app_handle: AppHandle, note: Note) {
    let path = app_handle.path_resolver().app_local_data_dir().unwrap().join(format!("{}.json", note.id));
    let file = fs::File::create(path).unwrap();
    serde_json::to_writer_pretty(file, &note).unwrap();
}