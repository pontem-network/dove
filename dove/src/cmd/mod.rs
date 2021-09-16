use std::path::PathBuf;

use anyhow::{Error, Result};
use semver::{Version, VersionReq};
use crate::context::{Context, get_context, load_manifest};

/// Project builder.
pub mod build;
/// Check project.
pub mod check;
/// Project dependencies loader.
pub mod clean;
/// Documentation generator.
pub mod docgen;
/// Dependencies fetcher.
pub mod fetch;
/// Project initializer.
pub mod init;
/// Project metadata.
pub mod metadata;
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
    fn context(&self, project_dir: PathBuf) -> Result<Context> {
        let manifest = load_manifest(&project_dir)?;

        if let Some(required_version) = &manifest.package.dove_version {
            check_dove_version(required_version, env!("CARGO_PKG_VERSION"))?;
        }

        get_context(project_dir, manifest)
    }

    /// Apply command with given context.
    fn apply(self, ctx: Context) -> Result<()>
    where
        Self: std::marker::Sized;

    /// Functions create execution context and apply command with it.
    fn execute(self, project_dir: PathBuf) -> Result<()>
    where
        Self: std::marker::Sized,
    {
        let context = self.context(project_dir)?;
        self.apply(context)
    }
}

fn check_dove_version(req_ver: &str, act_ver: &str) -> Result<(), Error> {
    let req = VersionReq::parse(req_ver)
        .map_err(|err| Error::new(err).context("Failed to parse dove_version from Dove.toml"))?;
    let actual = Version::parse(act_ver).expect("Expected valid dove version");
    if !req.matches(&actual) {
        Err(anyhow!("The dove version must meet the conditions '{}'. The current version of dove is '{}'.", req_ver, act_ver))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use semver::Version;
    use crate::cmd::check_dove_version;

    #[test]
    fn test_dove_version() {
        Version::parse(env!("CARGO_PKG_VERSION")).unwrap();
    }

    #[test]
    fn test_check_dove_version() {
        check_dove_version(">=1.2.3, <1.8.0", "1.5.2").unwrap();
        check_dove_version(">=1.2.2", "1.2.0").unwrap_err();
    }
}
