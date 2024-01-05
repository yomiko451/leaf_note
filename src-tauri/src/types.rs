use std::{time::{SystemTime, UNIX_EPOCH}, path::PathBuf};
use chrono::Local;
use serde::{Deserialize, Serialize};

trait GetMetadata {
    fn get_time() -> String {
        Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
    }

    fn get_id() -> u128 {
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_micros()
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    pub id: u128,
    pub title: String,
    pub content: String,
    pub created_at: String,
    pub updated_at: String,
    pub tags: Vec<String>,
    pub locked: bool
}

impl Note {
    pub fn new() -> Note {
        Note {
            id: Note::get_id(),
            title: "".to_string(),
            content: "".to_string(),
            created_at: Note::get_time(),
            updated_at: "暂无信息".to_string(),
            tags: vec![],
            locked: false
        }
    }
}

impl GetMetadata for Note {} 

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodoList {
    pub id: u128,
    pub title: String,
    pub content: Vec<Todo>,
    pub created_at: String
}

impl TodoList {
    pub fn new(title: String) -> TodoList {
        TodoList {
            id: TodoList::get_id(),
            title: title,
            content: vec![],
            created_at: TodoList::get_time()
        }
    }
}

impl GetMetadata for TodoList {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub id: u128,
    pub content: String,
    pub completed: bool
}

impl Todo {
    pub fn new(content: String) -> Todo {
        Todo {
            id: Todo::get_id(),
            content: content,
            completed: false
        }
    }
}

impl GetMetadata for Todo {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub cover_filter: bool,
    pub cover_url: PathBuf,
    pub font_size: usize,
    pub font_family: String,
    pub city: String,
    pub weather: Weather,
    pub ui_scale: usize,
    pub ui_style: bool
}

impl Config {
    pub fn new() -> Config {
        Config {
            font_size: 10,
            font_family: "微软雅黑".to_string(),
            cover_filter: false,
            cover_url: PathBuf::new(),
            city: "安庆".to_string(),
            weather: Weather::new(),
            ui_scale: 1,
            ui_style: false
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Weather {
    pub date: String,
    pub direct: String,
    pub temperature: String,
    pub weather: String,
}

impl Weather {
    pub fn new() -> Weather {
        Weather {
            date: "暂无信息".to_string(),
            direct: "暂无信息".to_string(),
            temperature: "暂无信息".to_string(),
            weather: "暂无信息".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_note_new() {
        let note = Note::new();
        println!("{:#?}", note);
    }
}