use anyhow::Error;
use structopt::StructOpt;

use crate::cmd::Cmd;
use crate::context::Context;
use crate::executor::execute_transaction;
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
    /// If set, the effects of executing `script_file` (i.e., published, updated, and
    /// deleted resources) will NOT be committed to disk.
    #[structopt(long = "dry-run", short = "d")]
    dry_run: bool,
    /// Print additional diagnostics
    #[structopt(short = "v", global = true)]
    verbose: bool,
    #[structopt(help = "Owner's account address", long = "account-address")]
    account_address: Option<String>,
}

impl Cmd for Run {
    fn apply(self, mut ctx: Context) -> Result<(), Error> {
        ctx.set_account_address(self.account_address.as_ref())?;

        let verbose = self.verbose;
        let dry_run = self.dry_run;
        let tr_build = TransactionBuilder::from_run_cmd(self, &ctx)?;
        let (_, tx) = tr_build.to_transaction()?;
        let deps = tr_build.build_dependencies()?;
        execute_transaction(&ctx, tr_build.signers, tx, deps, verbose, dry_run)
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
