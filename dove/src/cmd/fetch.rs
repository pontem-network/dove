use crate::cmd::Cmd;
use crate::context::Context;
use anyhow::Error;
use structopt::StructOpt;

/// Fetch dependencies.
#[derive(StructOpt, Debug)]
#[structopt(setting(structopt::clap::AppSettings::ColoredHelp))]
pub struct Fetch {
    #[structopt(long, hidden = true)]
    color: Option<String>,
}

impl Cmd for Fetch {
    fn apply(self, ctx: Context) -> Result<(), Error> {
        ctx.build_index()?;
        Ok(())
    }
}
