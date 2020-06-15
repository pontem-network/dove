use crossbeam_channel::Sender;
use ra_vfs::{Filter, RelativePath, RootEntry, Vfs, VfsTask, Watch};
use std::path::PathBuf;

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

pub fn ws_root_vfs(ws_root: PathBuf, fs_events_sender: Sender<VfsTask>) -> Vfs {
    let modules_root = RootEntry::new(ws_root, Box::new(MoveFilesFilter::default()));
    Vfs::new(
        vec![modules_root],
        Box::new(move |task| fs_events_sender.send(task).unwrap()),
        Watch(true),
    )
    .0
}
