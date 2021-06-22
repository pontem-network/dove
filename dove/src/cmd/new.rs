use anyhow::Error;
use http::Uri;
use std::fs;
use crate::cmd::Cmd;
use crate::context::{Context, get_context};
use crate::cmd::init::Init;
use structopt::StructOpt;
use std::path::PathBuf;
use crate::manifest::DoveToml;
use crate::stdoutln;

/// Create project command.
#[derive(StructOpt, Debug)]
#[structopt(setting(structopt::clap::AppSettings::ColoredHelp))]
pub struct New {
    #[structopt(help = "Project name.")]
    project_name: String,
    #[structopt(
        help = "Basic uri to blockchain api.",
        name = "Blockchain API",
        long = "repo",
        short = "r"
    )]
    repository: Option<Uri>,
    #[structopt(
        help = "Account address.",
        name = "address",
        long = "address",
        short = "a"
    )]
    address: Option<String>,
    #[structopt(
        help = "Compiler dialect",
        default_value = "pont",
        name = "Dialect",
        long = "dialect",
        short = "d"
    )]
    dialect: String,
    #[structopt(long, hidden = true)]
    color: Option<String>,
    #[structopt(
        help = "Creates only Dove.toml without dependencies.",
        name = "minimal",
        long = "minimal",
        short = "m"
    )]
    minimal: bool,
}

impl Cmd for New {
    fn context(&self, project_dir: PathBuf) -> Result<Context, Error> {
        let manifest = DoveToml::default();
        get_context(project_dir, manifest)
    }

    fn apply(self, mut ctx: Context) -> Result<(), Error> {
        let project_dir = ctx.project_dir.join(&self.project_name);
        if project_dir.exists() {
            return Err(anyhow!("destination `{:?}` already exists", project_dir));
        }

        stdoutln!("Creating project directory...");
        fs::create_dir(&project_dir)?;

        ctx.project_dir = project_dir.clone();
        let init = Init::new(self.repository, self.address, self.dialect, self.minimal);
        if let Err(err) = init.apply(ctx) {
            fs::remove_dir_all(&project_dir)?;
            Err(err)
        } else {
            Ok(())
        }
    }
}
