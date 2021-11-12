use std::fs;
use std::path::{PathBuf, Path};
use std::fs::read_to_string;
use structopt::StructOpt;
use toml::Value;

use move_cli::{Move, run_cli};
use move_core_types::account_address::AccountAddress;
use move_core_types::errmap::ErrorMapping;
use move_cli::Command as MoveCommand;
use move_cli::package::cli::PackageCommand;
use move_package::BuildConfig;

use crate::cmd::{Cmd, context_with_empty_manifest};
use crate::context::Context;
use crate::export::create_project_directories;
use std::collections::HashMap;
use toml::map::Map;

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

        add_dialect_and_addresses(&project_dir, &ctx.move_args)?;

        if !self.minimal {
            println!(
                "Creating default directories(to omit those, use --minimal): \n\
                        \t./sources\n\
                        \t./examples\n\
                        \t./scripts\n\
                        \t./doc_templates\n\
                        \t./tests"
            );
            // Create directories - "sources", "examples", "scripts", "doc_templates", "tests"
            create_project_directories(&project_dir)?;
        }
        println!(
            "Project was successfully created in {}",
            project_dir.display()
        );

        Ok(())
    }
}

fn add_dialect_and_addresses(project_dir: &Path, move_args: &Move) -> anyhow::Result<()> {
    if move_args.dialect.is_none() && move_args.named_addresses.is_empty() {
        return Ok(());
    }

    let move_toml_path = project_dir.join("Move.toml");
    let mut move_toml = read_to_string(&move_toml_path)?.parse::<Value>()?;
    // add to Move.toml - dialect,
    if let Some(dialect) = move_args.dialect.as_ref() {
        let packgage = move_toml
            .get_mut("package")
            .and_then(|package| package.as_table_mut())
            .ok_or(anyhow!(r#""package" section in "Move.toml" was not found"#))?;
        packgage.insert(
            "dialect".to_string(),
            Value::String(dialect.name().to_string()),
        );
    }

    if !move_args.named_addresses.is_empty() {
        if move_toml.get_mut("addresses").is_none() {
            let new_table = Value::Table(Map::new());
            move_toml
                .as_table_mut()
                .unwrap()
                .insert("addresses".to_string(), new_table);
        }
        let address_table = move_toml
            .get_mut("addresses")
            .and_then(|value| value.as_table_mut())
            .ok_or_else(|| anyhow!(r#"Couldn't get the "addresses" section in "Move.toml""#))?;

        for (name, address) in &move_args.named_addresses {
            address_table.insert(name.clone(), Value::String(address.to_string()));
        }
    }

    fs::write(&move_toml_path, move_toml.to_string())?;
    Ok(())
}
