use std::fs;
use std::ffi::OsStr;
use std::io::Write;
use std::path::{PathBuf, Path};
use std::fs::{remove_file, create_dir_all};
use structopt::StructOpt;
use toml::from_str;
use move_core_types::errmap::ErrorMapping;
use move_core_types::account_address::AccountAddress;
use move_cli::Command as MoveCommand;
use move_cli::package::cli::PackageCommand;
use move_cli::run_cli;
use move_package::BuildConfig;
use crate::cmd::Cmd;
use crate::context::Context;

/// Build dependencies.
#[derive(StructOpt, Debug)]
#[structopt(setting(structopt::clap::AppSettings::ColoredHelp))]
pub struct Build {
    // @todo test
    #[structopt(help = "Generate documentation.", long = "doc", short = "d")]
    doc: bool,

    // @todo test
    /// Generate error map for the package and its dependencies
    /// at path for use by the Move explanation tool.
    #[structopt(long)]
    error_map: Option<String>,

    // Pack the assembled modules into a single file,
    // except for those specified in modules_exclude
    // @todo check
    #[structopt(
        help = "Package modules in a binary file.",
        short = "p",
        long = "package"
    )]
    package: bool,
    // @todo test
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
    // @todo test
    // File name of module package.
    // Used with the "package" parameter.
    #[structopt(help = "File name of module package.", short = "o", long = "output")]
    output: Option<String>,
}

impl Cmd for Build {
    fn apply(&self, ctx: Context) -> anyhow::Result<()>
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
        self.run_error_map(&ctx)?;

        // packaging of modules
        self.run_package(&ctx)?;

        Ok(())
    }
}

impl Build {
    /// Generate error map for the package and its dependencies
    /// at path for use by the Move explanation tool.
    fn run_error_map(&self, ctx: &Context) -> anyhow::Result<()> {
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

        run_cli(
            move_stdlib::natives::all_natives(AccountAddress::from_hex_literal("0x1").unwrap()),
            &error_descriptions,
            &ctx.move_args,
            &cmd,
        )?;
        Ok(())
    }

    /// Names of modules to exclude from the package process..
    fn run_package(&self, ctx: &Context) -> anyhow::Result<()> {
        if !self.package {
            return Ok(());
        }

        // Path to the output file
        let mut output_file_path = PathBuf::from(
            self.output
                .as_ref()
                .map(|s| s.as_str())
                .unwrap_or("package.mv"),
        )
        .with_extension("mv");
        if let Some(parent_path) = output_file_path.parent() {
            if !parent_path.exists() {
                create_dir_all(&parent_path)?;
            }
        }
        if output_file_path.exists() {
            remove_file(&output_file_path)?;
        }

        // Search for modules
        let mut bytecode_modules_path =
            get_bytecode_modules_path(&ctx.project_dir, &ctx.manifest.package.name)
                .unwrap_or_default();

        for module_name in self.modules_exclude.iter() {
            let module_name = if module_name.ends_with(".mv") {
                module_name.to_lowercase()
            } else {
                module_name.to_lowercase() + ".mv"
            };

            if let Some((finded_index, _)) = bytecode_modules_path
                .iter()
                .enumerate()
                .filter_map(|(index, path)| {
                    path.file_name()
                        .map(|file_name| (index, file_name.to_string_lossy().to_lowercase()))
                })
                .find(|(index, file_name)| file_name == &module_name)
            {
                bytecode_modules_path.remove(finded_index);
            }
        }

        // Build into a single file
        if bytecode_modules_path.is_empty() {
            println!("NOTE: No modules for packaging");
            return Ok(());
        }

        let mut file = fs::File::create(&output_file_path)?;
        for path in bytecode_modules_path.iter() {
            let content = fs::read(path)?;
            file.write(&content);
        }

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

/// Return file paths from ./PROJECT_FOLDER/build/PROJECT_NAME/bytecode_modules
/// Only with the .mv extension
fn get_bytecode_modules_path(
    project_dir: &Path,
    project_name: &str,
) -> anyhow::Result<Vec<PathBuf>> {
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
