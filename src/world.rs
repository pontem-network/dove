use std::fs;
use std::path::PathBuf;

use crate::analysis::{Analysis, AnalysisChange};
use crate::compiler::utils::get_module_files;
use crate::config::Config;

fn analysis_change_from_config(config: &Config) -> AnalysisChange {
    let mut change = AnalysisChange::new();
    change.change_sender_address(config.sender_address);

    for module_folder in config.module_folders.iter() {
        let module_folder = match fs::canonicalize(module_folder) {
            Ok(path) => path,
            Err(_) => {
                log::error!("Cannot resolve path {:?}", module_folder);
                continue;
            }
        };
        log::info!("Loading standard library from {:?}", &module_folder);
        for (fname, new_text) in get_module_files(&module_folder) {
            change.change_file(fname, new_text);
        }
    }
    change
}

#[derive(Debug, Default)]
pub struct WorldState {
    pub ws_root: PathBuf,
    pub config: Config,
    pub analysis: Analysis,
}

impl WorldState {
    pub fn new(ws_root: PathBuf, config: Config) -> Self {
        let change = analysis_change_from_config(&config);
        let mut analysis = Analysis::default();
        analysis.apply_change(change);
        WorldState {
            ws_root,
            config,
            analysis,
        }
    }

    pub fn update_configuration(&mut self, config: Config) {
        let change = analysis_change_from_config(&config);
        let mut new_analysis = Analysis::default();
        new_analysis.apply_change(change);
        self.analysis = new_analysis;
        self.config = config;
    }
}
