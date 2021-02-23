use crate::cmd::Cmd;
use crate::context::Context;
use anyhow::Error;
use structopt::StructOpt;

/// Fetch dependencies.
#[derive(StructOpt, Debug)]
pub struct Fetch {}

impl Cmd for Fetch {
    fn apply(self, ctx: Context) -> Result<(), Error> {
        ctx.build_index()?;
        Ok(())
    }
}
