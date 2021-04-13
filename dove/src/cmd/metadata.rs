use anyhow::{Error, Result};
use serde::Serialize;
use structopt::StructOpt;

use crate::cmd::Cmd;
use crate::context::Context;
use crate::manifest::{Dependence, DoveToml, Git, Layout};

fn into_dove_json(ctx: Context) -> DoveJson {
    let project_name = ctx.project_name();
    let Context {
        project_dir,
        manifest,
        dialect: _,
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
    let package_json = PackageJson {
        name: project_name,
        account_address: package.account_address,
        authors: package.authors,
        blockchain_api: package.blockchain_api,
        git_dependencies: git_deps,
        local_dependencies: local_deps,
    };
    DoveJson {
        package: package_json,
        layout,
    }
}

/// Metadata project command.
#[derive(StructOpt, Debug)]
pub struct Metadata {
    #[structopt(short = "j", long = "json")]
    json: bool,
}

impl Cmd for Metadata {
    fn apply(self, ctx: Context) -> Result<(), Error> {
        if self.json {
            let manifest_as_json = into_dove_json(ctx);
            println!(
                "{}",
                serde_json::to_string_pretty::<DoveJson>(&manifest_as_json)?
            );
        } else {
            println!("{}", toml::to_string_pretty(&ctx.manifest)?);
        }

        Ok(())
    }
}

/// Movec manifest.
#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
pub struct DoveJson {
    /// Project info.
    pub package: PackageJson,
    /// Project layout.
    #[serde(default)]
    pub layout: Layout,
}

/// Project info.
#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
pub struct PackageJson {
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

        let dove_json = into_dove_json(context);
        //
        assert_eq!(dove_json.package.local_dependencies.len(), 1);
        assert_eq!(
            dove_json.package.local_dependencies[0],
            fs::canonicalize(move_project_dir.join("stdlib"))
                .unwrap()
                .into_os_string()
                .into_string()
                .unwrap()
        );
    }
}
