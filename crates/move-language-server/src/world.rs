use std::path::PathBuf;

use crossbeam_channel::{unbounded, Receiver};
use ra_vfs::{Filter, RelativePath, RootEntry, Vfs, VfsChange, VfsTask, Watch};

use analysis::analysis::{Analysis, AnalysisHost};
use analysis::change::AnalysisChange;
use analysis::config::Config;
use analysis::utils::io::leaked_fpath;

#[derive(Default)]
struct MoveFilesFilter;

impl Filter for MoveFilesFilter {
    fn include_dir(&self, _: &RelativePath) -> bool {
        true
    }

    fn include_file(&self, file_path: &RelativePath) -> bool {
        file_path.extension() == Some("move")
    }
}

pub struct WorldSnapshot {
    pub config: Config,
    pub analysis: Analysis,
}

#[derive(Debug)]
pub struct WorldState {
    pub ws_root: PathBuf,
    pub config: Config,
    pub analysis_host: AnalysisHost,
    pub vfs: Vfs,
    pub fs_events_receiver: Receiver<VfsTask>,
}

impl WorldState {
    pub fn new(ws_root: PathBuf, config: Config) -> WorldState {
        let mut analysis_host = AnalysisHost::default();

        let mut change = AnalysisChange::new();
        change.change_config(config.clone());
        analysis_host.apply_change(change);

        let (fs_events_sender, fs_events_receiver) = unbounded::<VfsTask>();
        let modules_root = RootEntry::new(ws_root.clone(), Box::new(MoveFilesFilter::default()));
        let vfs = Vfs::new(
            vec![modules_root],
            Box::new(move |task| fs_events_sender.send(task).unwrap()),
            Watch(true),
        )
        .0;

        WorldState {
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

    pub fn snapshot(&self) -> WorldSnapshot {
        WorldSnapshot {
            config: self.config.clone(),
            analysis: self.analysis_host.analysis(),
        }
    }
}
