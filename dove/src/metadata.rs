use std::fs;
use std::fs::{create_dir_all, read_to_string};
use anyhow::Error;
use serde::{Deserialize, Serialize};
use lang::compiler::metadata::{Unit, parse, FuncMeta, ModuleMeta};
use lang::compiler::file::is_move_file;
use crate::context::Context;
use std::path::PathBuf;
use crate::manifest::DoveToml;

/// All modules and scripts from dependencies
pub(crate) fn get_all_dependencies(ctx: &Context) -> Result<Vec<Unit>, Error> {
    let external_folder = ctx.path_for(&ctx.manifest.layout.deps);
    if !external_folder.exists() {
        return Ok(Vec::new());
    }

    let sender = ctx.account_address_str()?;
    let dialect = ctx.dialect.as_ref();

    let externals = external_folder
        .read_dir()?
        .filter_map(|path| path.ok())
        .map(|path| path.path())
        .filter(|path| {
            path.is_dir()
                && path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .unwrap_or_default()
                    .starts_with("git_")
        })
        .map(test_folder)
        .map(|(project_path, test_path)| {
            move_files_without_tests(project_path, test_path.as_ref())
        })
        .flatten()
        .collect::<Vec<_>>();

    let meta = externals
        .iter()
        .filter_map(|path| parse(&path.to_string_lossy(), dialect, &sender).ok())
        .flatten()
        .collect();

    Ok(meta)
}

fn test_folder(path: PathBuf) -> (PathBuf, Option<PathBuf>) {
    let dove_path = path.join("Dove.toml");
    if !dove_path.exists() {
        return (path, None);
    }

    let test = read_to_string(dove_path)
        .ok()
        .and_then(|str| toml::from_str::<DoveToml>(&str).ok())
        .map(|dovetoml| path.join(dovetoml.layout.tests_dir));

    (path, test)
}
fn move_files_without_tests(
    project_folder: PathBuf,
    test_folder: Option<&PathBuf>,
) -> Vec<PathBuf> {
    project_folder
        .read_dir()
        .ok()
        .map(|read| {
            read.into_iter()
                .filter_map(|path| path.ok())
                .map(|path| path.path())
                .filter(|path| {
                    !test_folder
                        .map(|test| path.starts_with(test))
                        .unwrap_or(false)
                })
                .filter_map(|path| {
                    if path.is_dir() {
                        Some(move_files_without_tests(path, test_folder))
                    } else if path.is_file() && is_move_file(&path) {
                        Some(vec![path])
                    } else {
                        None
                    }
                })
                .flatten()
                .collect::<Vec<PathBuf>>()
        })
        .unwrap_or_default()
}

/// All scripts and modules from dependencies
#[derive(Debug, Serialize, Deserialize)]
pub struct MapDependencies {
    /// All scripts from dependencies
    pub scripts: Vec<FuncMeta>,
    /// All modules from dependencies
    pub modules: Vec<ModuleMeta>,
}
impl MapDependencies {
    /// Create a dependencies map and save it to disk
    pub fn create_and_save(ctx: &Context) -> Result<(), Error> {
        let fpath = ctx.path_for(&ctx.manifest.layout.project_map);
        if !fpath.exists() {
            let parent = fpath.parent().map_or_else(
                || anyhow::bail!("The path to the dependencies map is set incorrectly"),
                Ok,
            )?;
            if !parent.exists() {
                create_dir_all(parent)?;
            }
        }

        let map = Self::create(ctx)?;
        let bmap = bcs::to_bytes(&map)?;
        fs::write(&fpath, bmap).map_err(|err| anyhow!("{}", err.to_string()))
    }

    /// Get all scripts and modules from dependencies
    pub fn create(ctx: &Context) -> Result<MapDependencies, Error> {
        let deps = get_all_dependencies(ctx)?;

        let (scripts, modules) = deps.into_iter().fold(
            (Vec::new(), Vec::new()),
            |(mut scripts, mut modules), unit| {
                match unit {
                    Unit::Module(module) => modules.push(module),
                    Unit::Script(script) => scripts.push(script),
                }
                (scripts, modules)
            },
        );

        Ok(MapDependencies { scripts, modules })
    }

    /// Download a dependencies map from disk
    pub fn load(ctx: &Context) -> Result<MapDependencies, Error> {
        let fpath = ctx.path_for(&ctx.manifest.layout.project_map);
        if !fpath.exists() {
            anyhow::bail!("The project map file was not found. Build a project.");
        }

        let bmap = fs::read(&fpath)?;
        bcs::from_bytes::<MapDependencies>(&bmap).map_err(|err| anyhow!("{:?}", err))
    }
}
