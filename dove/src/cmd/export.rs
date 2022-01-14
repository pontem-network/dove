use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::collections::BTreeMap;
use structopt::StructOpt;
use anyhow::{Error, Result};
use serde::Serialize;
use dialect::Dialect;
use move_cli::Move;
use crate::cmd::{Cmd, context_with_empty_manifest};
use crate::context::Context;
use crate::export::{
    create_project_directories, move_modules, dependency_create_from, DependenceExport,
};
use crate::export::movetoml::{AddressDeclarations, Dependencies, MoveToml, PackageInfo};
use crate::export::dove_manifest::read_manifest;

/// Export Dove.toml => Move.toml
#[derive(StructOpt, Debug)]
#[structopt(setting(structopt::clap::AppSettings::ColoredHelp))]
pub struct Export {}

impl Cmd for Export {
    /// Redefined. Empty Manifest
    fn context(&mut self, project_dir: PathBuf, move_args: Move) -> anyhow::Result<Context> {
        context_with_empty_manifest(project_dir, move_args)
    }

    fn apply(&mut self, ctx: &mut Context) -> anyhow::Result<()>
    where
        Self: Sized,
    {
        self.export(ctx)
    }
}

impl Export {
    fn export(&self, ctx: &Context) -> Result<(), Error> {
        let project_dir = ctx.project_dir.clone();

        let dove_toml_path = project_dir.join("Dove.toml");
        if !dove_toml_path.exists() {
            anyhow::bail!("file Dove.toml was not found");
        }
        let dove_toml = read_manifest(&dove_toml_path)?;
        let dialect = Dialect::from_str(&dove_toml.package.dialect.unwrap_or_default())?;

        // Project directories
        create_project_directories(&project_dir)?;

        // delete artifacts folder
        let artifacts_path = project_dir.join("artifacts");
        if artifacts_path.exists() {
            std::fs::remove_dir_all(artifacts_path)?;
        }

        // Move modules to the "source" folder
        move_modules(&project_dir)?;

        // <PROJECT_DIR>/doc.toml
        save_as_toml(&ctx.doc_path(), &dove_toml.doc)?;
        // <PROJECT_DIR>/boogie_options.toml
        if let Some(boogie) = &dove_toml.boogie_options {
            save_as_toml(&ctx.boogie_options_path(), &boogie)?;
        }

        // account_address
        let mut addresses = AddressDeclarations::new();
        addresses.insert(
            "Account".to_string(),
            Some(dialect.parse_address(&dove_toml.package.account_address)?),
        );

        // Dependencies
        let result_dependency_conversion: Vec<DependenceExport> =
            if let Some(deps) = dove_toml.package.dependencies {
                deps.deps
                    .iter()
                    .filter_map(dependency_create_from)
                    .collect()
            } else {
                Vec::new()
            };

        let dependencies_errors: BTreeMap<String, String> = result_dependency_conversion
            .iter()
            .filter_map(|dep| {
                dep.error
                    .as_ref()
                    .map(|err| (dep.name.clone(), err.to_string()))
            })
            .collect();
        let dependencies: Option<Dependencies> = if result_dependency_conversion.is_empty() {
            None
        } else {
            Some(
                result_dependency_conversion
                    .into_iter()
                    .map(|dep| (dep.name, dep.dep))
                    .collect(),
            )
        };

        let move_toml = MoveToml {
            package: PackageInfo {
                name: dove_toml.package.name.unwrap_or_else(|| {
                    project_dir
                        .file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_string()
                }),
                authors: Vec::new(),
                license: None,
                version: (0, 0, 1),
                dialect: Some(dialect),
                dove_version: dove_toml.package.dove_version,
            },
            addresses: Some(addresses),
            dependencies,
        };

        let mut move_toml_string = toml::to_string(&move_toml)?;
        // add/output errors
        for (name, error) in dependencies_errors.iter() {
            let error = error.replace('\n', "\n# \t").to_string();
            println!("\nWarning:\n# {}", &error);
            if let Some(pos) = move_toml_string
                .find(name)
                .and_then(|pos| { &move_toml_string[..pos] }.rfind('\n'))
            {
                move_toml_string.insert_str(pos, &format!("# ERROR: {}", error));
            }
        }
        // Saving move toml
        std::fs::write(&project_dir.join("Move.toml"), move_toml_string)
            .map_err(|err| anyhow!(err.to_string()))
    }
}

fn save_as_toml<S>(path: &Path, value: &S) -> Result<(), Error>
where
    S: Serialize,
{
    std::fs::write(path, toml::to_string(value)?).map_err(|err| anyhow!(err.to_string()))
}
