use std::fmt::Debug;
use std::fs;
use anyhow::Error;
use structopt::StructOpt;
use dove_lib::tx::model::{Transaction, EnrichedTransaction};
use dove_lib::tx::fn_call::Config;
use crate::cmd::Cmd;
use crate::context::Context;
use crate::stdoutln;
use crate::tx::cmd::CallDeclarationCmd;
use crate::tx::make_transaction;

/// Create transaction.
#[derive(StructOpt, Debug)]
#[structopt(setting(structopt::clap::AppSettings::ColoredHelp))]
#[structopt(usage = "dove tx [call] [OPTIONS]\n
    Examples:
    $ dove tx 'script_name<0x01::Dfinance::USD>([10,10], true, 68656c6c6f776f726c64, 100)' --f file_name
    $ dove tx script_name --file file_name -a [10,10] true 68656c6c6f776f726c64 100 0x1 -type 0x01::Dfinance::USD
    $ dove tx '0x1::Module::script_name<0x01::Dfinance::USD>()'
")]
pub struct CreateTransactionCmd {
    #[structopt(flatten)]
    call: CallDeclarationCmd,
    #[structopt(help = "Output file name.", long = "output", short = "o")]
    output: Option<String>,
    #[structopt(long, hidden = true)]
    color: Option<String>,
}

impl Cmd for CreateTransactionCmd {
    fn apply(mut self, ctx: Context) -> Result<(), Error> {
        let tx = make_transaction(&ctx, self.call, Config::for_tx())?;
        let output_filename = self.output.take();
        match tx {
            EnrichedTransaction::Local { .. } => unreachable!(),
            EnrichedTransaction::Global { tx, name } => {
                store_transaction(&ctx, &output_filename.unwrap_or(name), tx)
            }
        }
    }
}

fn store_transaction(ctx: &Context, name: &str, tx: Transaction) -> Result<(), Error> {
    let tx_dir = ctx.path_for(&ctx.manifest.layout.transactions_output);
    if !tx_dir.exists() {
        fs::create_dir_all(&tx_dir)?;
    }

    let mut tx_file = tx_dir.join(name);
    if !name.to_lowercase().ends_with(".mvt") {
        tx_file.set_extension("mvt");
    }

    if tx_file.exists() {
        fs::remove_file(&tx_file)?;
    }
    stdoutln!("Store transaction:{:?}", tx_file);
    Ok(fs::write(&tx_file, bcs::to_bytes(&tx)?)?)
}
