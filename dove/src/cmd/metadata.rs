use anyhow::{Error, Result};
use serde::Serialize;
use structopt::StructOpt;

use crate::cmd::Cmd;
use crate::context::Context;
use crate::manifest::{Dependence, DoveToml, Git, Layout};

fn into_metadata(ctx: Context) -> DoveMetadata {
    let project_name = ctx.project_name();
    let Context {
        project_dir,
        manifest,
        dialect,
    } = ctx;
    let DoveToml { package, layout } = manifest;

    let dependencies = package.dependencies.unwrap_or_default();
    let mut local_deps = vec![];
    let mut git_deps = vec![];
    for dep in dependencies.deps {
        match dep {
            Dependence::Git(git) => git_deps.push(git),
            Dependence::Path(deppath) => {
                if let Ok(abs_path) = project_dir.join(deppath.path).canonicalize() {
                    local_deps.push(abs_path.into_os_string().into_string().unwrap());
                }
            }
        }
    }
    let package_metadata = PackageMetadata {
        name: project_name,
        account_address: package.account_address,
        authors: package.authors,
        blockchain_api: package.blockchain_api,
        git_dependencies: git_deps,
        local_dependencies: local_deps,
        dialect: dialect.name().to_string(),
    };
    DoveMetadata {
        package: package_metadata,
        layout,
    }
}

/// Metadata project command.
#[derive(StructOpt, Debug)]
pub struct Metadata {}

impl Cmd for Metadata {
    fn apply(self, ctx: Context) -> Result<(), Error> {
        let metadata = into_metadata(ctx);
        println!(
            "{}",
            serde_json::to_string_pretty::<DoveMetadata>(&metadata)?
        );
        Ok(())
    }
}

/// Movec manifest.
#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
pub struct DoveMetadata {
    /// Project info.
    pub package: PackageMetadata,
    /// Project layout.
    #[serde(default)]
    pub layout: Layout,
}

/// Project info.
#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
pub struct PackageMetadata {
    /// Project name.
    pub name: String,
    /// Project AccountAddress.
    #[serde(default = "code_code_address")]
    pub account_address: Option<String>,
    /// Authors list.
    #[serde(default)]
    pub authors: Vec<String>,
    /// dnode base url.
    pub blockchain_api: Option<String>,
    /// Git dependency list.
    pub git_dependencies: Vec<Git>,
    /// Local dependency list.
    pub local_dependencies: Vec<String>,
    /// Dialect used in the project.
    pub dialect: String,
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;

    use crate::context::get_context;

    use super::*;

    #[test]
    fn paths_in_metadata_are_absolute() {
        let move_project_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("resources")
            .join("test_move_project");
        let context = get_context(move_project_dir.clone()).unwrap();

        let metadata = into_metadata(context);

        assert_eq!(metadata.package.dialect, "dfinance".to_string());

        // non-existent paths ain't present in the metadata
        assert_eq!(metadata.package.local_dependencies.len(), 1);
        assert_eq!(
            metadata.package.local_dependencies[0],
            fs::canonicalize(move_project_dir.join("stdlib"))
                .unwrap()
                .into_os_string()
                .into_string()
                .unwrap()
        );
    }
}
