use std::fs;
use std::path::{PathBuf, Path};
use std::fs::read_to_string;
use anyhow::Error;
use structopt::StructOpt;
use toml::Value;

use dialect::Dialect;
use move_cli::{Move, run_cli};
use move_core_types::account_address::AccountAddress;
use move_core_types::errmap::ErrorMapping;
use move_cli::Command as MoveCommand;
use move_cli::package::cli::PackageCommand;
use move_package::BuildConfig;

use crate::cmd::{Cmd, context_with_empty_manifest};
use crate::context::Context;
use crate::export::create_project_directories;

/// Create project command.
#[derive(StructOpt, Debug)]
#[structopt(setting(structopt::clap::AppSettings::ColoredHelp))]
pub struct New {
    #[structopt(help = "Project name.")]
    project_name: String,
    #[structopt(
        help = "Creates only Move.toml without dependencies.",
        name = "minimal",
        long = "minimal",
        short = "m"
    )]
    minimal: bool,
    #[structopt(long, hidden = true)]
    color: Option<String>,
}

impl Cmd for New {
    fn context(&self, project_dir: PathBuf, move_args: Move) -> anyhow::Result<Context> {
        context_with_empty_manifest(project_dir, move_args)
    }

    fn apply(&self, ctx: Context) -> anyhow::Result<()>
    where
        Self: Sized,
    {
        // for Move-cli
        let error_descriptions: ErrorMapping =
            bcs::from_bytes(move_stdlib::error_descriptions())?;
        let cmd = MoveCommand::Package {
            cmd: PackageCommand::New {
                name: self.project_name.clone(),
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

        let project_dir = ctx.project_dir.join(&self.project_name);
        if !project_dir.exists() {
            bail!("Failed to create a project")
        }
        let move_toml_path = project_dir.join("Move.toml");
        // add to Move.toml - dialect,
        if let Some(dialect) = ctx.move_args.dialect.as_ref() {
            add_dialect(&move_toml_path, dialect)?;
        }

        if self.minimal {
            return Ok(());
        }

        // Create directories - "sources", "examples", "scripts", "doc_templates", "tests"
        create_project_directories(&project_dir)
    }
}

fn add_dialect(move_toml_path: &Path, dialect: &Dialect) -> anyhow::Result<()> {
    let mut move_toml = read_to_string(move_toml_path)?.parse::<Value>()?;
    let packgage = move_toml
        .get_mut("package")
        .and_then(|package| package.as_table_mut())
        .ok_or(anyhow!(r#""package" section in "Move.toml" was not found"#))?;
    packgage.insert(
        "dialect".to_string(),
        Value::String(dialect.name().to_string()),
    );
    fs::write(&move_toml_path, move_toml.to_string())?;
    Ok(())
}
