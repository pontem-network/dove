use std::path::PathBuf;

use crossbeam_channel::{unbounded, Receiver};
use ra_vfs::{Vfs, VfsChange, VfsTask};

use crate::fs::ws_root_vfs;
use analysis::analysis::{Analysis, AnalysisHost};
use analysis::change::AnalysisChange;
use analysis::config::Config;
use utils::leaked_fpath;

pub struct GlobalStateSnapshot {
    pub config: Config,
    pub analysis: Analysis,
}

#[derive(Debug)]
pub struct GlobalState {
    pub ws_root: PathBuf,
    pub config: Config,
    pub analysis_host: AnalysisHost,
    pub vfs: Vfs,
    pub fs_events_receiver: Receiver<VfsTask>,
}

impl GlobalState {
    pub fn new(
        ws_root: PathBuf,
        config: Config,
        vfs: Vfs,
        fs_events_receiver: Receiver<VfsTask>,
    ) -> GlobalState {
        let mut analysis_host = AnalysisHost::default();

        let mut change = AnalysisChange::new();
        change.change_config(config.clone());
        analysis_host.apply_change(change);

        GlobalState {
            ws_root,
            config,
            analysis_host,
            vfs,
            fs_events_receiver,
        }
    }

    pub fn load_fs_changes(&mut self) -> bool {
        let vfs_changes = self.vfs.commit_changes();
        if vfs_changes.is_empty() {
            return false;
        }
        let mut change = AnalysisChange::new();
        for fs_change in vfs_changes {
            match fs_change {
                VfsChange::AddFile { file, text, .. } => {
                    let fpath = leaked_fpath(self.vfs.file2path(file).to_str().unwrap());
                    change.add_file(fpath, text.to_string());
                }
                VfsChange::ChangeFile { file, text } => {
                    let path = leaked_fpath(self.vfs.file2path(file).to_str().unwrap());
                    change.update_file(path, text.to_string());
                }
                VfsChange::RemoveFile { file, path, .. } => {
                    let fpath = path.to_path(self.vfs.file2path(file));
                    let fpath = leaked_fpath(fpath.to_str().unwrap());
                    change.remove_file(fpath);
                }
                VfsChange::AddRoot { files, .. } => {
                    for (file, _, text) in files {
                        let fpath = leaked_fpath(self.vfs.file2path(file).to_str().unwrap());
                        change.add_file(fpath, text.to_string());
                    }
                }
            }
        }
        self.analysis_host.apply_change(change);
        true
    }

    pub fn snapshot(&self) -> GlobalStateSnapshot {
        GlobalStateSnapshot {
            config: self.config.clone(),
            analysis: self.analysis_host.analysis(),
        }
    }
}

pub fn initialize_new_global_state(ws_root: PathBuf, config: Config) -> GlobalState {
    let (fs_events_sender, fs_events_receiver) = unbounded::<VfsTask>();
    let vfs = ws_root_vfs(ws_root.clone(), fs_events_sender);
    GlobalState::new(ws_root, config, vfs, fs_events_receiver)
}
