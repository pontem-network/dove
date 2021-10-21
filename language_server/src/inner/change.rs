use core::fmt;
use std::cmp::min;
use crate::inner::config::Config;
use std::fs::read_to_string;

pub enum RootChange {
    AddFile { path: String },
    ChangeFile { path: String },
    RemoveFile { path: String },
}

impl fmt::Debug for RootChange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn text(path: &str) -> String {
            read_to_string(&path)
                .map(|text| { &text[0..min(text.len(), 55)] }.to_string())
                .unwrap_or_else(|_| "file not found".to_string())
        }

        let mut s = f.debug_struct("RootChange");
        match self {
            RootChange::AddFile { path } => s
                .field("fpath", path)
                .field("type", &String::from("AddFile"))
                .field("text", &text(path)),
            RootChange::RemoveFile { path } => s
                .field("fpath", path)
                .field("type", &String::from("RemoveFile")),
            RootChange::ChangeFile { path } => s
                .field("fpath", path)
                .field("type", &String::from("ChangeFile"))
                .field("text", &text(path)),
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

    pub fn add_file(&mut self, fname: String) {
        self.tracked_files_changed
            .push(RootChange::AddFile { path: fname });
    }

    pub fn update_file(&mut self, fname: String) {
        self.tracked_files_changed
            .push(RootChange::ChangeFile { path: fname });
    }

    pub fn remove_file(&mut self, fname: String) {
        self.tracked_files_changed
            .push(RootChange::RemoveFile { path: fname })
    }

    pub fn change_config(&mut self, config: Config) {
        self.config_changed = Some(config);
    }
}
