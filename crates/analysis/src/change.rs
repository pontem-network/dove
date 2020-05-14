use crate::config::Config;
use utils::FilePath;

#[derive(Debug)]
pub enum RootChange {
    AddFile(FilePath, String),
    ChangeFile(FilePath, String),
    RemoveFile(FilePath),
}

#[derive(Default, Debug)]
pub struct AnalysisChange {
    pub(crate) tracked_files_changed: Vec<RootChange>,
    pub(crate) config_changed: Option<Config>,
}

impl AnalysisChange {
    pub fn new() -> Self {
        AnalysisChange::default()
    }

    pub fn add_file(&mut self, fname: FilePath, text: String) {
        self.tracked_files_changed
            .push(RootChange::AddFile(fname, text));
    }

    pub fn update_file(&mut self, fname: FilePath, text: String) {
        self.tracked_files_changed
            .push(RootChange::ChangeFile(fname, text));
    }

    pub fn remove_file(&mut self, fname: FilePath) {
        self.tracked_files_changed
            .push(RootChange::RemoveFile(fname))
    }

    pub fn change_config(&mut self, config: Config) {
        self.config_changed = Some(config);
    }
}
