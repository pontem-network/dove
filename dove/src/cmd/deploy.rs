use core::mem;
use std::collections::HashMap;
use std::fs;
use std::str::FromStr;
use std::fs::remove_file;
use std::path::{PathBuf, Path};

use anyhow::Error;
use clap::Parser;
use serde::{Serialize, Deserialize};
use anyhow::Result;
use itertools::Itertools;

use move_binary_format::access::ModuleAccess;
use move_binary_format::CompiledModule;
use move_cli::Command as MoveCommand;
use move_cli::package::cli::PackageCommand;
use move_cli::run_cli;
use move_core_types::language_storage::ModuleId;

use crate::context::Context;
use crate::publish::{NodeAccessParams, Publish};

#[derive(Parser, Debug)]
#[clap(about = "dove deploy [FILE_NAME|PATH] [OPTIONS]
    Examples:
    $ dove deploy
    $ dove deploy PACKAGE_NAME --account WALLET_KEY --gas 300
    $ dove deploy PACKAGE_NAME --secret --url ws://127.0.0.1:9944 --gas 400 --modules_exclude MODULE_NAME_1 MODULE_NAME_2 ..
    $ dove deploy MODULE_NAME --secret --url https://127.0.0.1:9933 --gas 400
    $ dove deploy PATH/TO/FILE --account //Alice --gas 300
")]
pub struct Deploy {
    #[clap(help = "Module/Bundle name or path")]
    file: Option<String>,

    // * Only for bundle
    // Names of modules to exclude from the package process.
    // Modules are taken from the <PROJECT_PATH>/build/<PROJECT_NAME>/bytecode_modules directory.
    // The names are case-insensitive and can be specified with an extension.mv or without it.
    // --modules_exclude NAME_1 NAME_2 NAME_3
    #[clap(
        help = "Names of modules to exclude from the package process.",
        long = "modules_exclude",
        multiple_values = true
    )]
    modules_exclude: Vec<String>,

    #[clap(flatten)]
    request: NodeAccessParams,
}

impl Deploy {
    pub fn apply(&mut self, ctx: &mut Context) -> Result<()> {
        // Run `dove package build` first to build all necessary artifacts.
        run_dove_package_build(ctx)?;

        // packaging of modules
        self.bundle_modules_into_pac(ctx)?;

        if !self.request.need_to_publish() {
            return Ok(());
        }

        // Publish a bundle or module to a node
        self.publish(ctx)
    }

    fn bundle_modules_into_pac(&self, ctx: &Context) -> Result<()> {
        // Path to the output file
        let output_file_path = ctx
            .bundles_output_path(ctx.manifest.package.name.as_str())?
            .with_extension("pac");
        if output_file_path.exists() {
            remove_file(&output_file_path)?;
        }

        // Search for modules
        let bytecode_modules_path =
            get_bytecode_modules_path(&ctx.project_root_dir, &ctx.manifest.package.name)
                .unwrap_or_default();

        let mut pac = ModulePackage::default();

        for module in bytecode_modules_path {
            let module_name = module.file_name().map(|name| {
                let name = name.to_string_lossy();
                name[0..name.len() - ".mv".len()].to_string()
            }).ok_or_else(|| anyhow!("Failed to package move module: '{:?}'. File with .mv extension was expected.", module))?;
            if self.modules_exclude.contains(&module_name) {
                continue;
            }
            pac.put(fs::read(&module)?);
        }

        pac.sort()?;

        fs::write(&output_file_path, pac.encode()?)?;

        println!(
            "Modules are packed {}",
            output_file_path
                .canonicalize()
                .unwrap_or_default()
                .display()
        );
        Ok(())
    }

