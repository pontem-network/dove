use std::path::PathBuf;
use std::string::ToString;
use structopt::StructOpt;
use lazy_static::lazy_static;
use move_cli::Move;
use regex::Regex;
use crate::cmd::{Cmd, context_with_empty_manifest};
use crate::context::Context;
use crate::export::create_project_directories;

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
    #[structopt(long, hidden = true)]
    color: Option<String>,
}

impl Init {
    /// Creates a new Init command.
    pub fn new(minimal: bool) -> Init {
        Init {
            minimal,
            color: None,
        }
    }
}

impl Cmd for Init {
    fn context(&self, project_dir: PathBuf, move_args: Move) -> anyhow::Result<Context> {
        context_with_empty_manifest(project_dir, move_args)
    }

    fn apply(&self, ctx: Context) -> anyhow::Result<()>
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
        let dialect_name = ctx
            .move_args
            .dialect
            .map_or("pont", |dialect| dialect.name());
        let move_toml_string = move_toml_new(project_name, dialect_name);
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
            create_project_directories(&project_dir)?;
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

fn move_toml_new(project_name: &str, dialect: &str) -> String {
    format!(
        r#"[package]
name = "{}"
version = "0.0.0"
dialect = "{}"
"#,
        project_name, dialect
    )
}
