use analysis::change::AnalysisChange;
use analysis::config::Config;
use move_language_server::global_state::{initialize_new_global_state, GlobalStateSnapshot};
use std::path::PathBuf;
use utils::io::read_move_files;
use utils::{leaked_fpath, MoveFile, MoveFilePath, io};

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

pub fn global_state_snapshot(
    file: MoveFile,
    config: Config,
    additional_files: Vec<MoveFile>,
) -> GlobalStateSnapshot {
    let ws_root = std::env::current_dir().unwrap();
    let mut global_state = initialize_new_global_state(ws_root, config);
    let mut change = AnalysisChange::new();

    for folder in &global_state.config.modules_folders {
        for (fpath, text) in read_move_files(folder) {
            change.add_file(fpath, text);
        }
    }

    for (fpath, text) in additional_files {
        change.add_file(fpath, text);
    }

    change.update_file(file.0, file.1);

    global_state.analysis_host.apply_change(change);
    global_state.snapshot()
}

#[macro_export]
macro_rules! config {
    () => {{
        analysis::config::Config::default()
    }};
    ($json: tt) => {{
        let config_json = serde_json::json!($json);
        let mut config = analysis::config::Config::default();
        config.update(&config_json);
        config
    }};
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
