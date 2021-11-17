use std::fs::read_to_string;
use std::path::PathBuf;
use anyhow::Result;

use dialect::init_context;
use move_cli::Move;
use move_package::source_package::{layout, manifest_parser};
use move_package::source_package::parsed_manifest::SourceManifest;
use move_symbol_pool::symbol::Symbol;

use crate::context::Context;
use std::collections::BTreeMap;

/// Project builder.
pub mod build;
/// Project dependencies loader.
pub mod clean;
/// Documentation generator.
pub mod docgen;
/// Export Dove.toml => Move.toml
pub mod export;
/// Dependencies fetcher.
pub mod fetch;
/// Project initializer.
pub mod init;
/// Project creator.
pub mod new;
/// Run move prover.
pub mod prover;
/// Script executor.
pub mod run;
/// Test runner.
pub mod test;
/// Create transaction.
pub mod tx;

/// Move command.
pub trait Cmd {
    /// Returns project context.
    /// This function must be overridden if the command is used with a custom context.
    fn context(&mut self, project_dir: PathBuf, move_args: Move) -> Result<Context> {
        init_context(move_args.dialect);
        let manifest_string =
            read_to_string(project_dir.join(layout::SourcePackageLayout::Manifest.path()))?;
        let toml_manifest = manifest_parser::parse_move_manifest_string(manifest_string)?;
        let manifest = manifest_parser::parse_source_manifest(toml_manifest)?;

        // let mut named_address = manifest.addresses.unwrap_or_default();
        // for (name, addr) in &move_args.named_addresses {
        //     named_address.insert(Symbol::from(name), Some(AccountAddress::new(addr.into_bytes())));
        // }

        Ok(Context {
            project_dir,
            move_args,
            manifest,
        })
    }

    /// Apply command with given context.
    fn apply(&mut self, ctx: &mut Context) -> Result<()>;
}

/// Context with empty manifest
pub fn context_with_empty_manifest(project_dir: PathBuf, move_args: Move) -> Result<Context> {
    init_context(move_args.dialect);
    Ok(Context {
        project_dir,
        move_args,
        // empty manifest
        manifest: default_sourcemanifest(),
    })
}

/// empty manifest
fn default_sourcemanifest() -> SourceManifest {
    use move_package::source_package::parsed_manifest::PackageInfo;

    SourceManifest {
        package: PackageInfo {
            name: Symbol::from("Default"),
            version: (0, 0, 0),
            license: None,
            authors: Vec::new(),
        },
        addresses: None,
        dependencies: BTreeMap::new(),
        dev_address_assignments: None,
        dev_dependencies: BTreeMap::new(),
        build: None,
    }
}
