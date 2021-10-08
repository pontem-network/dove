use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Error;

use proto::file::{Diff, File as FileModel};
use rip_str::RipString;

#[derive(Debug)]
pub struct MFile {
    pub path: Arc<PathBuf>,
    pub content: RipString,
}

impl MFile {
    pub fn load(path: Arc<PathBuf>) -> Result<MFile, Error> {
        let content = RipString::from(fs::read_to_string(path.as_ref())?.as_str());
        Ok(MFile { path, content })
    }

    pub fn tp(&self) -> String {
        if let Some(ext) = self.path.extension() {
            ext.to_string_lossy().to_string()
        } else {
            "palaintext".to_string()
        }
    }

    pub fn update_file(&mut self, diff: Vec<Diff>) -> Result<(), Error> {
        for diff in diff {
            self.content.edit(
                diff.range_offset as usize..(diff.range_offset + diff.range_length) as usize,
                &diff.text,
            );
        }
        let mut f = OpenOptions::new()
            .read(true)
            .write(true)
            .open(self.path.as_ref())?;
        f.set_len(0)?;
        write!(f, "{}", &self.content)?;
        Ok(())
    }

    pub fn make_model(&self) -> FileModel {
        FileModel {
            content: self.content.to_string(),
            tp: self.tp(),
        }
    }
}
