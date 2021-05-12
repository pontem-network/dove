use std::path::{PathBuf, Path};
use crate::manifest::{DoveToml, MANIFEST, read_manifest, default_dialect};
use std::str::FromStr;
use anyhow::{Result, anyhow, Error};
use std::env;
use lang::compiler::dialects::{Dialect, DialectName};
use crate::index::Index;
use move_core_types::account_address::AccountAddress;

/// Project context.
pub struct Context {
    /// Project root directory.
    pub project_dir: PathBuf,
    /// Project manifest.
    pub manifest: DoveToml,
    /// Move dialect.
    pub dialect: Box<dyn Dialect>,
}

impl Context {
    /// Create absolute path in project.
    pub fn path_for<P: AsRef<Path>>(&self, path: P) -> PathBuf {
        self.project_dir.join(path)
    }

    /// Create absolute paths in project.
    pub fn paths_for<P: AsRef<Path>>(&self, paths: &[P]) -> Vec<PathBuf> {
        paths
            .iter()
            .map(|d| self.path_for(&d))
            .filter(|p| p.exists())
            .collect()
    }

    /// Build project index.
    pub fn build_index(&self) -> Result<Index, Error> {
        let mut index = Index::load(self)?;
        index.build()?;
        Ok(index)
    }

    /// Returns project name or default name `project` if the name is not defined.
    pub fn project_name(&self) -> String {
        self.manifest.package.name.clone().unwrap_or_else(|| {
            self.project_dir
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("project")
                .to_owned()
        })
    }

    /// Returns provided account address.
    pub fn account_address(&self) -> Result<AccountAddress> {
        self.dialect
            .parse_address(&self.manifest.package.account_address)
    }
}

/// Create a new context for the current directory.
pub fn create_context() -> Result<Context> {
    let project_dir = env::current_dir()?;
    let manifest = DoveToml::default();

    let dialect_name = manifest
        .package
        .dialect
        .clone()
        .unwrap_or_else(default_dialect);
    let dialect = DialectName::from_str(&dialect_name)?;

    Ok(Context {
        project_dir,
        manifest,
        dialect: dialect.get_dialect(),
    })
}

/// Returns project context.
pub fn get_context(project_dir: PathBuf) -> Result<Context> {
    let manifest = load_manifest(&project_dir)?;

    let dialect_name = manifest
        .package
        .dialect
        .clone()
        .unwrap_or_else(default_dialect);
    let dialect = DialectName::from_str(&dialect_name)?;

    Ok(Context {
        project_dir,
        manifest,
        dialect: dialect.get_dialect(),
    })
}

fn load_manifest(project_dir: &Path) -> Result<DoveToml> {
    let manifest = project_dir.join(MANIFEST);
    if !manifest.exists() {
        Err(anyhow!(
            "could not find `{}` in `{:?}`.",
            MANIFEST,
            project_dir
        ))
    } else {
        read_manifest(&manifest)
    }
}
