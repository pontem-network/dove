use std::fs;
use std::path::PathBuf;

use anyhow::Result;

use analysis::db::FilePath;
use analysis::utils::io::{get_module_files, leaked_fpath};

pub(crate) fn load_module_files(module_paths: Vec<PathBuf>) -> Result<Vec<(FilePath, String)>> {
    let mut deps = vec![];
    for module_path in module_paths {
        anyhow::ensure!(
            module_path.exists(),
            "Cannot open {:?}: No such file or directory",
            module_path
        );
        if module_path.is_file() {
            let fpath = leaked_fpath(module_path.to_str().unwrap());
            let text = fs::read_to_string(fpath).unwrap();
            deps.push((fpath, text));
        } else {
            for dep in get_module_files(module_path.as_path()) {
                deps.push(dep);
            }
        }
    }
    Ok(deps)
}
