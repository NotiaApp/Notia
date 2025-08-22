// photo_manager.rs
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PhotoNote {
    pub path: String,
    pub note: String,
    pub timestamp: String,
}

#[derive(Debug)]
pub struct PhotoManager {
    pub photos: Vec<String>,
    pub notes: HashMap<String, PhotoNote>,
    pub notes_file: PathBuf,
}

impl Default for PhotoManager {
    fn default() -> Self {
        Self::new()
    }
}

impl PhotoManager {
    pub fn new() -> Self {
        let mut manager = PhotoManager {
            photos: Vec::new(),
            notes: HashMap::new(),
            notes_file: dirs::home_dir().unwrap_or_else(|| PathBuf::from("."))
                .join(".notia_notes.json"),
        };

        manager.load_notes();
        manager.scan_photos();
        manager
    }

    pub fn scan_photos(&mut self) {
        let mut photos = Vec::new();

        // Standart resim dizinlerini tara
        let picture_dirs = vec![
            dirs::picture_dir(),
            dirs::home_dir().map(|p| p.join("Pictures")),
            dirs::home_dir().map(|p| p.join("Resimler")),
            dirs::home_dir().map(|p| p.join("Downloads")),
            dirs::home_dir().map(|p| p.join("İndirilenler")),
        ];

        for dir_opt in picture_dirs {
            if let Some(dir) = dir_opt {
                if dir.exists() {
                    if let Ok(entries) = fs::read_dir(dir) {
                        for entry in entries.flatten() {
                            let path = entry.path();
                            if path.is_file() {
                                if let Some(ext) = path.extension() {
                                    if let Some(ext_str) = ext.to_str() {
                                        match ext_str.to_lowercase().as_str() {
                                            "jpg" | "jpeg" | "png" | "gif" | "bmp" | "webp" => {
                                                if let Ok(path_str) = path.into_os_string().into_string() {
                                                    photos.push(path_str);
                                                }
                                            }
                                            _ => {}
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        self.photos = photos;
    }

    pub fn load_notes(&mut self) {
        if self.notes_file.exists() {
            if let Ok(content) = fs::read_to_string(&self.notes_file) {
                if let Ok(notes_vec) = serde_json::from_str::<Vec<PhotoNote>>(&content) {
                    for note in notes_vec {
                        self.notes.insert(note.path.clone(), note);
                    }
                }
            }
        }
    }

    pub fn save_notes(&self) {
        let notes_vec: Vec<PhotoNote> = self.notes.values().cloned().collect();
        if let Ok(json) = serde_json::to_string_pretty(&notes_vec) {
            let _ = fs::write(&self.notes_file, json);
        }
    }

    pub fn add_note(&mut self, photo_path: &str, note: String) {
        let photo_note = PhotoNote {
            path: photo_path.to_string(),
            note,
            timestamp: chrono::Utc::now().to_rfc3339(),
        };
        self.notes.insert(photo_path.to_string(), photo_note);
        self.save_notes();
    }

    pub fn get_note(&self, photo_path: &str) -> Option<&PhotoNote> {
        self.notes.get(photo_path)
    }

    pub fn remove_note(&mut self, photo_path: &str) {
        self.notes.remove(photo_path);
        self.save_notes();
    }



    pub fn clear_notes(&mut self) {
        self.notes.clear();
        self.save_notes();
    }
}
