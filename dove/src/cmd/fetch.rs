use crate::cmd::Cmd;
use crate::context::Context;
use anyhow::Error;
use structopt::StructOpt;
use crate::index::Index;

/// Fetch dependencies.
#[derive(StructOpt, Debug)]
pub struct Fetch {}

impl Cmd for Fetch {
    fn apply(self, ctx: Context) -> Result<(), Error> {
        let mut index = Index::load(&ctx)?;
        index.build()?;
        Ok(())
    }
}
