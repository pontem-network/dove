use anyhow::{Error, Result};
use serde::Serialize;
use structopt::StructOpt;

use crate::index::resolver::git;
use crate::cmd::Cmd;
use crate::context::{Context, str_path};
use crate::manifest::{Dependence, Git, Layout};
use std::path::Path;

fn into_metadata(mut ctx: Context) -> Result<DoveMetadata, Error> {
    let layout = ctx.manifest.layout.to_absolute(&ctx)?;

    let mut local_deps = vec![];
    let mut git_deps = vec![];
    if let Some(dependencies) = ctx.manifest.package.dependencies.take() {
        for dep in dependencies.deps {
            match dep {
                Dependence::Git(git) => {
                    git_deps.push(GitMetadata::new(git, &ctx)?);
                }
                Dependence::Path(dep_path) => {
                    local_deps.push(ctx.str_path_for(&dep_path.path)?);
                }
            }
        }
    }

    let package_metadata = PackageMetadata {
        name: ctx.project_name(),
        account_address: ctx.manifest.package.account_address,
        authors: ctx.manifest.package.authors,
        blockchain_api: ctx.manifest.package.blockchain_api,
        git_dependencies: git_deps,
        local_dependencies: local_deps,
        dialect: ctx.dialect.name().to_string(),
    };
    Ok(DoveMetadata {
        package: package_metadata,
        layout,
    })
}

/// Metadata project command.
#[derive(StructOpt, Debug)]
pub struct Metadata {}

impl Cmd for Metadata {
    fn apply(self, ctx: Context) -> Result<(), Error> {
        let metadata = into_metadata(ctx)?;
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
    pub account_address: String,
    /// Authors list.
    #[serde(default)]
    pub authors: Vec<String>,
    /// dnode base url.
    pub blockchain_api: Option<String>,
    /// Git dependency list.
    pub git_dependencies: Vec<GitMetadata>,
    /// Local dependency list.
    pub local_dependencies: Vec<String>,
    /// Dialect used in the project.
    pub dialect: String,
}

/// Git dependency metadata.
#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
pub struct GitMetadata {
    /// Git url.
    pub git: String,
    /// Branch name.
    pub branch: Option<String>,
    /// Commit hash.
    pub rev: Option<String>,
    /// Tag.
    pub tag: Option<String>,
    /// Path.
    pub path: Option<String>,
    /// Local path.
    pub local_path: Option<String>,
}

impl GitMetadata {
    /// Create a new git metadata.
    pub fn new(git: Git, ctx: &Context) -> Result<GitMetadata, Error> {
        let path: &Path = ctx.manifest.layout.target_deps.as_ref();
        let path = ctx.path_for(path.join(&git::make_local_name(&git)));
        let local_path = if path.exists() {
            Some(str_path(path)?)
        } else {
            None
        };

        Ok(GitMetadata {
            git: git.git,
            branch: git.branch,
            rev: git.rev,
            tag: git.tag,
            path: git.path,
            local_path,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};

    use crate::context::{get_context, load_manifest};

    use super::*;

    #[test]
    fn paths_in_metadata_are_absolute() {
        fn check_absolute<P: AsRef<Path>>(path: &P) {
            let path = path.as_ref();
            assert!(
                path.starts_with(env!("CARGO_MANIFEST_DIR")),
                "Path {:?} is not absolute",
                path
            );
        }

        let move_project_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("resources")
            .join("test_move_project");
        let manifest = load_manifest(&move_project_dir).unwrap();
        let context = get_context(move_project_dir, manifest).unwrap();

        let metadata = into_metadata(context).unwrap();

        assert_eq!(metadata.package.dialect, "pont".to_string());

        // non-existent paths ain't present in the metadata
        assert_eq!(metadata.package.local_dependencies.len(), 2);
        check_absolute(&metadata.package.local_dependencies[0]);
        check_absolute(&metadata.package.local_dependencies[1]);

        check_absolute(&metadata.layout.module_dir);
        check_absolute(&metadata.layout.script_dir);
        check_absolute(&metadata.layout.tests_dir);
        check_absolute(&metadata.layout.module_output);
        check_absolute(&metadata.layout.packages_output);
        check_absolute(&metadata.layout.script_output);
        check_absolute(&metadata.layout.transaction_output);
        check_absolute(&metadata.layout.target_deps);
        check_absolute(&metadata.layout.target);
        check_absolute(&metadata.layout.index);
    }
}
