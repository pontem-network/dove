use anyhow::Error;
use move_lang::shared::Flags;
use move_lang::unwrap_or_report_errors;
use structopt::StructOpt;
use lang::compiler::check;
use crate::cmd::Cmd;
use crate::context::Context;

/// Check project.
#[derive(StructOpt, Debug)]
#[structopt(setting(structopt::clap::AppSettings::ColoredHelp))]
pub struct Check {}

impl Cmd for Check {
    fn apply(self, ctx: Context) -> Result<(), Error> {
        let (_, interface) = ctx.build_index()?;
        let (files, res) = check(
            &[
                ctx.str_path_for(&ctx.manifest.layout.scripts_dir)?,
                ctx.str_path_for(&ctx.manifest.layout.modules_dir)?,
            ],
            &[interface.dir.to_string_lossy().to_string()],
            ctx.dialect.as_ref(),
            &ctx.account_address_str()?,
            None,
            Flags::empty(),
        )?;
        unwrap_or_report_errors!(files, res);

        Ok(())
    }
}
