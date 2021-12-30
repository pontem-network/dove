use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::fs;
use proc_macro::TokenStream;
use anyhow::{Result};
use log::debug;
use toml::Value;

/// Get the project version and a short hash
#[proc_macro]
pub fn version(item: TokenStream) -> TokenStream {
    let params = item.to_string();
    let path = params.trim_matches('"');
    let full_version = version_with_shorthash(path).unwrap();
    format!(r#""{}""#, &full_version).parse().unwrap()
}

/// Get the project version and a short hash
fn version_with_shorthash(project_path: &str) -> Result<String> {
    debug!("project_path: {}", project_path);
    debug!(
        "current_path: {}",
        PathBuf::from_str(".")?.canonicalize()?.display()
    );

    let path = PathBuf::from_str(project_path)?.canonicalize()?;
    debug!("path: {}", path.display());

    let full_hash = hash_project(&path)?;
    debug!("long hash: {}", &full_hash);

    let short_hash = &full_hash[..8];
    debug!("short hash: {}", short_hash);

    let version = version_project(&path)?;
    debug!("Version: {}", &version);

    Ok(format!("{}_{}", version, short_hash))
}

/// Full project hash
fn hash_project(path: &Path) -> Result<String> {
    let files = project_files(&path)?;

    let hash = files
        .iter()
        .filter_map(|path| sha256::digest_file(&path).ok())
        .collect::<Vec<String>>()
        .join("");
    Ok(sha256::digest_bytes(hash.as_bytes()))
}

fn version_project(path: &Path) -> Result<String> {
    let file = fs::read_to_string(path.join("Cargo.toml"))?.parse::<Value>()?;

    Ok(file
        .as_table()
        .and_then(|value| value.get("package"))
        .and_then(|value| value.get("version"))
        .and_then(|value| value.as_str().map(|v| v.to_string()))
        .unwrap_or("0.0.0".to_string()))
}

/// List of project files
fn project_files(path: &Path) -> Result<Vec<PathBuf>> {
    let result: Vec<PathBuf> = fs::read_dir(&path)?
        .filter_map(|path| path.ok())
        .map(|path| path.path())
        .filter_map(|path| {
            if path.is_dir() {
                let name = path.file_name()?.to_str()?;
                if ["target", "debug", "release", ".idea"].contains(&name) {
                    None
                } else {
                    project_files(&path).ok()
                }
            } else if path.is_file() {
                let ext = path.extension()?.to_str()?;
                if ["rs", "toml", "lock"].contains(&ext) {
                    Some(vec![path])
                } else {
                    None
                }
            } else {
                None
            }
        })
        .flatten()
        .collect();

    Ok(result)
}

#[cfg(test)]
mod tests {
    use log::debug;
    use crate::{version_with_shorthash};

    #[test]
    fn test_version_with_shorthash() {
        env_logger::init();

        let version = version_with_shorthash("../pontemapi").unwrap();
        debug!("{:?}", version);
    }
}
