use anyhow::Error;
use structopt::StructOpt;

use lang::tx::fn_call::Config;
use crate::cmd::Cmd;
use crate::context::Context;
use crate::tx::cmd::CallDeclarationCmd;
use crate::tx::make_transaction;
use crate::executor::execute_transaction;
use lang::tx::model::EnrichedTransaction;

/// Run move script
#[derive(StructOpt, Debug)]
#[structopt(setting(structopt::clap::AppSettings::ColoredHelp))]
#[structopt(usage = "dove run [call] [OPTIONS]\n
    Examples:
    $ dove run 'script_name([10,10], true, 68656c6c6f776f726c64, 100, 0x1)' --f file_name
    $ dove run script_name --file file_name -a [10,10] true 68656c6c6f776f726c64 100 0x1
    $ dove run 'script_name()'
    $ dove run 'Module::function()'
    $ dove run '0x1::Module::function()'
")]
pub struct Run {
    #[structopt(flatten)]
    call: CallDeclarationCmd,
    #[structopt(long, hidden = true)]
    color: Option<String>,
    /// If set, the effects of executing `script_file` (i.e., published, updated, and
    /// deleted resources) will NOT be committed to disk.
    #[structopt(long = "dry-run", short = "d")]
    dry_run: bool,
    /// Print additional diagnostics
    #[structopt(short = "v", global = true)]
    verbose: bool,
}

impl Cmd for Run {
    fn apply(self, ctx: Context) -> Result<(), Error> {
        let tx = make_transaction(&ctx, self.call, Config::for_run())?;
        match tx {
            EnrichedTransaction::Local { tx, signers, deps } => {
                execute_transaction(&ctx, signers, tx, deps, self.verbose, self.dry_run)
            }
            EnrichedTransaction::Global { .. } => unreachable!(),
        }
    }
}
