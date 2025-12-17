use tauri::Manager;

#[tauri::command]
pub fn exit(app: tauri::AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.close();
    }
}
