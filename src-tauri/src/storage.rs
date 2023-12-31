use tauri::AppHandle;
use std::fs;
use chrono::Local;
use crate::{types::{Note, TodoList, Config}, spider};

#[tauri::command]
pub fn check_requirements(app_handle: AppHandle) {
    let path = app_handle
        .path_resolver()
        .app_local_data_dir()
        .unwrap();
    let note_path = path.join("note");
    if !fs::metadata(&note_path).is_ok() {
        fs::create_dir(&note_path).unwrap();
    };
    let todo_path = path.join("todo");
    if !fs::metadata(&todo_path).is_ok() {
        fs::create_dir(&todo_path).unwrap();
    };
    let cover_path = path.join("cover");
    if !fs::metadata(&cover_path).is_ok() {
        fs::create_dir(&cover_path).unwrap();
    };
    let config_path = path.join("config.json");
    if !fs::metadata(&config_path).is_ok() {
        let file = fs::File::create(config_path).unwrap();
        serde_json::to_writer_pretty(file, &Config::new()).unwrap();
    };
}

#[tauri::command]
pub fn load_note(app_handle: AppHandle) -> Vec<Note> {
    let note_path = app_handle.path_resolver().app_local_data_dir().unwrap().join("note");
    fs::read_dir(note_path).unwrap().into_iter().map(|f| {
        let path = f.unwrap().path();
        serde_json::from_reader(fs::File::open(path).unwrap()).unwrap()
    }).collect()
}

#[tauri::command]
pub fn load_todo_list(app_handle: AppHandle) -> Vec<TodoList> {
    let note_path = app_handle.path_resolver().app_local_data_dir().unwrap().join("todo");
    fs::read_dir(note_path).unwrap().into_iter().map(|f| {
        let path = f.unwrap().path();
        serde_json::from_reader(fs::File::open(path).unwrap()).unwrap()
    }).collect()
}

#[tauri::command]
pub async fn load_config(app_handle: AppHandle) -> Config {
    let path = app_handle.path_resolver().app_local_data_dir().unwrap().join("config.json");
    let file = fs::File::open(&path).unwrap();
    let mut config: Config = serde_json::from_reader(&file).unwrap();
    let date = Local::now().format("%Y-%m-%d").to_string();
    if config.weather.date != date {
        config.weather = spider::get_weather().await;
        config.weather.date = date;
        update_config(app_handle, config.clone());
        println!("ddd")
    }
    config
}

#[tauri::command]
pub fn update_config(app_handle: AppHandle, config: Config) {
    let path = app_handle.path_resolver().app_local_data_dir().unwrap().join("config.json");
    let file = fs::File::create(path).unwrap();
    serde_json::to_writer_pretty(file, &config).unwrap();
}

#[tauri::command]
pub fn save_note(app_handle: AppHandle, note: Note) {
    let path = app_handle.path_resolver().app_local_data_dir().unwrap().join(format!("note/{}.json", note.id));
    let file = fs::File::create(path).unwrap();
    serde_json::to_writer_pretty(file, &note).unwrap();
}

#[tauri::command]
pub fn delete_note(app_handle: AppHandle, item: Note) {
    let path = app_handle.path_resolver().app_local_data_dir().unwrap().join(format!("note/{}.json", item.id));
    fs::remove_file(path).unwrap();
}

#[tauri::command]
pub fn save_todo_list(app_handle: AppHandle, todo_list: TodoList) {
    let path = app_handle.path_resolver().app_local_data_dir().unwrap().join(format!("todo/{}.json", todo_list.id));
    let file = fs::File::create(path).unwrap();
    serde_json::to_writer_pretty(file, &todo_list).unwrap();
}

#[tauri::command]
pub fn delete_todo_list(app_handle: AppHandle, item: TodoList) {
    let path = app_handle.path_resolver().app_local_data_dir().unwrap().join(format!("todo/{}.json", item.id));
    fs::remove_file(path).unwrap();
}

#[tauri::command]
pub fn check_note_exist(app_handle: AppHandle, item: Note) -> bool {
    let path = app_handle.path_resolver().app_local_data_dir().unwrap().join(format!("note/{}.json", item.id));
    fs::metadata(path).is_ok()
}

