use std::fs;
use std::path::{PathBuf, Path};
use std::fs::read_to_string;
use toml::Value;
use toml::map::Map;
use structopt::StructOpt;
use move_cli::{Move, run_cli};
use move_core_types::account_address::AccountAddress;
use move_core_types::errmap::ErrorMapping;
use move_cli::Command as MoveCommand;
use move_cli::package::cli::PackageCommand;
use crate::cmd::{Cmd, context_with_empty_manifest};
use crate::context::Context;
use crate::export::create_project_directories;
use crate::{MOVE_STDLIB_URL, MOVE_STDLIB_VERSION};

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

    #[structopt(
        help = "Named  address.",
        long = "addresses",
        short = "a",
        parse(try_from_str = parse_named_address)
    )]
    addresses: Vec<(String, String)>,
}

impl Cmd for New {
    fn context(&mut self, project_dir: PathBuf, move_args: Move) -> anyhow::Result<Context> {
        context_with_empty_manifest(project_dir, move_args)
    }

    fn apply(&mut self, ctx: &mut Context) -> anyhow::Result<()>
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

        add_dialect_addresses_and_stdlib(&project_dir, &ctx.move_args, &self.addresses)?;

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

fn add_dialect_addresses_and_stdlib(
    project_dir: &Path,
    move_args: &Move,
    named_addresses: &[(String, String)],
) -> anyhow::Result<()> {
    let move_toml_path = project_dir.join("Move.toml");
    let mut move_toml = read_to_string(&move_toml_path)
        .map_err(|err| {
            anyhow!(
                "Could not read the file {}\n{:?}",
                move_toml_path.display(),
                err
            )
        })?
        .parse::<Value>()
        .map_err(|err| {
            anyhow!(
                "Failed to convert to TOML {}\n{:?}",
                move_toml_path.display(),
                err
            )
        })?;
    // add to Move.toml - dialect,
    if let Some(dialect) = move_args.dialect.as_ref() {
        let packgage = move_toml
            .get_mut("package")
            .and_then(|package| package.as_table_mut())
            .ok_or_else(|| anyhow!(r#""package" section in "Move.toml" was not found"#))?;
        packgage.insert(
            "dialect".to_string(),
            Value::String(dialect.name().to_string()),
        );
    }

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
    address_table.insert("Std".to_string(), Value::String("0x1".to_string()));

    for (name, address) in named_addresses {
        address_table.insert(name.clone(), Value::String(address.to_string()));
    }

    let move_toml_string = move_toml.to_string() + &dependencies_movestdlib();
    fs::write(&move_toml_path, move_toml_string)?;

    Ok(())
}

/// Move.toml: Dependency movestdlib
pub fn dependencies_movestdlib() -> String {
    format!(
        r#"[dependencies.MoveStdlib]
git = "{}"
rev = "{}""#,
        MOVE_STDLIB_URL, MOVE_STDLIB_VERSION
    )
}

/// Address.
pub fn parse_named_address(s: &str) -> anyhow::Result<(String, String)> {
    let before_after = s.split('=').collect::<Vec<_>>();

    if before_after.len() != 2 {
        anyhow::bail!("Invalid named address assignment. Must be of the form <address_name>=<address>, but found '{}'", s);
    }
    let name = before_after[0].to_string();
    let addr = before_after[1].to_string();
    Ok((name, addr))
}
