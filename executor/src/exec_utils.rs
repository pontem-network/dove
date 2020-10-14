use utils::{MoveFilePath, FilesSourceText};

pub fn get_files_for_error_reporting(
    script: (MoveFilePath, String),
    deps: Vec<(MoveFilePath, String)>,
) -> FilesSourceText {
    let mut mapping = FilesSourceText::with_capacity(deps.len() + 1);
    for (fpath, text) in vec![script].into_iter().chain(deps.into_iter()) {
        mapping.insert(fpath, text);
    }
    mapping
}
