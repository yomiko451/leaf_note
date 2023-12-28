use std::time::{SystemTime, UNIX_EPOCH};
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


#[derive(Debug, Serialize, Deserialize)]
pub struct Note {
    pub id: u128,
    pub title: String,
    pub content: String,
    pub created_at: String,
    pub updated_at: String,
    pub tags: Vec<String>,
    pub stared: bool,
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
            stared: false
        }
    }
}

impl GetMetadata for Note {} 

#[derive(Debug, Serialize, Deserialize)]
pub struct TodoList {
    pub id: u128,
    pub title: String,
    pub content: Vec<Todo>,
    pub created_at: String
}

impl TodoList {
    pub fn new(title: String) -> TodoList {
        TodoList {
            id: 0,
            title: title,
            content: vec![],
            created_at: TodoList::get_time()
        }
    }
}

impl GetMetadata for TodoList {}

#[derive(Debug, Serialize, Deserialize)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_note_new() {
        let note = Note::new();
        println!("{:#?}", note);
    }
}