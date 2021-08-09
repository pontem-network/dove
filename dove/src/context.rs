use std::path::{Path, PathBuf};
use std::str::FromStr;

use anyhow::{anyhow, Error, Result};
use move_core_types::account_address::AccountAddress;

use lang::compiler::dialects::{Dialect, DialectName};

use crate::index::Index;
use crate::manifest::{default_dialect, DoveToml, MANIFEST, read_manifest};
use diem_crypto::hash::CryptoHash;
use crate::index::interface::{InterfaceBuilder, Interface};

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
    /// Returns create absolute path in project as string.
    pub fn str_path_for<P: AsRef<Path>>(&self, path: P) -> Result<String, Error> {
        let mut abs_path = self.path_for(path);

        if abs_path.exists() {
            abs_path = dunce::canonicalize(abs_path)?;
        }

        abs_path
            .to_str()
            .map(|path| path.to_owned())
            .ok_or_else(|| anyhow!("Failed to display absolute path:[{:?}]", abs_path))
    }

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
    pub fn build_index(&self) -> Result<(Index, Interface), Error> {
        let index_path = self.path_for(&self.manifest.layout.index);
        let old_index = Index::load(&index_path)?.unwrap_or_default();

        let package_hash = self.package_hash();
        let index = if old_index.package_hash == package_hash {
            old_index
        } else {
            let new_index = Index::build(package_hash, self)?;
            new_index.store(&index_path)?;
            new_index.remove_unused(old_index.diff(&new_index))?;
            new_index.remove_unnecessary_elements_in_dependencies();
            new_index
        };
        let builder = InterfaceBuilder::new(self, &index);
        let interface = builder.build()?;
        Ok((index, interface))
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

    /// Returns provided account address.
    pub fn account_address_str(&self) -> Result<String> {
        Ok(format!(
            "0x{}",
            self.dialect
                .parse_address(&self.manifest.package.account_address)?
        ))
    }
    /// Calculates package hash.
    pub fn package_hash(&self) -> String {
        self.manifest.package.hash().to_string()
    }

    /// Returns interface files dir.
    pub fn interface_files_dir(&self) -> PathBuf {
        self.path_for(&self.manifest.layout.artifacts)
            .join("interface_files_dir")
    }

    /// Returns directory for dependency bytecode.
    pub fn deps_mv_dir(&self) -> PathBuf {
        self.path_for(&self.manifest.layout.artifacts).join("depmv")
    }

    /// Interface files lock.
    pub fn interface_files_lock(&self) -> PathBuf {
        self.path_for(&self.manifest.layout.artifacts)
            .join("interface.lock")
    }
}

pub(crate) fn get_context(project_dir: PathBuf, manifest: DoveToml) -> Result<Context> {
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

pub(crate) fn load_manifest(project_dir: &Path) -> Result<DoveToml> {
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

pub(crate) fn str_path<P: AsRef<Path>>(path: P) -> Result<String, Error> {
    let path = path.as_ref();

    path.to_str()
        .map(|path| path.to_owned())
        .ok_or_else(|| anyhow!("Failed to display absolute path:[{:?}]", path))
}
