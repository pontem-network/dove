use crate::{leaked_fpath, MoveFile, MoveFilePath};
use anyhow::Result;
use std::path::{Path, PathBuf};
use std::{fs, io};

fn iterate_directory<P: AsRef<Path>>(path: P) -> impl Iterator<Item = PathBuf> {
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

pub fn iter_over_move_files<P: AsRef<Path>>(folder: P) -> Vec<String> {
    let dirfiles = iterate_directory(folder);
    dirfiles
        .flat_map(|path| {
            if path.extension()?.to_str()? == "move" {
                path.into_os_string().into_string().ok()
            } else {
                None
            }
        })
        .collect()
}

pub fn read_move_files<P: AsRef<Path>>(modules_folder: P) -> Vec<(MoveFilePath, String)> {
    let module_filenames = iter_over_move_files(modules_folder)
        .into_iter()
        .collect::<Vec<_>>();

    let mut lib_files = Vec::with_capacity(module_filenames.len());
    for mod_fname in module_filenames {
        let lib_file = match load_move_file(mod_fname.clone()) {
            Ok(text) => text,
            Err(io_error) => {
                log::warn!("Cannot read file {:?} ({}) skipping", io_error, mod_fname);
                continue;
            }
        };
        lib_files.push(lib_file);
    }
    lib_files
}

pub fn load_move_file<P: AsRef<Path>>(path: P) -> io::Result<MoveFile> {
    let fpath = leaked_fpath(path);
    let mut text = fs::read_to_string(fpath)?;
    ra_vfs::normalize_newlines(&mut text);
    Ok((fpath, text))
}

pub fn load_move_module_files(module_paths: Vec<PathBuf>) -> Result<Vec<MoveFile>> {
    let mut module_files = vec![];
    for module_path in module_paths {
        anyhow::ensure!(
            module_path.exists(),
            "Cannot open {:?}: No such file or directory",
            module_path
        );
        if module_path.is_file() {
            let module_file = load_move_file(module_path)?;
            module_files.push(module_file);
        } else {
            for module_file in read_move_files(module_path) {
                module_files.push(module_file);
            }
        }
    }
    Ok(module_files)
}
