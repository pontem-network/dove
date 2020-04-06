use std::fs;
use std::path::{Path, PathBuf};

use lsp_types::{Diagnostic, Position, Range};
use move_ir_types::location::Loc;
use move_lang::errors::{Error, FilesSourceText};
use move_lang::strip_comments_and_verify;
use move_lang::test_utils::MOVE_EXTENSION;

fn count_line_col(source: &str, pos: usize) -> (u64, u64) {
    let chars_before_pos = &source[..pos];

    let line = chars_before_pos
        .chars()
        .map(|chr| (chr == '\n') as u64)
        .sum::<u64>();

    let last_newline_pos = chars_before_pos.rfind(|chr| chr == '\n').unwrap_or(0);

    let col;
    if last_newline_pos == 0 {
        col = pos as u64;
    } else {
        col = (pos - last_newline_pos - 1) as u64
    }
    (line, col)
}

fn location_into_range(loc: Loc, source: &str) -> Range {
    let (line_start, col_start) = count_line_col(source, loc.span().start().to_usize());
    let (line_end, col_end) = count_line_col(source, loc.span().end().to_usize());
    Range::new(
        Position::new(line_start, col_start),
        Position::new(line_end, col_end),
    )
}

pub fn convert_error_into_diags(error: Error, source: &str) -> Vec<Diagnostic> {
    error
        .into_iter()
        .map(|(loc, message)| {
            let range = location_into_range(loc, source);
            Diagnostic::new_simple(range, message)
        })
        .collect()
}

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

fn get_stdlib_filenames(stdlib_path: &Path) -> Vec<String> {
    let dirfiles = iterate_directory(stdlib_path);
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

pub fn get_stdlib_files(stdlib_path: &Path) -> FilesSourceText {
    let stdlib_fnames = get_stdlib_filenames(stdlib_path)
        .iter()
        .map(|s| leak_str(s))
        .collect::<Vec<&'static str>>();

    let mut lib_files = FilesSourceText::with_capacity(stdlib_fnames.len());
    for mod_fname in stdlib_fnames {
        let mod_text = fs::read_to_string(mod_fname).unwrap().replace("\r\n", "\n");
        let stripped_mod_text = strip_comments_and_verify(mod_fname, &mod_text).unwrap();
        lib_files.insert(mod_fname, stripped_mod_text);
    }
    lib_files
}