    /// Publish a bundle or module to a node
    fn publish(&self, ctx: &Context) -> Result<()> {
        let file_name = self
            .file
            .as_ref()
            .ok_or_else(|| anyhow!("File name not specified"))?;

        let file_path = if let Some(path) = str_to_path(file_name) {
            path
        } else {
            search_by_file_name(&ctx.project_root_dir, file_name)?
        };

        Publish::try_from((&self.request, file_path))?
            .apply()
            .map(|hash| {
                println!("Hash: {}", hash);
            })
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct ModulePackage {
    modules: Vec<Vec<u8>>,
}

impl ModulePackage {
    pub fn put(&mut self, module: Vec<u8>) {
        self.modules.push(module);
    }

    pub fn sort(&mut self) -> Result<(), Error> {
        let mut modules = Vec::with_capacity(self.modules.len());
        mem::swap(&mut self.modules, &mut modules);

        let mut modules = modules
            .into_iter()
            .map(|bytecode| {
                CompiledModule::deserialize(&bytecode)
                    .map(|unit| (unit.self_id(), (bytecode, unit)))
                    .map_err(|_| anyhow!("Failed to deserialize move module."))
            })
            .collect::<Result<HashMap<_, _>, _>>()?;

        let mut ids_list: Vec<_> = modules.keys().cloned().collect();
        ids_list.sort();

        for id in ids_list {
            self.write_sub_tree(&id, &mut modules);
        }

        Ok(())
    }

    fn write_sub_tree(
        &mut self,
        id: &ModuleId,
        modules: &mut HashMap<ModuleId, (Vec<u8>, CompiledModule)>,
    ) {
        if let Some((bytecode, unit)) = modules.remove(id) {
            let deps = Self::take_deps(id, &unit);
            for dep in deps {
                self.write_sub_tree(&dep, modules);
            }
            println!("Packing '{}'...", id.name());
            self.modules.push(bytecode);
        }
    }

    fn take_deps(id: &ModuleId, unit: &CompiledModule) -> Vec<ModuleId> {
        unit.module_handles()
            .iter()
            .map(|hdl| unit.module_id_for_handle(hdl))
            .filter(|dep_id| dep_id != id)
            .collect()
    }

    pub fn encode(&self) -> Result<Vec<u8>, Error> {
        bcs::to_bytes(&self).map_err(|err| err.into())
    }
}

/// Return file paths from ./PROJECT_FOLDER/build/PROJECT_NAME/bytecode_modules
/// Only with the .mv extension
fn get_bytecode_modules_path(project_dir: &Path, project_name: &str) -> Result<Vec<PathBuf>> {
    let path = project_dir
        .join("build")
        .join(project_name)
        .join("bytecode_modules");
    if !path.exists() {
        return Ok(Vec::new());
    }

    search_by_extension(&path, &["mv"])
}

pub fn run_dove_package_build(ctx: &mut Context) -> Result<()> {
    let build_cmd = MoveCommand::Package {
        cmd: PackageCommand::Build {},
    };
    run_cli(
        ctx.native_functions.clone(),
        &ctx.cost_table,
        &ctx.error_descriptions,
        &ctx.move_args,
        &build_cmd,
    )
}

#[inline]
fn str_to_path(path: &str) -> Option<PathBuf> {
    PathBuf::from_str(path)
        .ok()
        .and_then(|path| path.canonicalize().ok())
}

/// Recursive file search by extension list
fn search_by_extension(path: &Path, list_extension: &[&str]) -> Result<Vec<PathBuf>> {
    let list = fs::read_dir(path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .filter_map(|path| {
            if path.is_dir() {
                search_by_extension(&path, list_extension).ok()
            } else if path.is_file() {
                let ext = path
                    .extension()
                    .and_then(|t| t.to_str())
                    .unwrap_or_default();
                if list_extension.contains(&ext) {
                    Some(vec![path])
                } else {
                    None
                }
            } else {
                None
            }
        })
        .flatten()
        .collect::<Vec<_>>();
    Ok(list)
}

fn search_by_file_name(path_project: &Path, file_name: &str) -> Result<PathBuf> {
    let mut list: Vec<PathBuf> = search_by_extension(path_project, &["mv", "mvt", "pac"])?
        .into_iter()
        .filter(|path| {
            let name_ext = path.file_name().and_then(|name| name.to_str());
            let name = path.file_stem().and_then(|name| name.to_str());
            name_ext == Some(file_name) || name == Some(file_name)
        })
        .collect();
    if list.is_empty() {
        bail!(r#"File named "{}" not found"#, file_name);
    } else if list.len() > 1 {
        let paths_string: String = list.iter().map(|path| path.display()).join("\n");
        bail!(
            r"Found more than one file named {:?}\nSpecify the full path to the file\n{}",
            file_name,
            paths_string
        )
    }
    Ok(list.remove(0))
}
