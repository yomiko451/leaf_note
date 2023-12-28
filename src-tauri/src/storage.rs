use tauri::AppHandle;
use std::fs;
use crate::types::Note;

#[tauri::command]
pub fn initialize(app_handle: AppHandle) {
    let path = app_handle
        .path_resolver()
        .app_local_data_dir()
        .unwrap();
    let notes_path = path.join("notes");
    if !fs::metadata(&notes_path).is_ok() {
        fs::create_dir(&notes_path).unwrap();
    };
    let todo_path = path.join("todo");
    if !fs::metadata(&todo_path).is_ok() {
        fs::create_dir(&todo_path).unwrap();
    };
}

#[tauri::command]
pub fn load_data() {
    unimplemented!()
}

#[tauri::command]
pub fn save_note(app_handle: AppHandle, note: Note) {
    let path = app_handle.path_resolver().app_local_data_dir().unwrap().join(format!("notes/{}.json", note.id));
    let file = fs::File::create(path).unwrap();
    serde_json::to_writer_pretty(file, &note).unwrap();
}