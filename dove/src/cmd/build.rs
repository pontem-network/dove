use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use anyhow::Error;
use move_lang::{
    compiled_unit,
    errors::{FilesSourceText, output_errors},
};
use move_lang::compiled_unit::CompiledUnit;
use move_lang::unwrap_or_report_errors;
use serde::{Deserialize, Serialize};
use structopt::StructOpt;
use termcolor::{ColorChoice, StandardStream};

use lang::compiler::build;
use lang::compiler::file::find_move_files;

use crate::cmd::Cmd;
use crate::cmd::docgen::DocGen;
use crate::context::Context;
use crate::stdoutln;

/// Build dependencies.
#[derive(StructOpt, Debug)]
#[structopt(setting(structopt::clap::AppSettings::ColoredHelp))]
pub struct Build {
    #[structopt(
        help = "Add transitive dependencies to the target.",
        short = "t",
        long = "tree"
    )]
    tree: bool,
    #[structopt(
        help = "Package modules in a binary file.",
        short = "p",
        long = "package"
    )]
    package: bool,
    #[structopt(help = "Emit source map.", short = "s", long = "emit_source_maps")]
    emit_source_maps: bool,
    #[structopt(help = "File name of module package.", short = "o", long = "output")]
    output: Option<String>,
    #[structopt(
        help = "Names of files or directory excluded from the build process.",
        short = "e",
        long = "exclude"
    )]
    exclude: Vec<String>,
    #[structopt(
        help = "Names of modules to exclude from the package process..",
        long = "modules_exclude"
    )]
    modules_exclude: Vec<String>,
    #[structopt(
        help = "Do not specify the order of modules.",
        short = "u",
        long = "unordered"
    )]
    unordered: bool,
    #[structopt(help = "Generate documentation.", long = "doc", short = "d")]
    doc: bool,
    #[structopt(long, hidden = true)]
    color: Option<String>,
}

impl Cmd for Build {
    fn apply(self, ctx: Context) -> Result<(), Error> {
        let dirs = ctx.paths_for(&[
            &ctx.manifest.layout.scripts_dir,
            &ctx.manifest.layout.modules_dir,
        ]);

        let index = ctx.build_index()?;

        let mut dep_list = index.into_deps_roots();

        let (exclude_files, exclude_dirs): (Vec<_>, Vec<_>) =
            self.exclude.iter().partition(|e| e.ends_with(".move"));

        let exclude_dirs = ctx.paths_for(&exclude_dirs);

        let mut source_list = find_move_files(&dirs)
            .filter_map(|path| path.canonicalize().ok())
            .filter(|path| {
                !(exclude_dirs.iter().any(|dir| path.starts_with(dir))
                    || exclude_files.iter().any(|file| path.ends_with(file)))
            })
            .map(|path| path.to_string_lossy().to_string())
            .collect::<Vec<_>>();

        if self.tree {
            source_list.extend(dep_list);
            dep_list = vec![];
        }

        let sender = ctx.account_address()?;

        let interface_files_dir = ctx.interface_files_dir();
        if !interface_files_dir.exists() {
            fs::create_dir_all(&interface_files_dir)?;
        }

        let (files, res) = build(
            &source_list,
            &dep_list,
            ctx.dialect.as_ref(),
            Some(sender),
            Some(interface_files_dir.to_string_lossy().to_string()),
        )?;
        let units = unwrap_or_report_errors!(files, res);

        self.verify_and_store(
            &ctx,
            files,
            units,
            &self.modules_exclude,
            self.emit_source_maps,
        )?;

        if self.doc {
            let doc = DocGen {};
            doc.apply(ctx)?;
        }

        Ok(())
    }
}

