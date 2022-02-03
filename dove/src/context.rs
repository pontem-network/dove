use std::fs;
use std::fs::read_to_string;
use std::path::PathBuf;
use anyhow::Error;
use move_cli::Move;
use move_package::compilation::package_layout::CompiledPackageLayout;
use move_package::source_package::{layout, manifest_parser};
use move_package::source_package::parsed_manifest::{AddressDeclarations, SourceManifest};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use anyhow::Result;
use move_core_types::errmap::ErrorMapping;
use move_core_types::gas_schedule::CostTable;

use move_vm_runtime::native_functions::NativeFunctionTable;

pub struct Context {
    pub project_root_dir: PathBuf,
    pub move_args: Move,
    pub manifest: SourceManifest,
    pub manifest_hash: u64,
    pub error_descriptions: ErrorMapping,
    pub native_functions: NativeFunctionTable,
    pub cost_table: CostTable,
}

impl Context {
    pub fn new(
        project_root_dir: PathBuf,
        move_args: Move,
        error_descriptions: ErrorMapping,
        native_functions: NativeFunctionTable,
        cost_table: CostTable,
    ) -> Result<Self> {
        let manifest_string =
            read_to_string(project_root_dir.join(layout::SourcePackageLayout::Manifest.path()))?;
        let mut hasher = DefaultHasher::default();
        manifest_string.hash(&mut hasher);
        let manifest_hash = hasher.finish();
        let toml_manifest = manifest_parser::parse_move_manifest_string(manifest_string)?;
        let manifest = manifest_parser::parse_source_manifest(toml_manifest)?;

        Ok(Context {
            project_root_dir,
            move_args,
            manifest,
            manifest_hash,
            error_descriptions,
            native_functions,
            cost_table,
        })
    }

    /// Path for bundle
    ///     ./build/<package name>/bundles
    pub fn bundles_output_path(&self, package_name: &str) -> Result<PathBuf, Error> {
        let dir = self
            .project_root_dir
            .join("build")
            .join(self.manifest.package.name.as_str())
            .join("bundles");
        if !dir.exists() {
            fs::create_dir_all(&dir)?;
        }
        Ok(dir.join(package_name))
    }

    /// Creates path to the move cli build folder.
    pub fn path_for_build(&self, pac_name: Option<&str>, path: CompiledPackageLayout) -> PathBuf {
        let build = self
            .project_root_dir
            .join(CompiledPackageLayout::Root.path());
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
    pub fn address_declarations(&self) -> AddressDeclarations {
        self.manifest.addresses.clone().unwrap_or_default()
    }

    /// Returns transaction output folder for specified `package` or for the default package.
    pub fn tx_output_path(&self, pac: Option<String>) -> PathBuf {
        let mut build = self
            .project_root_dir
            .join(CompiledPackageLayout::Root.path());
        if let Some(pac) = pac {
            build = build.join(pac);
        } else {
            build = build.join(self.manifest.package.name.as_str());
        }
        build.join("transaction")
    }
}
