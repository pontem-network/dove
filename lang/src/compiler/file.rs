use anyhow::{Result, Error};
use std::fs::File;
use std::io::Read;
use std::path::{PathBuf, Path};
use std::convert::TryFrom;
use walkdir::DirEntry;
use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq)]
pub struct MoveFile {
    name: String,
    content: String,
}

impl MoveFile {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<MoveFile> {
        let path = path.as_ref();
        let mut f = File::open(&path)
            .map_err(|err| std::io::Error::new(err.kind(), format!("{}: {:?}", err, path)))?;
        let mut content = String::new();
        f.read_to_string(&mut content)?;

        let name = path
            .to_str()
            .map(|path| path.to_owned())
            .ok_or_else(|| anyhow!("Failed to convert source path"))?;

        Ok(MoveFile { name, content })
    }

    pub fn with_content<'s, 's1, S, S1>(name: S, content: S1) -> MoveFile
    where
        S: Into<Cow<'s, str>>,
        S1: Into<Cow<'s1, str>>,
    {
        // TODO replace String with Cow<'s, str>
        MoveFile {
            name: name.into().into_owned(),
            content: content.into().into_owned(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn into(self) -> (String, String) {
        (self.name, self.content)
    }
}

impl TryFrom<&PathBuf> for MoveFile {
    type Error = Error;

    fn try_from(value: &PathBuf) -> Result<Self, Self::Error> {
        MoveFile::load(value)
    }
}

pub fn load_move_files<P: AsRef<Path>>(paths: &[P]) -> Result<Vec<MoveFile>> {
    let mut module_files = vec![];
    for path in paths {
        let path = path.as_ref();
        anyhow::ensure!(
            path.exists(),
            "Cannot open {:?}: No such file or directory",
            path
        );
        if path.is_file() {
            module_files.push(MoveFile::load(path)?);
        } else {
            for path in find_move_files(path)? {
                module_files.push(MoveFile::load(path)?);
            }
        }
    }
    Ok(module_files)
}

fn is_move_file(entry: &DirEntry) -> bool {
    entry.file_type().is_file()
        && !entry
            .file_name()
            .to_str()
            .map(|name| name.starts_with('.'))
            .unwrap_or(true)
        && entry
            .path()
            .extension()
            .map(|ext| ext.eq("move"))
            .unwrap_or(false)
}

pub fn find_move_files<P: AsRef<Path>>(path: P) -> Result<Vec<String>> {
    walkdir::WalkDir::new(path)
        .into_iter()
        .filter_map(|entry| match entry {
            Ok(entry) => {
                if is_move_file(&entry) {
                    Some(
                        entry
                            .into_path()
                            .into_os_string()
                            .into_string()
                            .map_err(|path| anyhow!("Failed to convert path:{:?}", path)),
                    )
                } else {
                    None
                }
            }
            Err(err) => Some(Err(err.into())),
        })
        .collect()
}
