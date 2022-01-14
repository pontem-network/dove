use std::path::PathBuf;
use structopt::StructOpt;
use lazy_static::lazy_static;
use regex::Regex;
use move_cli::Move;
use crate::cmd::{Cmd, context_with_empty_manifest};
use crate::context::Context;
use crate::export::create_project_directories;
use crate::cmd::new::dependencies_movestdlib;
use crate::cmd::new::parse_named_address;

/// Init project command.
#[derive(StructOpt, Debug)]
#[structopt(setting(structopt::clap::AppSettings::ColoredHelp))]
pub struct Init {
    #[structopt(
        help = "Creates only Dove.toml.",
        name = "minimal",
        long = "minimal",
        short = "m"
    )]
    minimal: bool,
    #[structopt(help = "Named  address.", long = "addresses", short = "a", parse(try_from_str = parse_named_address))]
    addresses: Vec<(String, String)>,
}

impl Cmd for Init {
    fn context(&mut self, project_dir: PathBuf, move_args: Move) -> anyhow::Result<Context> {
        context_with_empty_manifest(project_dir, move_args)
    }

    fn apply(&mut self, ctx: &mut Context) -> anyhow::Result<()>
    where
        Self: Sized,
    {
        let project_dir = ctx.project_dir.as_path();
        let move_toml_path = project_dir.join("Move.toml");
        anyhow::ensure!(
            !move_toml_path.exists(),
            "init cannot be run on existing project."
        );

        let project_name = project_dir
            .file_name()
            .and_then(|name| name.to_str())
            .ok_or_else(|| anyhow!("Failed to extract directory name."))?;
        anyhow::ensure!(
            is_valid_name(project_name),
            r#"Invalid project name "{}". Allowed symbols a-z, A-Z, 0-9,_,-"#,
            project_name
        );

        let move_toml_string = move_toml_new(project_name, &ctx.move_args, &self.addresses);
        std::fs::write(move_toml_path, move_toml_string)?;

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
            create_project_directories(project_dir)?;
        }

        println!(
            "Project successfully initialized in {}",
            project_dir.display()
        );

        Ok(())
    }
}

fn is_valid_name(text: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^[\w\-_]{1,64}$").unwrap();
    }
    RE.is_match(text)
}

fn move_toml_new(project_name: &str, move_args: &Move, addresses: &[(String, String)]) -> String {
    let mut move_toml_string = format!(
        "\
        [package]\n\
        name = \"{}\"\n\
        version = \"0.0.0\"\n\
        ",
        project_name
    );

    if let Some(dialect_name) = move_args.dialect.map(|dialect| dialect.name()) {
        move_toml_string += format!("dialect = \"{}\"\n", dialect_name).as_str();
    }

    move_toml_string += "\n[addresses]\n";
    move_toml_string += "Std = \"0x1\"\n";
    for (name, address) in addresses {
        move_toml_string += format!("{} = \"{}\"\n", name, address).as_str();
    }

    move_toml_string + &dependencies_movestdlib()
}
