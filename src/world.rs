use std::fs;
use std::path::PathBuf;

use move_lang::errors::FilesSourceText;

use crate::compiler::utils::get_module_files;
use crate::config::Config;

#[derive(Debug)]
pub struct WorldState {
    pub ws_root: PathBuf,
    pub config: Config,
    pub available_module_files: FilesSourceText,
}

impl WorldState {
    pub fn new(ws_root: PathBuf, config: Config) -> Self {
        WorldState {
            ws_root,
            config,
            available_module_files: FilesSourceText::new(),
        }
    }

    pub fn with_modules_loaded(ws_root: PathBuf, config: Config) -> Self {
        let mut state = Self::new(ws_root, config);
        state.reload_available_module_files();
        state
    }

    fn reload_available_module_files(&mut self) {
        let mut module_files = FilesSourceText::new();
        for module_folder in &self.config.module_folders {
            let module_folder = match fs::canonicalize(module_folder) {
                Ok(path) => path,
                Err(_) => {
                    log::error!("Cannot resolve path {:?}", module_folder);
                    return;
                }
            };
            log::info!("Loading standard library from {:?}", &module_folder);
            module_files.extend(get_module_files(&module_folder));
        }
        self.available_module_files = module_files;
    }

    pub fn update_configuration(&mut self, config: Config) {
        self.config = config;
        self.reload_available_module_files();
    }
}
