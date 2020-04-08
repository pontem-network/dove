use std::fs;
use std::path::PathBuf;

use crate::analysis::{Analysis, AnalysisChange};
use crate::compiler::utils::get_module_files;
use crate::config::Config;

#[derive(Debug, Default)]
pub struct WorldState {
    pub ws_root: PathBuf,
    pub config: Config,
    pub analysis: Analysis,
}

impl WorldState {
    pub fn new(ws_root: PathBuf, config: Config) -> Self {
        WorldState {
            ws_root,
            config,
            analysis: Analysis::default(),
        }
    }

    pub fn with_modules_loaded(ws_root: PathBuf, config: Config) -> Self {
        let mut state = Self::new(ws_root, config);
        state.reload_available_module_files();
        state
    }

    fn reload_available_module_files(&mut self) {
        let mut change = AnalysisChange::new();
        for module_folder in &self.config.module_folders {
            let module_folder = match fs::canonicalize(module_folder) {
                Ok(path) => path,
                Err(_) => {
                    log::error!("Cannot resolve path {:?}", module_folder);
                    return;
                }
            };
            log::info!("Loading standard library from {:?}", &module_folder);
            for (fname, new_text) in get_module_files(&module_folder) {
                change.change_file(fname, new_text);
            }
        }
        self.analysis.apply_change(change);
    }

    pub fn update_configuration(&mut self, config: Config) {
        self.config = config;
        self.reload_available_module_files();
    }
}
