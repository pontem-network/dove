use std::path::PathBuf;
use crate::{MoveFile, io, MoveFilePath, leaked_fpath};

pub fn get_script_path() -> MoveFilePath {
    leaked_fpath(get_modules_path().join("script.move"))
}

// just need some valid fname
pub fn existing_module_file_abspath() -> MoveFilePath {
    let abspath = std::env::current_dir()
        .unwrap()
        .join("resources")
        .join("modules")
        .join("record.move")
        .into_os_string()
        .into_string()
        .unwrap();
    leaked_fpath(&abspath)
}

pub fn get_test_resources_dir() -> PathBuf {
    std::env::current_dir()
        .unwrap()
        .parent() // crates/
        .unwrap()
        .parent() // project root
        .unwrap()
        .join("resources")
        .join("tests")
}

pub fn get_stdlib_path() -> PathBuf {
    get_test_resources_dir().join("stdlib")
}

pub fn get_modules_path() -> PathBuf {
    get_test_resources_dir().join("modules")
}

pub fn setup_test_logging() {
    std::env::set_var("RUST_LOG", "info");
    // silently returns Err if called more than once
    env_logger::builder()
        .is_test(true)
        .try_init()
        .unwrap_or_default();
}

pub fn stdlib_mod(name: &str) -> MoveFile {
    io::load_move_file(get_stdlib_path().join(name)).unwrap()
}

pub fn modules_mod(name: &str) -> MoveFile {
    io::load_move_file(get_modules_path().join(name)).unwrap()
}

pub fn stdlib_transaction_mod() -> MoveFile {
    stdlib_mod("Transaction.move")
}

pub fn record_mod() -> MoveFile {
    io::load_move_file(get_modules_path().join("record.move")).unwrap()
}
