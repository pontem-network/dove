use std::fs;
use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Error;
use strings::rope::Rope;

use proto::file::{Diff, File as FileModel};
use std::fs::OpenOptions;
use std::io::Write;

#[derive(Debug)]
pub struct MFile {
    pub path: Arc<PathBuf>,
    pub content: Rope,
}

impl MFile {
    pub fn load(path: Arc<PathBuf>) -> Result<MFile, Error> {
        let content = Rope::from_string(fs::read_to_string(path.as_ref())?);
        Ok(MFile {
            path,
            content,
        })
    }

    pub fn tp(&self) -> String {
        if let Some(ext) = self.path.extension() {
            ext.to_string_lossy().to_string()
        } else {
            "palaintext".to_string()
        }
    }

    pub fn update_file(&mut self, diff: Vec<Diff>) -> Result<(), Error> {
        println!("\"\n{}\n\"", self.content);
        println!("diff:{:?}", diff);
        for diff in diff {
            if diff.text.is_empty() {
                self.content.remove(diff.range_offset as usize, (diff.range_offset + diff.range_length) as usize);
            } else {
                if diff.range_length != 0 {
                    self.content.remove(diff.range_offset as usize, (diff.range_offset + diff.range_length) as usize);
                }

                self.content.insert(diff.range_offset as usize, diff.text);
            }
        }
        println!("\'\n{}\n\'", self.content);

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

#[test]
fn test() {
    let mut r = Rope::from_string(
        "ww\nww".to_string()
    );
    r.remove(0, 2);
    println!("\'\n{}\n\'", r);
}