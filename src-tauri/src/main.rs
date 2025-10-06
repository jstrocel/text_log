// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use chrono::Local;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

#[tauri::command]
fn save_entry(content: String) -> Result<String, String> {
    // Get timestamp
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    
    // Create entries directory in user's home
    let mut dir = dirs::home_dir().ok_or("Could not find home directory")?;
    dir.push("MyJournal");
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
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![save_entry])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
