use std::borrow::Cow;
use std::convert::TryFrom;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

use anyhow::{Error, Result};
use walkdir::DirEntry;

#[derive(Debug, Clone, PartialEq)]
pub struct MoveFile<'n, 'c> {
    name: Cow<'n, str>,
    content: Cow<'c, str>,
}

impl<'n, 'c> MoveFile<'n, 'c> {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<MoveFile<'static, 'static>> {
        let path = path.as_ref();
        let mut f = File::open(&path)
            .map_err(|err| std::io::Error::new(err.kind(), format!("{}: {:?}", err, path)))?;
        let mut content = String::new();
        f.read_to_string(&mut content)?;

        let name = path
            .to_str()
            .map(|path| path.to_owned())
            .ok_or_else(|| anyhow!("Failed to convert source path"))?;

        Ok(MoveFile {
            name: Cow::Owned(name),
            content: Cow::Owned(content),
        })
    }

    pub fn with_content<N, C>(name: N, content: C) -> MoveFile<'n, 'c>
    where
        N: Into<Cow<'n, str>>,
        C: Into<Cow<'c, str>>,
    {
        MoveFile {
            name: name.into(),
            content: content.into(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn into(self) -> (String, String) {
        (self.name.into_owned(), self.content.into_owned())
    }
}

impl TryFrom<&PathBuf> for MoveFile<'static, 'static> {
    type Error = Error;

    fn try_from(value: &PathBuf) -> Result<Self, Self::Error> {
        MoveFile::load(value)
    }
}
// TODO refactoring. Make iterator like api to work with move files.

pub fn load_move_files_with_filter<P, F>(
    paths: &[P],
    filter: &F,
) -> Result<Vec<MoveFile<'static, 'static>>>
where
    F: Fn(&Path) -> bool,
    P: AsRef<Path>,
{
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
            for path in find_move_files_with_filter(path, filter)? {
                module_files.push(MoveFile::load(path)?);
            }
        }
    }
    Ok(module_files)
}

pub fn load_move_files<P: AsRef<Path>>(paths: &[P]) -> Result<Vec<MoveFile<'static, 'static>>> {
    load_move_files_with_filter(paths, &|_| true)
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

pub fn find_move_files_with_filter<P, F>(path: P, filter: &F) -> Result<Vec<String>>
where
    P: AsRef<Path>,
    F: Fn(&Path) -> bool,
{
    walkdir::WalkDir::new(path)
        .into_iter()
        .filter_map(|entry| match entry {
            Ok(entry) => {
                if is_move_file(&entry) {
                    let path = entry.into_path();
                    if filter(&path) {
                        Some(
                            path.into_os_string()
                                .into_string()
                                .map_err(|path| anyhow!("Failed to convert path:{:?}", path)),
                        )
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            Err(err) => Some(Err(err.into())),
        })
        .collect()
}

pub fn find_move_files<P: AsRef<Path>>(path: P) -> Result<Vec<String>> {
    find_move_files_with_filter(path, &|_| true)
}
