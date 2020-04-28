//! Keeps track of file subscriptions -- the set of currently opened files for
//! which we want to publish diagnostics, syntax highlighting, etc.

use std::collections::HashSet;

use analysis::db::FilePath;

#[derive(Debug, Default, Clone)]
pub struct OpenedFiles {
    files: HashSet<FilePath>,
}

impl OpenedFiles {
    pub fn add(&mut self, fpath: FilePath) {
        self.files.insert(fpath);
    }
    pub fn remove(&mut self, fpath: FilePath) {
        self.files.remove(&fpath);
    }
    pub fn files(&self) -> Vec<FilePath> {
        self.files.iter().copied().collect()
    }
}
