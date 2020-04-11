use std::fs;
use std::path::{Path, PathBuf};

use move_lang::test_utils::MOVE_EXTENSION;

use crate::ide::db::FilePath;

pub fn leaked_fpath(s: &str) -> FilePath {
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
        .map(|s| leaked_fpath(s))
        .collect::<Vec<&'static str>>();

    let mut lib_files = Vec::with_capacity(module_filenames.len());
    for mod_fname in module_filenames {
        let mod_text = fs::read_to_string(mod_fname).unwrap().replace("\r\n", "\n");
        lib_files.push((mod_fname, mod_text));
        // let stripped_mod_text = strip_comments_and_verify(mod_fname, &mod_text).unwrap();
        // lib_files.insert(mod_fname, stripped_mod_text);
    }
    lib_files
}

pub fn get_canonical_fname<P: AsRef<Path>>(path: P) -> &'static str {
    leaked_fpath(fs::canonicalize(path).unwrap().to_str().unwrap())
}
