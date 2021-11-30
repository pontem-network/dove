use std::fs;
use std::path::PathBuf;
use anyhow::Error;
use move_cli::Move;
use move_core_types::account_address::AccountAddress;
use move_package::compilation::package_layout::CompiledPackageLayout;
use move_package::source_package::parsed_manifest::{AddressDeclarations, SourceManifest};
use move_symbol_pool::Symbol;

/// Project context.
pub struct Context {
    /// Project root directory.
    pub project_dir: PathBuf,
    /// Move args.
    pub move_args: Move,
    /// Project manifest.
    pub manifest: SourceManifest,
    /// Manifest hash.
    pub manifest_hash: u64,
}

impl Context {
    /// Path for bundle
    ///     ./build/<package name>/bundles
    pub fn bundles_output_path(&self, package_name: &str) -> Result<PathBuf, Error> {
        let dir = self
            .project_dir
            .join("build")
            .join(self.manifest.package.name.as_str())
            .join("bundles");
        if !dir.exists() {
            fs::create_dir_all(&dir)?;
        }
        Ok(dir.join(package_name))
    }

    /// Path to boogie_options.toml
    ///     <PROJECT_DIR>/boogie_options.toml
    pub fn boogie_options_path(&self) -> PathBuf {
        self.project_dir.join("boogie_options.toml")
    }

    /// Path to doc.toml
    ///     <PROJECT_DIR>/doc.toml
    pub fn doc_path(&self) -> PathBuf {
        self.project_dir.join("doc.toml")
    }

    /// Creates path to the move cli build folder.
    pub fn path_for_build(&self, pac_name: Option<&str>, path: CompiledPackageLayout) -> PathBuf {
        let build = self.project_dir.join(CompiledPackageLayout::Root.path());
        if CompiledPackageLayout::Root != path {
            if let Some(pac_name) = pac_name {
                build.join(pac_name).join(path.path())
            } else {
                build
            }
        } else {
            build
        }
    }

    /// Creates and returns map of named addresses.
    pub fn named_address(&self) -> AddressDeclarations {
        let mut named_address = self.manifest.addresses.clone().unwrap_or_default();
        for (name, addr) in &self.move_args.named_addresses {
            named_address.insert(
                Symbol::from(name.as_str()),
                Some(AccountAddress::new(addr.into_bytes())),
            );
        }
        named_address
    }

    /// Returns transaction output folder for specified `package` or for the default package.
    pub fn tx_output_path(&self, pac: Option<String>) -> PathBuf {
        let mut build = self.project_dir.join(CompiledPackageLayout::Root.path());
        if let Some(pac) = pac {
            build = build.join(pac);
        } else {
            build = build.join(self.manifest.package.name.as_str());
        }
        build.join("transaction")
    }
}
