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
use serde::{Deserialize, Serialize};
use structopt::StructOpt;
use termcolor::{ColorChoice, StandardStream};

use lang::compiler::file::load_move_files_with_filter;
use lang::flow::builder::{Artifacts, MoveBuilder, StaticResolver};

use crate::cmd::{Cmd, load_dependencies};
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
    #[structopt(help = "File name of module package.", short = "o", long = "output")]
    output: Option<String>,
    #[structopt(
        help = "Names of files excluded from the build process. \
        If the excluded name does not contain an extension '.move' is interpreted as the module name. \
        This is useful when the 'tree' flag is used and transitive dependencies are added to target.\
        In this case the dependencies listed in exclude will be excluded.",
        short = "e",
        long = "exclude"
    )]
    exclude: Vec<String>,
    #[structopt(
        help = "Do not specify the order of modules.",
        short = "u",
        long = "unordered"
    )]
    unordered: bool,
    #[structopt(long, hidden = true)]
    color: Option<String>,
}

impl Cmd for Build {
    fn apply(self, ctx: Context) -> Result<(), Error> {
        let dirs = ctx.paths_for(&[
            &ctx.manifest.layout.scripts_dir,
            &ctx.manifest.layout.modules_dir,
        ]);

        // Build project index...
        let mut index = ctx.build_index()?;

        // Load dependencies by set of path...
        let dep_set = index.make_dependency_set(&dirs)?;
        let mut dep_list = load_dependencies(dep_set)?;

        let (exclude_files, exclude_modules): (Vec<_>, Vec<_>) =
            self.exclude.iter().partition(|e| e.ends_with(".move"));

        let exclude_files = exclude_files
            .iter()
            .map(|exclude| ctx.path_for(exclude))
            .collect::<Vec<_>>();

        let exclude_files = exclude_files
            .iter()
            .map(|exclude| exclude.as_os_str())
            .collect::<Vec<_>>();

        let mut source_list = load_move_files_with_filter(&dirs, &|path| {
            !exclude_files.contains(&path.as_os_str())
        })?;

        if self.tree {
            source_list.extend(dep_list);
            dep_list = vec![];
        }

        let source_ref = source_list.iter().collect::<Vec<_>>();

        // Build move files...
        let sender = ctx.account_address()?;
        let Artifacts { files, prog } = MoveBuilder::new(
            ctx.dialect.as_ref(),
            Some(sender),
            StaticResolver::new(dep_list),
        )
        .build(&source_ref);

        match prog {
            Err(errors) => {
                let mut writer = StandardStream::stderr(ColorChoice::Auto);
                output_errors(&mut writer, files, errors);
                Err(anyhow!("could not compile:{}", ctx.project_name()))
            }
            Ok(compiled_units) => {
                // Verify and store compilation results...
                self.verify_and_store(&ctx, files, compiled_units, &exclude_modules)?;
                Ok(())
            }
        }
    }
}

impl Build {
    /// Verify and store compilation results.
    fn verify_and_store(
        &self,
        ctx: &Context,
        files: FilesSourceText,
        compiled_units: Vec<CompiledUnit>,
        exclude_modules: &[&String],
    ) -> Result<(), Error> {
        let (compiled_units, ice_errors) = compiled_unit::verify_units(compiled_units);
        let (modules, scripts): (Vec<_>, Vec<_>) = compiled_units
            .into_iter()
            .filter(|u| !exclude_modules.contains(&&u.name()))
            .partition(|u| matches!(u, CompiledUnit::Module { .. }));

        self.store_modules(ctx, modules)?;
        self.store_scripts(ctx, scripts)?;

        if !ice_errors.is_empty() {
            let mut writer = StandardStream::stderr(ColorChoice::Auto);
            output_errors(&mut writer, files, ice_errors);
            Err(anyhow!("could not verify:{}", ctx.project_name()))
        } else {
            Ok(())
        }
    }

    fn store_modules(&self, ctx: &Context, units: Vec<CompiledUnit>) -> Result<(), Error> {
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

                self.store_units(ctx, units, &modules_dir)?;
            }
        }
        Ok(())
    }

    fn store_scripts(&self, ctx: &Context, units: Vec<CompiledUnit>) -> Result<(), Error> {
        if !units.is_empty() {
            let scripts_dir = ctx.path_for(&ctx.manifest.layout.scripts_output);
            if scripts_dir.exists() {
                fs::remove_dir_all(&scripts_dir)?;
            }
            fs::create_dir_all(&scripts_dir)?;

            self.store_units(ctx, units, &scripts_dir)?;
        }
        Ok(())
    }

    fn store_units(
        &self,
        ctx: &Context,
        units: Vec<CompiledUnit>,
        base_dir: &Path,
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

            File::create(&path)?.write_all(&bytecode)?
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
