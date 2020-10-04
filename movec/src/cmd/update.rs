use anyhow::Error;
use std::path::Path;
use crate::manifest::MoveToml;
use std::fs;

/// Execute update dependencies command.
pub fn execute(project_dir: &Path, manifest: MoveToml) -> Result<(), Error> {
    let cache_path = manifest
        .layout
        .as_ref()
        .and_then(|l| l.bytecode_cache.as_ref().map(|c| c.to_owned()))
        .ok_or_else(|| Error::msg("Expected layout cache path"))?;
    let cache_path = project_dir.join(cache_path);
    if cache_path.exists() {
        fs::remove_dir_all(&cache_path)?;
        fs::create_dir_all(&cache_path)?;
    }
    Ok(())
}
