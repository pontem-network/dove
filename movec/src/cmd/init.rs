use anyhow::Error;
use http::Uri;
use std::path::Path;
use crate::manifest::{MANIFEST, MoveToml, store_manifest, Layout};
use std::fs;

/// Execute init command.
pub fn execute(
    project_dir: &Path,
    source_dir: String,
    repository: Option<Uri>,
    address: Option<String>,
) -> Result<(), Error> {
    let project_dir = project_dir.join(&source_dir);
    if !project_dir.exists() {
        return Err(anyhow!("destination `{:?}` not found.", project_dir));
    }

    let cmove_path = project_dir.join(MANIFEST);
    if cmove_path.exists() {
        return Err(anyhow!("destination `{:?}` already exists", cmove_path));
    }

    let mut cmove = MoveToml::default();
    cmove.package.name = Some(source_dir);
    cmove.package.blockchain_api = repository.map(|uri| uri.to_string());
    cmove.package.account_address = address;

    store_manifest(&project_dir.join(MANIFEST), cmove)?;
    let mut layout = Layout::default();
    layout.fill();
    let module_dir = project_dir.join(layout.module_dir.unwrap());
    fs::create_dir_all(&module_dir)?;
    let script_dir = project_dir.join(layout.script_dir.unwrap());
    fs::create_dir_all(&script_dir)?;
    Ok(())
}
