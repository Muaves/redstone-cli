use std::fs;

pub fn setup() {
    fs::create_dir_all("launcher_data/versions").unwrap();
    fs::create_dir_all("launcher_data/libraries").unwrap();
    fs::create_dir_all("launcher_data/assets").unwrap();
}

