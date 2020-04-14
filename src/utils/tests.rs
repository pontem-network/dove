use std::path::PathBuf;

pub fn get_tests_dir() -> PathBuf {
    std::env::current_dir().unwrap().join("tests")
}

pub fn get_stdlib_path() -> PathBuf {
    get_tests_dir().join("stdlib")
}

pub fn get_modules_path() -> PathBuf {
    get_tests_dir().join("modules")
}

pub fn setup_test_logging() {
    std::env::set_var("RUST_LOG", "info");
    // silently returns Err if called more than once
    env_logger::builder()
        .is_test(true)
        .try_init()
        .unwrap_or_default();
}
