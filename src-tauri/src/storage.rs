use crate::types::Note;

#[tauri::command]
pub fn create_note() -> Note {
    Note::new()
}