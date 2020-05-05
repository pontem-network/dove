use std::fs;
use std::path::{Path, PathBuf};

use move_lang::test_utils::MOVE_EXTENSION;

use crate::db::FilePath;

pub fn leaked_fpath<P: AsRef<Path>>(path: P) -> FilePath {
    let s = path.as_ref().to_str().unwrap();
    Box::leak(Box::new(s.to_owned()))
}

fn iterate_directory(path: &Path) -> impl Iterator<Item = PathBuf> {
    walkdir::WalkDir::new(path)
        .into_iter()
        .map(::std::result::Result::unwrap)
        .filter(|entry| {
            entry.file_type().is_file()
                && entry
                    .file_name()
                    .to_str()
                    .map_or(false, |s| !s.starts_with('.')) // Skip hidden files
        })
        .map(|entry| entry.path().to_path_buf())
}

fn get_module_filenames(folder: &Path) -> Vec<String> {
    let dirfiles = iterate_directory(folder);
    dirfiles
        .flat_map(|path| {
            if path.extension()?.to_str()? == MOVE_EXTENSION {
                path.into_os_string().into_string().ok()
            } else {
                None
            }
        })
        .collect()
}

pub fn get_module_files(modules_folder: &Path) -> Vec<(FilePath, String)> {
    let module_filenames = get_module_filenames(modules_folder)
        .iter()
        .map(leaked_fpath)
        .collect::<Vec<_>>();

    let mut lib_files = Vec::with_capacity(module_filenames.len());
    for mod_fname in module_filenames {
        let mod_text = match fs::read_to_string(mod_fname) {
            Ok(text) => text.replace("\r\n", "\n"),
            Err(_) => {
                log::warn!("Cannot read file {:?}, skipping", mod_fname);
                continue;
            }
        };
        lib_files.push((mod_fname, mod_text));
    }
    lib_files
}

pub fn get_canonical_fname<P: AsRef<Path>>(path: P) -> &'static str {
    leaked_fpath(fs::canonicalize(path).unwrap().to_str().unwrap())
}
