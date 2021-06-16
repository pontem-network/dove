use anyhow::Error;
use structopt::StructOpt;
use move_executor::executor::render_execution_result;

use crate::cmd::Cmd;
use crate::context::Context;
use crate::transaction::TransactionBuilder;

/// Run script.
#[derive(StructOpt, Debug)]
#[structopt(setting(structopt::clap::AppSettings::ColoredHelp))]
pub struct Run {
    #[structopt(help = "Script call declaration.\
        Example: 'create_balance([10,10], true, 68656c6c6f776f726c64, 100, 0x1)'")]
    call: Option<String>,
    #[structopt(help = "Script name.", long = "name", short = "n")]
    script_name: Option<String>,
    #[structopt(help = "Script file name.", long = "file", short = "f")]
    file_name: Option<String>,
    #[structopt(
        help = r#"Number of script main() function arguments in quotes, e.g. "10 20 30""#,
        name = "Script args.",
        long = "args",
        short = "a"
    )]
    args: Option<Vec<String>>,
    #[structopt(name = "Script signers.", long = "signers", short = "s")]
    signers: Option<Vec<String>>,
    #[structopt(long, hidden = true)]
    color: Option<String>,
}

impl Cmd for Run {
    fn apply(self, ctx: Context) -> Result<(), Error> {
        let trbuild = TransactionBuilder::from_run_cmd(&self, &ctx)?;
        render_execution_result(trbuild.run())
    }
}

impl<'a> TransactionBuilder<'a> {
    /// Create a TransactionBuilder based on the transmitted data
    pub fn from_run_cmd(cmd: &'a Run, ctx: &'a Context) -> Result<TransactionBuilder<'a>, Error> {
        let mut trbuild = Self::new(ctx);
        trbuild.script_file_name = cmd.file_name.clone();
        trbuild
            .set_from_cmd_call(cmd.call.as_ref())?
            .set_script_name_from_cmd(cmd.script_name.clone())
            .set_args_from_cmd(cmd.args.clone())
            .set_signers_from_cmd(cmd.signers.clone())?;

        Ok(trbuild)
    }
}
