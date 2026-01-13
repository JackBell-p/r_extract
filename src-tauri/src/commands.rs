use std::fs;

use tauri::{Manager, Window};
use tauri_plugin_dialog::DialogExt;

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

#[tauri::command]
pub async fn pick_folder(app: tauri::AppHandle, window: Window) -> String {
    let mut path = String::new();

    if let Some(folder) = app
        .dialog()
        .file()
        .set_parent(&window)
        .set_title("选择解压文件夹")
        .blocking_pick_folder()
    {
        path = folder.to_string();
    }

    path
}

#[tauri::command]
pub async fn pick_file(app: tauri::AppHandle, window: Window) -> String {
    let mut path = String::new();

    if let Some(file) = app
        .dialog()
        .file()
        .set_parent(&window)
        .set_title("选择要解压的文件")
        .add_filter("压缩文件", &["zip", "7z"])
        .blocking_pick_file()
    {
        path = file.to_string();
    }

    path
}
