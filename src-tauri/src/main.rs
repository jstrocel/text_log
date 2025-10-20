// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use chrono::Local;
use serde::{Deserialize, Serialize};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tauri::{Emitter, State};

#[derive(Serialize, Deserialize, Clone)]
struct AppSettings {
    journal_directory: PathBuf,
}

impl AppSettings {
    fn default() -> Self {
        let default_dir = dirs::document_dir()
            .map(|mut path| {
                path.push("MyJournal");
                path
            })
            .unwrap_or_else(|| PathBuf::from("./MyJournal"));
        
        Self {
            journal_directory: default_dir,
        }
    }
    
    fn load() -> Self {
        let settings_path = Self::get_settings_path();
        
        if let Ok(settings_content) = fs::read_to_string(&settings_path) {
            if let Ok(settings) = serde_json::from_str::<AppSettings>(&settings_content) {
                return settings;
            }
        }
        
        let default_settings = Self::default();
        let _ = default_settings.save();
        default_settings
    }
    
    fn save(&self) -> Result<(), String> {
        let settings_path = Self::get_settings_path();
        
        if let Some(parent) = settings_path.parent() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        
        let settings_json = serde_json::to_string_pretty(self).map_err(|e| e.to_string())?;
        fs::write(&settings_path, settings_json).map_err(|e| e.to_string())?;
        
        Ok(())
    }
    
    fn get_settings_path() -> PathBuf {
        let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push("text_log");
        path.push("settings.json");
        path
    }
}

type AppState = Arc<Mutex<AppSettings>>;

#[tauri::command]
fn get_directory(state: State<AppState>) -> Result<String, String> {
    let settings = state.lock().map_err(|e| e.to_string())?;
    Ok(settings.journal_directory.to_string_lossy().to_string())
}

#[tauri::command]
fn choose_directory(state: State<'_, AppState>, app: tauri::AppHandle) -> Result<(), String> {
    use tauri_plugin_dialog::{DialogExt};
    
    let state_clone = Arc::clone(&state.inner());
    
    app.dialog()
        .file()
        .pick_folder(move |folder_path| {
            if let Some(path) = folder_path {
                if let Ok(mut settings) = state_clone.lock() {
                    let path_buf = PathBuf::from(path.to_string());
                    settings.journal_directory = path_buf.clone();
                    let _ = settings.save();
                    
                    // Emit event to update frontend
                    let path_str = path_buf.to_string_lossy().to_string();
                    let _ = app.emit("directory-changed", &path_str);
                }
            }
        });
    
    Ok(())
}

#[tauri::command]
fn read_current_file(state: State<AppState>) -> Result<String, String> {
    // Get current directory from state (same logic as save_entry)
    let dir = {
        let settings = state.lock().map_err(|e| e.to_string())?;
        settings.journal_directory.clone()
    };
    
    // Create filename with date (same logic as save_entry)
    let filename = Local::now().format("%Y-%m-%d.txt").to_string();
    let filepath = dir.join(&filename);
    
    // Read file contents or return empty string if file doesn't exist
    match fs::read_to_string(&filepath) {
        Ok(contents) => Ok(contents),
        Err(_) => Ok(String::new()) // Return empty string if file doesn't exist
    }
}

#[tauri::command]
fn save_entry(content: String, state: State<AppState>) -> Result<String, String> {
    // Get timestamp
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    
    // Get current directory from state
    let dir = {
        let settings = state.lock().map_err(|e| e.to_string())?;
        settings.journal_directory.clone()
    };
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    
    // Create filename with date
    let filename = Local::now().format("%Y-%m-%d.txt").to_string();
    let filepath = dir.join(&filename);
    
    // Append to file with timestamp
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&filepath)
        .map_err(|e| e.to_string())?;
    
    writeln!(file, "[{}]", timestamp).map_err(|e| e.to_string())?;
    writeln!(file, "{}", content).map_err(|e| e.to_string())?;
    writeln!(file, "\n---\n").map_err(|e| e.to_string())?;
    
    Ok(filepath.to_string_lossy().to_string())
}

fn main() {
    let app_settings = Arc::new(Mutex::new(AppSettings::load()));
    
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .manage(app_settings)
        .invoke_handler(tauri::generate_handler![save_entry, get_directory, choose_directory, read_current_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
