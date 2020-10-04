use anyhow::Error;
use http::Uri;
use std::path::Path;
use std::fs;
use crate::cmd::init;

/// Execute create project command.
pub fn execute(
    root: &Path,
    source_dir: String,
    repository: Option<Uri>,
    address: Option<String>,
) -> Result<(), Error> {
    let project_dir = root.join(&source_dir);
    if project_dir.exists() {
        return Err(anyhow!("destination `{:?}` already exists", project_dir));
    }
    fs::create_dir(&project_dir)?;
    init::execute(root, source_dir, repository, address)
}
