use core::fmt;
use serde::export::Formatter;
use std::cmp::min;
use utils::MoveFilePath;
use crate::inner::config::Config;

pub enum RootChange {
    AddFile(MoveFilePath, String),
    ChangeFile(MoveFilePath, String),
    RemoveFile(MoveFilePath),
}

impl fmt::Debug for RootChange {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut s = f.debug_struct("RootChange");
        match self {
            RootChange::AddFile(fpath, text) => s
                .field("fpath", fpath)
                .field("type", &String::from("AddFile"))
                .field("text", &text[0..min(text.len(), 55)].to_owned()),
            RootChange::RemoveFile(fpath) => s
                .field("fpath", fpath)
                .field("type", &String::from("RemoveFile")),
            RootChange::ChangeFile(fpath, text) => s
                .field("fpath", fpath)
                .field("type", &String::from("ChangeFile"))
                .field("text", &text[0..min(text.len(), 55)].to_owned()),
        };
        s.finish()
    }
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
