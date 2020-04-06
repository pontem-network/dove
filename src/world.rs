use std::fs;
use std::path::PathBuf;

use move_lang::errors::FilesSourceText;

use crate::compiler::utils::get_stdlib_files;
use crate::config::MoveDialect;

#[derive(Debug)]
pub struct Config {
    pub dialect: MoveDialect,
    pub stdlib_path: Option<PathBuf>,
}

#[derive(Debug)]
pub struct WorldState {
    pub config: Config,
    pub stdlib_files: FilesSourceText,
}

impl WorldState {
    pub fn new(config: Config) -> Self {
        WorldState {
            config,
            stdlib_files: FilesSourceText::new(),
        }
    }

    pub fn new_with_stdlib_loaded(config: Config) -> Self {
        let mut state = Self::new(config);
        state.reload_stdlib();
        state
    }

    fn reload_stdlib(&mut self) {
        if self.config.stdlib_path.is_none() {
            self.stdlib_files = FilesSourceText::new();
            return;
        }
        let stdlib_path = self.config.stdlib_path.as_ref().unwrap();
        let canon_stdlib_path = match fs::canonicalize(stdlib_path) {
            Ok(path) => path,
            Err(_) => {
                log::error!("Cannot resolve path {:?}", stdlib_path);
                return;
            }
        };
        log::info!("Loading standard library from {:?}", &stdlib_path);
        self.stdlib_files = get_stdlib_files(&canon_stdlib_path);
    }

    pub fn update_configuration(&mut self, config: Config) {
        self.config = config;
        self.reload_stdlib();
    }
}
