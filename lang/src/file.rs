use anyhow::{Result, Error};
use std::fs::File;
use std::io::Read;
use std::path::{PathBuf, Path};
use std::convert::TryFrom;
use walkdir::DirEntry;

#[derive(Debug, Clone)]
pub struct MvFile {
    name: String,
    content: String,
}

impl MvFile {
    pub fn with_path(path: String) -> Result<MvFile> {
        let mut f = File::open(&path)
            .map_err(|err| std::io::Error::new(err.kind(), format!("{}: {}", err, path)))?;
        let mut source_buffer = String::new();
        f.read_to_string(&mut source_buffer)?;

        Ok(MvFile {
            name: path,
            content: source_buffer,
        })
    }

    pub fn with_content(name: String, content: String) -> MvFile {
        MvFile { name, content }
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

impl TryFrom<&PathBuf> for MvFile {
    type Error = Error;

    fn try_from(value: &PathBuf) -> Result<Self, Self::Error> {
        MvFile::try_from(value.as_path())
    }
}

impl TryFrom<&Path> for MvFile {
    type Error = Error;

    fn try_from(value: &Path) -> Result<Self, Self::Error> {
        let path = value
            .to_str()
            .map(|path| path.to_owned())
            .ok_or_else(|| anyhow!("Failed to convert source path"))?;
        MvFile::with_path(path)
    }
}

pub fn load_move_files<P: AsRef<Path>>(paths: &[P]) -> Result<Vec<MvFile>> {
    let mut module_files = vec![];
    for path in paths {
        let path = path.as_ref();
        anyhow::ensure!(
            path.exists(),
            "Cannot open {:?}: No such file or directory",
            path
        );
        if path.is_file() {
            module_files.push(MvFile::try_from(path)?);
        } else {
            for path in find_move_files(path)? {
                module_files.push(MvFile::with_path(path)?);
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
            .map(|name| name.starts_with("."))
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
