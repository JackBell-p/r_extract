use tauri::Manager;

mod commands;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.set_maximizable(false);
                let _ = window.set_resizable(false);
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![commands::exit, commands::minimize])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
