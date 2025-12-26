use std::fs;

use tauri::Manager;

#[tauri::command]
pub fn exit(app: tauri::AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.close();
    }
}

#[tauri::command]
pub fn minimize(app: tauri::AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.minimize();
    }
}

#[tauri::command]
pub async fn get_file_size(path: String) -> u64 {
    fs::metadata(path).map(|m| m.len()).unwrap_or(0)
}
