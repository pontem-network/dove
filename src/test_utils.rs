use std::path::PathBuf;

use move_lang::errors::FilesSourceText;

use crate::compiler::utils::get_module_files;

fn get_tests_dir() -> PathBuf {
    PathBuf::from(".").join("tests")
}

pub fn get_stdlib_path() -> PathBuf {
    get_tests_dir().join("stdlib")
}

pub fn get_modules_path() -> PathBuf {
    get_tests_dir().join("modules")
}

pub fn load_stdlib_files() -> FilesSourceText {
    get_module_files(get_stdlib_path().as_path())
}
