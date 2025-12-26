pub fn decompress_sevenz(path: String, dest: String) -> bool {
    match sevenz_rust2::decompress_file(path, dest) {
        Ok(()) => true,
        Err(_) => false,
    }
}
