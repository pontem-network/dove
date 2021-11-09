use std::fs::read_to_string;
use std::path::PathBuf;
use crate::context::Context;

use anyhow::{Error, Result};
use dialect::{Dialect, init_context};
use move_cli::Move;
use move_package::resolution::resolution_graph::ResolutionGraph;
use move_package::source_package::{layout, manifest_parser};
use move_package::source_package::manifest_parser::parse_dialect;
use crate::cmd::new::New;
use structopt::StructOpt;
use crate::cmd::init::Init;

// use semver::{Version, VersionReq};
// use crate::context::{Context, get_context, load_manifest};
//
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
    fn context(&self, project_dir: PathBuf, move_args: Move) -> Result<Context> {
        init_context(move_args.dialect);
        let manifest_string =
            read_to_string(project_dir.join(layout::SourcePackageLayout::Manifest.path()))?;
        let toml_manifest = manifest_parser::parse_move_manifest_string(manifest_string)?;
        let manifest = manifest_parser::parse_source_manifest(toml_manifest)?;

        Ok(Context {
            project_dir,
            move_args,
            manifest,
        })
    }

    /// Apply command with given context.
    fn apply(&self, ctx: Context) -> Result<()>;
}

//
// fn check_dove_version(req_ver: &str, act_ver: &str) -> Result<(), Error> {
//     let req = VersionReq::parse(req_ver)
//         .map_err(|err| Error::new(err).context("Failed to parse dove_version from Dove.toml"))?;
//     let actual = Version::parse(act_ver).expect("Expected valid dove version");
//     if !req.matches(&actual) {
//         Err(anyhow!("The dove version must meet the conditions '{}'. The current version of dove is '{}'.", req_ver, act_ver))
//     } else {
//         Ok(())
//     }
// }
//
// #[cfg(test)]
// mod tests {
//     use semver::Version;
//     use crate::cmd::check_dove_version;
//
//     #[test]
//     fn test_dove_version() {
//         Version::parse(env!("CARGO_PKG_VERSION")).unwrap();
//     }
//
//     #[test]
//     fn test_check_dove_version() {
//         check_dove_version(">=1.2.3, <1.8.0", "1.5.2").unwrap();
//         check_dove_version(">=1.2.2", "1.2.0").unwrap_err();
//     }
// }
