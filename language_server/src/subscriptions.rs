//! Keeps track of file subscriptions -- the set of currently opened files for
//! which we want to publish diagnostics, syntax highlighting, etc.

use std::collections::HashSet;

#[derive(Debug, Default, Clone)]
pub struct OpenedFiles {
    files: HashSet<String>,
}

impl OpenedFiles {
    pub fn add(&mut self, fpath: String) {
        self.files.insert(fpath);
    }
    pub fn remove(&mut self, fpath: String) {
        self.files.remove(&fpath);
    }
    pub fn files(&self) -> &HashSet<String> {
        &self.files
    }
}
