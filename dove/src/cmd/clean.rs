use anyhow::Error;
use std::fs;
use crate::cmd::Cmd;
use crate::context::Context;
use structopt::StructOpt;

/// Clean target directory command.
#[derive(StructOpt, Debug)]
pub struct Clean {}

impl Cmd for Clean {
    fn apply(self, ctx: Context) -> Result<(), Error> {
        let target = ctx.path_for(&ctx.manifest.layout.target);
        if target.exists() {
            fs::remove_dir_all(target).map_err(Into::into)
        } else {
            Ok(())
        }
    }
}
