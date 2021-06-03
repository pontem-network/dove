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
        let artifacts = ctx.path_for(&ctx.manifest.layout.artifacts);
        let index_path = ctx.path_for(&ctx.manifest.layout.index);

        if index_path.exists() {
            fs::remove_file(index_path)?;
        }

        if artifacts.exists() {
            fs::remove_dir_all(artifacts).map_err(Into::into)
        } else {
            Ok(())
        }
    }
}
