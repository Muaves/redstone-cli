use std::fs;
pub fn setup() {
    let _ = fs::create_dir_all("launcher_data/versions");
    let _ = fs::create_dir_all("launcher_data/libraries");
    println!("Folders initialized.");
}
