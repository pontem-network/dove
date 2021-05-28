use crate::cmd::Cmd;
use crate::context::Context;
use anyhow::Error;
use structopt::StructOpt;
use crate::stdoutln;
use crate::stdout::colorize::good;

/// Fetch dependencies.
#[derive(StructOpt, Debug)]
#[structopt(setting(structopt::clap::AppSettings::ColoredHelp))]
pub struct Fetch {
    #[structopt(long, hidden = true)]
    color: Option<String>,
}

impl Cmd for Fetch {
    fn apply(self, ctx: Context) -> Result<(), Error> {
        stdoutln!("Build project index...");
        ctx.build_index()?;
        stdoutln!("Fetch dependencies {}", good("completed"));
        Ok(())
    }
}
