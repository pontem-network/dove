use std::path::{Path, PathBuf};
use anyhow::Result;
use walkdir::{DirEntry, IntoIter};

pub struct Files<'a, P: AsRef<Path>> {
    paths: &'a [P],
    walker_iter: Option<IntoIter>,
}

impl<'a, P: AsRef<Path>> Files<'a, P> {
    fn next_entry(&mut self) -> Option<Result<DirEntry>> {
        loop {
            if let Some(iter) = self.walker_iter.as_mut() {
                let item = iter.next();
                if let Some(entry) = item {
                    return Some(entry.map_err(|err| err.into()));
                }
            }

            if self.paths.is_empty() {
                return None;
            }

            self.walker_iter = Some(walkdir::WalkDir::new(&self.paths[0]).into_iter());
            self.paths = &self.paths[1..];
        }
    }
}

impl<'a, P: AsRef<Path>> Iterator for Files<'a, P> {
    type Item = PathBuf;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(entry) = self.next_entry() {
                if let Ok(entry) = entry {
                    if is_move_file(&entry) {
                        return Some(entry.into_path());
                    }
                }
            } else {
                return None;
            }
        }
    }
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

pub fn find_move_files<P>(paths: &[P]) -> Files<P>
where
    P: AsRef<Path>,
{
    Files {
        paths,
        walker_iter: None,
    }
}

pub fn find_move_files_with_filter<P, F>(path: P, filter: &F) -> Result<Vec<PathBuf>>
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
                        Some(Ok(path))
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
