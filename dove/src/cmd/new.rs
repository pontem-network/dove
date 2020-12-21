use anyhow::Error;
use http::Uri;
use std::fs;
use crate::cmd::Cmd;
use crate::context::{Context, create_context};
use libra::prelude::*;
use crate::cmd::init::Init;
use structopt::StructOpt;

/// Create project command.
#[derive(StructOpt, Debug)]
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
        name = "Dialect",
        long = "dialect",
        short = "d"
    )]
    dialect: Option<String>,
}

impl Cmd for New {
    fn context(&self) -> Result<Context, Error> {
        create_context()
    }

    fn apply(self, mut ctx: Context) -> Result<(), Error> {
        Identifier::new(self.project_name.as_str())?;

        let project_dir = ctx.project_dir.join(&self.project_name);
        if project_dir.exists() {
            return Err(anyhow!("destination `{:?}` already exists", project_dir));
        }

        fs::create_dir(&project_dir)?;

        ctx.project_dir = project_dir;
        let init = Init::new(self.repository, self.address, self.dialect);
        init.apply(ctx)
    }
}
