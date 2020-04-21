use lsp_types::{Diagnostic, DiagnosticRelatedInformation, Location, Range, Url};
use move_ir_types::location::Loc;
use move_lang::errors::{Error, FilesSourceText};

use move_lang::shared::Address;

use crate::config::Config;
use crate::utils::location::File;

pub type FilePath = &'static str;

#[derive(Debug)]
pub struct FileDiagnostic {
    pub fpath: FilePath,
    pub diagnostic: Option<Diagnostic>,
}

impl FileDiagnostic {
    pub fn new(fpath: FilePath, diagnostic: Diagnostic) -> FileDiagnostic {
        FileDiagnostic {
            fpath,
            diagnostic: Some(diagnostic),
        }
    }

    pub fn new_empty(fpath: FilePath) -> FileDiagnostic {
        FileDiagnostic {
            fpath,
            diagnostic: None,
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct RootDatabase {
    pub config: Config,
    pub tracked_files: FilesSourceText,
}

impl RootDatabase {
    pub fn module_files(&self) -> FilesSourceText {
        self.tracked_files
            .clone()
            .into_iter()
            .filter(|(f, _)| self.is_fpath_for_a_module(f))
            .collect()
    }

    pub fn sender_address(&self) -> Address {
        self.config.sender_address
    }

    pub fn apply_change(&mut self, change: AnalysisChange) {
        if let Some(config) = change.config_changed {
            self.config = config;
        }
        for root_change in change.tracked_files_changed {
            match root_change {
                RootChange::AddFile(fpath, text) => {
                    log::info!("AddFile: {:?}", fpath);
                    self.tracked_files.insert(fpath, text);
                }
                RootChange::ChangeFile(fpath, text) => {
                    log::info!("ChangeFile: {:?}", fpath);
                    self.tracked_files.insert(fpath, text);
                }
                RootChange::RemoveFile(fpath) => {
                    if !self.tracked_files.contains_key(fpath) {
                        log::warn!("RemoveFile: file {:?} does not exist", fpath);
                    }
                    log::info!("RemoveFile: {:?}", fpath);
                    self.tracked_files.remove(fpath);
                }
            }
        }
    }

    pub fn libra_error_into_diagnostic(&self, error: Error) -> FileDiagnostic {
        assert!(!error.is_empty(), "Libra's Error is an empty Vec");
        let (primary_loc, primary_message) = error.get(0).unwrap().to_owned();
        let mut diagnostic = {
            let range = self.loc_to_range(primary_loc);
            Diagnostic::new_simple(range, primary_message)
        };
        // first error is an actual one, others are related info
        if error.len() > 1 {
            let mut related_info = vec![];
            for (related_loc, related_message) in error[1..].iter() {
                let range = self.loc_to_range(*related_loc);
                let related_fpath = related_loc.file();
                let file_uri = Url::from_file_path(related_fpath)
                    .unwrap_or_else(|_| panic!("Cannot build Url from path {:?}", related_fpath));

                let related_info_item = DiagnosticRelatedInformation {
                    location: Location::new(file_uri, range),
                    message: related_message.to_string(),
                };
                related_info.push(related_info_item);
            }
            diagnostic.related_information = Some(related_info)
        }
        FileDiagnostic::new(primary_loc.file(), diagnostic)
    }

    fn loc_to_range(&self, loc: Loc) -> Range {
        let text = self.tracked_files.get(loc.file()).unwrap().to_owned();
        let file = File::new(text);
        let start_pos = file.position(loc.span().start().to_usize()).unwrap();
        let end_pos = file.position(loc.span().end().to_usize()).unwrap();
        Range::new(start_pos, end_pos)
    }

    fn is_fpath_for_a_module(&self, fpath: FilePath) -> bool {
        for module_folder in self.config.module_folders.iter() {
            if fpath.starts_with(module_folder.to_str().unwrap()) {
                return true;
            }
        }
        false
    }
}

#[derive(Debug)]
pub enum RootChange {
    AddFile(FilePath, String),
    ChangeFile(FilePath, String),
    RemoveFile(FilePath),
}

#[derive(Default, Debug)]
pub struct AnalysisChange {
    tracked_files_changed: Vec<RootChange>,
    config_changed: Option<Config>,
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
