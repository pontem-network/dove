use anyhow::Error;
use structopt::StructOpt;

use move_executor::executor::render_execution_result;

use crate::cmd::Cmd;
use crate::context::Context;
use crate::transaction::TransactionBuilder;

/// Run move script
#[derive(StructOpt, Debug)]
#[structopt(setting(structopt::clap::AppSettings::ColoredHelp))]
#[structopt(usage = "dove run [call] [OPTIONS]\n
    Examples:
    $ dove run 'script_name([10,10], true, 68656c6c6f776f726c64, 100, 0x1)' --f file_name
    $ dove run --file file_name --name script_name -a [10,10] true 68656c6c6f776f726c64 100 0x1
    $ dove run 'script_name()' --signers 0x1 0x2\
")]
pub struct Run {
    #[structopt(help = "Script call declaration.\n\
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
        let tr_build = TransactionBuilder::from_run_cmd(self, &ctx)?;
        render_execution_result(tr_build.run())
    }
}

impl<'a> TransactionBuilder<'a> {
    /// Create a TransactionBuilder based on the transmitted data
    pub fn from_run_cmd(cmd: Run, ctx: &'a Context) -> Result<TransactionBuilder<'a>, Error> {
        let mut trbuild = Self::new(ctx);
        trbuild.script_file_name = cmd.file_name;
        trbuild
            .with_cmd_call(cmd.call)?
            .with_cmd_script_name(cmd.script_name)
            .with_cmd_args(cmd.args)
            .with_cmd_signers(cmd.signers)?;

        Ok(trbuild)
    }
}
