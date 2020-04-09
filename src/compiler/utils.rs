use std::fs;
use std::path::{Path, PathBuf};

use move_lang::errors::FilesSourceText;
use move_lang::strip_comments_and_verify;
use move_lang::test_utils::MOVE_EXTENSION;

pub fn leak_str(s: &str) -> &'static str {
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

pub fn get_module_files(modules_folder: &Path) -> FilesSourceText {
    let module_filenames = get_module_filenames(modules_folder)
        .iter()
        .map(|s| leak_str(s))
        .collect::<Vec<&'static str>>();

    let mut lib_files = FilesSourceText::with_capacity(module_filenames.len());
    for mod_fname in module_filenames {
        let mod_text = fs::read_to_string(mod_fname).unwrap().replace("\r\n", "\n");
        let stripped_mod_text = strip_comments_and_verify(mod_fname, &mod_text).unwrap();
        lib_files.insert(mod_fname, stripped_mod_text);
    }
    lib_files
}

pub fn get_canonical_fname<P: AsRef<Path>>(path: P) -> &'static str {
    leak_str(fs::canonicalize(path).unwrap().to_str().unwrap())
}
