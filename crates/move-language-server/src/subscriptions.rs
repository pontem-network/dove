//! Keeps track of file subscriptions -- the set of currently opened files for
//! which we want to publish diagnostics, syntax highlighting, etc.

use std::collections::HashSet;
use utils::MoveFilePath;

#[derive(Debug, Default, Clone)]
pub struct OpenedFiles {
    files: HashSet<MoveFilePath>,
}

impl OpenedFiles {
    pub fn add(&mut self, fpath: MoveFilePath) {
        self.files.insert(fpath);
    }
    pub fn remove(&mut self, fpath: MoveFilePath) {
        self.files.remove(&fpath);
    }
    pub fn files(&self) -> Vec<MoveFilePath> {
        self.files.iter().copied().collect()
    }
}
