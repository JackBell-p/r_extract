pub mod sevenz;

#[tauri::command]
pub fn decompress(path: String, dest: String, ext: &str) -> bool {
    match ext {
        "7z" => sevenz::decompress_sevenz(path, dest),
        _ => false,
    }
}
