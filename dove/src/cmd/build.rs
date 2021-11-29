use core::mem;
use std::collections::HashMap;
use std::fs;
use std::fs::{remove_file};
use std::ffi::OsStr;
use std::fs::remove_file;
use std::path::{PathBuf, Path};
use anyhow::Error;
use structopt::StructOpt;
use anyhow::Result;
use move_binary_format::access::ModuleAccess;
use move_binary_format::CompiledModule;
use move_core_types::errmap::ErrorMapping;
use move_core_types::account_address::AccountAddress;
use move_cli::Command as MoveCommand;
use move_cli::package::cli::PackageCommand;
use move_cli::run_cli;
use move_core_types::language_storage::ModuleId;
use move_package::BuildConfig;
use move_symbol_pool::Symbol;

use crate::cmd::Cmd;
use crate::context::Context;
use serde::{Serialize, Deserialize};

/// Build dependencies.
#[derive(StructOpt, Debug, Default)]
#[structopt(setting(structopt::clap::AppSettings::ColoredHelp))]
pub struct Build {
    #[structopt(help = "Generate documentation.", long = "doc", short = "d")]
    doc: bool,

    /// Generate error map for the package and its dependencies
    /// at path for use by the Move explanation tool.
    #[structopt(long)]
    error_map: Option<String>,
    /// Address. Used as an additional parameter in error_map
    #[structopt(long)]
    address: Option<String>,

    // Pack the assembled modules into a single file,
    // except for those specified in modules_exclude
    #[structopt(
        help = "Package modules in a binary file.",
        short = "p",
        long = "package"
    )]
    package: bool,
    // Names of modules to exclude from the package process..
    // Used with the "package" parameter.
    // Modules are taken from the ./build/NAME_PROJECT/bytecode_modules directory.
    // The names are case-insensitive and can be specified with an extension.mv or without it.
    // -modules_exclude NAME_1 NAME_2 NAME_3.mv
    #[structopt(
        help = "Names of modules to exclude from the package process..",
        long = "modules_exclude"
    )]
    modules_exclude: Vec<String>,
    // File name of module package.
    // Used with the "package" parameter.
    #[structopt(help = "File name of module package.", short = "o", long = "output")]
    output: Option<String>,
}

impl Cmd for Build {
    fn apply(&mut self, ctx: &mut Context) -> anyhow::Result<()>
    where
        Self: Sized,
    {
        // Move-cli build
        let error_descriptions: ErrorMapping =
            bcs::from_bytes(move_stdlib::error_descriptions())?;
        let cmd = MoveCommand::Package {
            cmd: PackageCommand::Build {},
            path: Some(ctx.project_dir.clone()),
            config: BuildConfig {
                generate_abis: false,
                generate_docs: self.doc,
                test_mode: false,
                dev_mode: false,
            },
        };

        run_cli(
            move_stdlib::natives::all_natives(AccountAddress::from_hex_literal("0x1").unwrap()),
            &error_descriptions,
            &ctx.move_args,
            &cmd,
        )?;

        // Move-cli error map
        self.run_error_map(ctx)?;

        // packaging of modules
        self.run_package(ctx)?;

        // Checking directories in the "build" section, if there are none, then create
        checking_build_directories(ctx)?;

        // Checking directories in the "build" section, if there are none, then create
        checking_build_directories(ctx)?;

        Ok(())
    }
}

impl Build {
    /// Generate error map for the package and its dependencies
    /// at path for use by the Move explanation tool.
    fn run_error_map(&self, ctx: &Context) -> Result<()> {
        if self.error_map.is_none() {
            return Ok(());
        }

        let path = PathBuf::from(self.error_map.clone().unwrap_or_default());

        let error_descriptions: ErrorMapping =
            bcs::from_bytes(move_stdlib::error_descriptions())?;
        let cmd = MoveCommand::Package {
            cmd: PackageCommand::ErrMapGen {
                error_prefix: None,
                output_file: path,
            },
            path: Some(ctx.project_dir.clone()),
            config: BuildConfig {
                generate_abis: false,
                generate_docs: false,
                test_mode: false,
                dev_mode: false,
            },
        };

        let address = self.address.clone().unwrap_or_else(|| "0x1".to_string());
        let account = if !address.starts_with("0x") {
            ctx.manifest
                .addresses
                .as_ref()
                .and_then(|list| list.get(&Symbol::from(address.as_str())).cloned())
                .and_then(|add| add)
                .unwrap_or(AccountAddress::from_hex_literal("0x1")?)
        } else {
            AccountAddress::from_hex_literal(&address)?
        };

        run_cli(
            move_stdlib::natives::all_natives(account),
            &error_descriptions,
            &ctx.move_args,
            &cmd,
        )?;
        Ok(())
    }

    /// Names of modules to exclude from the package process..
    fn run_package(&self, ctx: &Context) -> Result<()> {
        if !self.package {
            return Ok(());
        }

        // Path to the output file
        let output_file_path = ctx
            .bundles_output_path(
                self.output
                    .as_deref()
                    .unwrap_or_else(|| ctx.manifest.package.name.as_str()),
            )?
            .with_extension("pac");
        if output_file_path.exists() {
            remove_file(&output_file_path)?;
        }

        // Search for modules
        let bytecode_modules_path =
            get_bytecode_modules_path(&ctx.project_dir, &ctx.manifest.package.name)
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
}

/// Checking directories in the "build" section, if there are none, then create
/// Fixes an error when reassembling an empty project
///     <PROJECT_DIR>/build/<PROJECT_NAME>/bytecode_modules
///     <PROJECT_DIR>/build/<PROJECT_NAME>/bytecode_scripts
///     <PROJECT_DIR>/build/<PROJECT_NAME>/source_maps
///     <PROJECT_DIR>/build/<PROJECT_NAME>/sources
fn checking_build_directories(ctx: &Context) -> Result<()> {
    let build_path = ctx
        .project_dir
        .join("build")
        .join(ctx.manifest.package.name.as_str());
    for path in [
        build_path.join("bytecode_modules"),
        build_path.join("bytecode_scripts"),
        build_path.join("source_maps"),
        build_path.join("sources"),
    ] {
        if path.exists() {
            continue;
        }
        fs::create_dir_all(&path)?;
    }
    Ok(())
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

    let list = fs::read_dir(path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, _>>()
        .map(|list| {
            list.into_iter()
                .filter(|path| path.is_file() && path.extension() == Some(OsStr::new("mv")))
                .collect::<Vec<_>>()
        })?;
    Ok(list)
}

/// Build project.
pub fn run_internal_build(ctx: &mut Context) -> Result<(), Error> {
    let mut cmd = Build::default();
    cmd.apply(ctx)
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

        let ids_list: Vec<_> = modules.keys().cloned().collect();

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
