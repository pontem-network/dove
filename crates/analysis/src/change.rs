use crate::config::Config;
use utils::MoveFilePath;

#[derive(Debug)]
pub enum RootChange {
    AddFile(MoveFilePath, String),
    ChangeFile(MoveFilePath, String),
    RemoveFile(MoveFilePath),
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

    pub fn add_file(&mut self, fname: MoveFilePath, text: String) {
        self.tracked_files_changed
            .push(RootChange::AddFile(fname, text));
    }

    pub fn update_file(&mut self, fname: MoveFilePath, text: String) {
        self.tracked_files_changed
            .push(RootChange::ChangeFile(fname, text));
    }

    pub fn remove_file(&mut self, fname: MoveFilePath) {
        self.tracked_files_changed
            .push(RootChange::RemoveFile(fname))
    }

    pub fn change_config(&mut self, config: Config) {
        self.config_changed = Some(config);
    }
}
