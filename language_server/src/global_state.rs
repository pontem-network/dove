use crate::main_loop::FileSystemEvent;
use crate::inner::config::Config;
use crate::inner::analysis::Analysis;
use crate::inner::db::RootDatabase;
use crate::inner::change::AnalysisChange;
use lang::compiler::file;

pub struct GlobalStateSnapshot {
    pub config: Config,
    pub analysis: Analysis,
}

#[derive(Debug)]
pub struct GlobalState {
    db: RootDatabase,
}

impl GlobalState {
    pub fn new(config: Config, initial_fs_events: Vec<FileSystemEvent>) -> GlobalState {
        let mut global_state = GlobalState {
            db: RootDatabase::new(config),
        };
        global_state.update_from_events(initial_fs_events);
        global_state
    }

    pub fn config(&self) -> &Config {
        &self.db.config
    }

    pub fn analysis(&self) -> Analysis {
        Analysis::new(self.db.clone())
    }

    pub fn update_from_events(&mut self, fs_events: Vec<FileSystemEvent>) {
        let mut change = AnalysisChange::new();
        for fs_event in fs_events {
            match fs_event {
                FileSystemEvent::AddFile(file) => {
                    let (fpath, text) = file.into();
                    change.add_file(fpath, text);
                }
                FileSystemEvent::ChangeFile(file) => {
                    let (fpath, text) = file.into();
                    change.update_file(fpath, text);
                }
                FileSystemEvent::RemoveFile(fpath) => {
                    change.remove_file(fpath);
                }
            }
        }
        log::info!("Applying change to the in-memory files db:\n{:#?}", &change);
        self.apply_change(change);
    }

    pub fn apply_change(&mut self, change: AnalysisChange) {
        self.db.apply_change(change);
    }

    pub fn snapshot(&self) -> GlobalStateSnapshot {
        GlobalStateSnapshot {
            config: self.config().clone(),
            analysis: self.analysis(),
        }
    }
}

pub fn initialize_new_global_state(config: Config) -> GlobalState {
    let mut initial_fs_events = vec![];
    match &config.stdlib_folder {
        Some(folder) => {
            for file in file::load_move_files(&[folder]).unwrap() {
                initial_fs_events.push(FileSystemEvent::AddFile(file));
            }
        }
        None => {}
    }
    for file in file::load_move_files(&config.modules_folders).unwrap() {
        initial_fs_events.push(FileSystemEvent::AddFile(file));
    }
    GlobalState::new(config, initial_fs_events)
}
