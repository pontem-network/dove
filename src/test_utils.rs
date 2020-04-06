use std::path::Path;

use move_lang::errors::FilesSourceText;

use crate::compiler::utils::get_module_files;

#[cfg(not(target_os = "windows"))]
pub const STDLIB_DIR: &str = r"./tests/stdlib";

#[cfg(target_os = "windows")]
pub const STDLIB_DIR: &str = r".\tests\stdlib";

pub fn load_stdlib_files() -> FilesSourceText {
    get_module_files(Path::new(STDLIB_DIR))
}
