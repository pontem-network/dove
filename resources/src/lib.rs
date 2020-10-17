use std::path::PathBuf;

pub fn asset(str: &str) -> PathBuf {
    assets_dir().join(str)
}

pub fn modules_path() -> PathBuf {
    assets_dir().join("modules")
}

pub fn stdlib_path() -> PathBuf {
    assets_dir().join("stdlib")
}

pub fn assets_dir() -> PathBuf {
    std::env::current_dir()
        .unwrap()
        .parent()
        .unwrap()
        .join("resources")
        .join("assets")
}