impl Build {
    /// Verify and store compilation results.
    fn verify_and_store(
        &self,
        ctx: &Context,
        files: FilesSourceText,
        compiled_units: Vec<CompiledUnit>,
        exclude_modules: &[String],
        emit_source_maps: bool,
    ) -> Result<(), Error> {
        let (compiled_units, ice_errors) = compiled_unit::verify_units(compiled_units);
        let (modules, scripts): (Vec<_>, Vec<_>) = compiled_units
            .into_iter()
            .filter(|u| !exclude_modules.contains(&u.name()))
            .partition(|u| matches!(u, CompiledUnit::Module { .. }));

        self.store_modules(ctx, modules, emit_source_maps)?;
        self.store_scripts(ctx, scripts, emit_source_maps)?;

        if !ice_errors.is_empty() {
            let mut writer = StandardStream::stderr(ColorChoice::Auto);
            output_errors(&mut writer, files, ice_errors);
            Err(anyhow!("could not verify:{}", ctx.project_name()))
        } else {
            Ok(())
        }
    }

    fn store_modules(
        &self,
        ctx: &Context,
        units: Vec<CompiledUnit>,
        emit_source_maps: bool,
    ) -> Result<(), Error> {
        if !units.is_empty() {
            if self.package {
                let packages_dir = ctx.path_for(&ctx.manifest.layout.bundles_output);
                if !packages_dir.exists() {
                    fs::create_dir_all(&packages_dir)?;
                }

                let pac_file = match &self.output {
                    None => {
                        let mut pac_file = match &ctx.manifest.package.name {
                            None => packages_dir.join("modules"),
                            Some(pac_name) => packages_dir.join(pac_name),
                        };
                        pac_file.set_extension("pac");
                        pac_file
                    }
                    Some(name) => {
                        let mut pac_file = packages_dir.join(name);
                        if !name.to_lowercase().ends_with(".pac") {
                            pac_file.set_extension("pac");
                        }
                        pac_file
                    }
                };

                stdoutln!("Package content: ");
                for unit in &units {
                    stdoutln!("\t{}", unit.name());
                }
                stdoutln!("Store: {:?}", pac_file.as_os_str());
                let package = ModulePackage::with_units(units);
                File::create(&pac_file)?.write_all(&package.encode()?)?
            } else {
                let modules_dir = ctx.path_for(&ctx.manifest.layout.modules_output);
                if modules_dir.exists() {
                    fs::remove_dir_all(&modules_dir)?;
                }
                fs::create_dir_all(&modules_dir)?;

                self.store_units(ctx, units, &modules_dir, emit_source_maps)?;
            }
        }
        Ok(())
    }

    fn store_scripts(
        &self,
        ctx: &Context,
        units: Vec<CompiledUnit>,
        emit_source_maps: bool,
    ) -> Result<(), Error> {
        if !units.is_empty() {
            let scripts_dir = ctx.path_for(&ctx.manifest.layout.scripts_output);
            if scripts_dir.exists() {
                fs::remove_dir_all(&scripts_dir)?;
            }
            fs::create_dir_all(&scripts_dir)?;

            self.store_units(ctx, units, &scripts_dir, emit_source_maps)?;
        }
        Ok(())
    }

    fn store_units(
        &self,
        ctx: &Context,
        units: Vec<CompiledUnit>,
        base_dir: &Path,
        emit_source_maps: bool,
    ) -> Result<(), Error> {
        for (idx, unit) in units.into_iter().enumerate() {
            let mut path = if !self.unordered {
                base_dir.join(format!("{}_{}", idx, unit.name()))
            } else {
                base_dir.join(unit.name())
            };

            path.set_extension("mv");
            let mut bytecode = unit.serialize();
            ctx.dialect.adapt_to_target(&mut bytecode)?;

            File::create(&path)?.write_all(&bytecode)?;

            if emit_source_maps {
                path.set_extension("mvsm");
                File::create(&path)?.write_all(&unit.serialize_source_map())?;
            }
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct ModulePackage {
    modules: Vec<Vec<u8>>,
}

impl ModulePackage {
    fn with_units(units: Vec<CompiledUnit>) -> ModulePackage {
        ModulePackage {
            modules: units.into_iter().map(|unit| unit.serialize()).collect(),
        }
    }

    pub fn encode(&self) -> Result<Vec<u8>, Error> {
        bcs::to_bytes(&self).map_err(|err| err.into())
    }
}
