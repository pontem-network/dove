use std::path::Path;
use crate::manifest::MoveToml;
use std::fs;
use anyhow::Result;
use crate::dependence::loader::{Loader, RestBytecodeLoader};

/// Makes a RestBytecodeLoader with project path and project manifest.
pub fn make_rest_loader(
    project_dir: &Path,
    cmove: &MoveToml,
) -> Result<Option<Loader<RestBytecodeLoader>>> {
    let path = cmove
        .layout
        .as_ref()
        .and_then(|l| l.bytecode_cache.as_ref())
        .ok_or_else(|| anyhow!("Expected cache path"))?;
    let cache_path = project_dir.join(path);
    if !cache_path.exists() {
        fs::create_dir_all(&cache_path)?;
    }

    Ok(if let Some(uri) = cmove.package.blockchain_api.as_ref() {
        Some(Loader::new(
            Some(cache_path),
            RestBytecodeLoader::new(uri.parse()?),
        ))
    } else {
        None
    })
}
