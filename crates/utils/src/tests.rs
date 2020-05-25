use crate::{leaked_fpath, FilePath};
use std::path::PathBuf;

pub fn get_script_path() -> FilePath {
    leaked_fpath(get_modules_path().join("script.move"))
}

// just need some valid fname
pub fn existing_file_abspath() -> FilePath {
    let abspath = std::env::current_dir()
        .unwrap()
        .join("resources")
        .join("modules.rs")
        .join("covid_tracker.move")
        .into_os_string()
        .into_string()
        .unwrap();
    leaked_fpath(&abspath)
}

pub fn get_resources_dir() -> PathBuf {
    std::env::current_dir()
        .unwrap()
        .parent() // crates/
        .unwrap()
        .parent() // project root
        .unwrap()
        .join("resources")
}

pub fn get_stdlib_path() -> PathBuf {
    get_resources_dir().join("stdlib")
}

pub fn get_modules_path() -> PathBuf {
    get_resources_dir().join("modules")
}

pub fn setup_test_logging() {
    std::env::set_var("RUST_LOG", "info");
    // silently returns Err if called more than once
    env_logger::builder()
        .is_test(true)
        .try_init()
        .unwrap_or_default();
}
