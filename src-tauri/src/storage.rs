use tauri::{AppHandle, Window};
use std::{fs, path::PathBuf, process::Command};
use chrono::Local;
use rand::{self, Rng};
use crate::{types::{Note, TodoList, Config, Todo}, spider, serve};

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
pub async fn load_config(app_handle: AppHandle, window: Window) -> Config {
    let path = app_handle.path_resolver().app_local_data_dir().unwrap().join("config.json");
    let cover_path = app_handle.path_resolver().app_local_data_dir().unwrap().join("cover");
    let file = fs::File::open(&path).unwrap();
    let mut config: Config = serde_json::from_reader(&file).unwrap();
    config.cover_url = get_cover(cover_path);
    serve::resize_main_window(window, config.ui_scale);
    check_date(app_handle, config).await
}

async fn check_date(app_handle: AppHandle, mut config: Config) -> Config {
    let date = Local::now().format("%Y-%m-%d").to_string();
    if config.weather.date != date {
        config.weather = spider::get_weather(config.city.clone()).await;
        update_config(app_handle, config.clone());
    }
    config
}

fn get_cover(cover_path: PathBuf) -> PathBuf {
    if let Ok(file) = fs::read_dir(cover_path) {
        let files = file.map(|f|f.unwrap().path()).collect::<Vec<_>>();
        if files.len() > 0 {
            let index = rand::thread_rng().gen_range(0..(files.len() - 1));
            files[index].clone()
        } else {
            PathBuf::new()
        }
    } else {
        PathBuf::new()
    }
}

#[tauri::command]
pub fn create_note(app_handle: AppHandle) -> Note {
    let note = Note::new();
    save_note(app_handle, note.clone());
    note
}

#[tauri::command]
pub fn create_todo_list(app_handle: AppHandle, title: String) -> TodoList {
    let todo_list = TodoList::new(title);
    save_todo_list(app_handle, todo_list.clone());
    todo_list
}

#[tauri::command]
pub fn create_todo(content: String) -> Todo {
    Todo::new(content)
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
pub fn delete_note(app_handle: AppHandle, note: Note) {
    let path = app_handle.path_resolver().app_local_data_dir().unwrap().join(format!("note/{}.json", note.id));
    fs::remove_file(path).unwrap();
}

#[tauri::command]
pub fn save_todo_list(app_handle: AppHandle, todo_list: TodoList) {
    let path = app_handle.path_resolver().app_local_data_dir().unwrap().join(format!("todo/{}.json", todo_list.id));
    let file = fs::File::create(path).unwrap();
    serde_json::to_writer_pretty(file, &todo_list).unwrap();
}

#[tauri::command]
pub fn delete_todo_list(app_handle: AppHandle, todo_list: TodoList) {
    let path = app_handle.path_resolver().app_local_data_dir().unwrap().join(format!("todo/{}.json", todo_list.id));
    fs::remove_file(path).unwrap();
}

#[tauri::command]
pub fn open_folder(app_handle: AppHandle, code: usize) {
    let path = app_handle.path_resolver().app_local_data_dir().unwrap();
    match code {
        0 => {
            Command::new("explorer.exe")
               .arg(path.join("note")).spawn().unwrap();
        },
        1 => {
            Command::new("explorer.exe")
               .arg(path.join("todo")).spawn().unwrap();
        },
        2 => {
            Command::new("explorer.exe")
              .arg(path.join("cover")).spawn().unwrap();
        },
        _ => ()
    };
}
