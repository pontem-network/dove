use serde_derive::{Serialize, Deserialize};
use anyhow::Error;
use std::path::Path;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use toml::Value;

/// Movec manifest name.
pub const MANIFEST: &str = "Move.toml";

/// Movec manifest.
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct MoveToml {
    /// Project info.
    pub package: Package,
    /// Project layout.
    pub layout: Option<Layout>,
}

/// Project info.
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct Package {
    /// Project name.
    pub name: Option<String>,
    /// Project AccountAddress.
    pub account_address: Option<String>,
    /// Authors list.
    pub authors: Option<Vec<String>>,
    /// dnode base url.
    pub blockchain_api: Option<String>,
}

/// Project layout.
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct Layout {
    /// Directory with module sources.
    pub module_dir: Option<String>,
    /// Directory with script sources.
    pub script_dir: Option<String>,
    /// Movec cache.
    pub bytecode_cache: Option<String>,
    /// Directory with compiled modules.
    pub module_output: Option<String>,
    /// Directory with compiled scripts.
    pub script_output: Option<String>,
    /// Processing directory.
    pub temp_dir: Option<String>,
}

impl Layout {
    /// Create a new layout.
    pub fn new() -> Layout {
        Layout {
            module_dir: None,
            script_dir: None,
            bytecode_cache: None,
            module_output: None,
            script_output: None,
            temp_dir: None,
        }
    }

    /// Fill layout with default values.
    pub fn fill(&mut self) {
        self.module_dir
            .get_or_insert_with(|| "src/modules".to_owned());
        self.script_dir
            .get_or_insert_with(|| "src/scripts".to_owned());
        self.bytecode_cache
            .get_or_insert_with(|| "target/deps".to_owned());
        self.module_output
            .get_or_insert_with(|| "target/artifacts/modules".to_owned());
        self.script_output
            .get_or_insert_with(|| "target/artifacts/scripts".to_owned());
        self.temp_dir
            .get_or_insert_with(|| "target/build".to_owned());
    }
}

/// Reads the manifest by path.
pub fn read_manifest(path: &Path) -> Result<MoveToml, Error> {
    Ok(toml::from_str(&fs::read_to_string(path)?)?)
}

/// Stores the manifest by path.
pub fn store_manifest(path: &Path, manifest: MoveToml) -> Result<(), Error> {
    let value = toml::to_vec(&Value::try_from(manifest)?)?;
    let mut f = OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .open(path)?;
    f.set_len(0)?;
    f.write_all(&value)?;
    Ok(())
}
