use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};
use chrono::Local;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Note {
    pub id: u128,
    pub title: String,
    pub content: String,
    pub created_at: String,
    pub updated_at: String,
    pub tags: Vec<String>,
    pub stared: bool,
    pub path: PathBuf,
}

impl Note {
    pub fn new() -> Note {
        let id = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_micros();
        let time = Local::now()
            .format("%Y-%m-%d")
            .to_string();
        Note {
            id,
            title: "".to_string(),
            content: "".to_string(),
            created_at: time.clone(),
            updated_at: time,
            tags: vec![],
            stared: false,
            path: PathBuf::from(""),
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