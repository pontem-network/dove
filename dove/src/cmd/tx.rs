use structopt::StructOpt;
use std::fmt::Debug;
use std::fs;
use std::path::Path;
use anyhow::Error;
use lang::bytecode::accessor::BytecodeRef;
use crate::cmd::build::run_internal_build;
use crate::cmd::Cmd;
use crate::context::Context;
use crate::tx::cmd::CallDeclarationCmd;
use crate::tx::fn_call::Config;
use crate::tx::make_transaction;
use crate::tx::model::{EnrichedTransaction, Transaction};

/// Create transaction.
#[derive(StructOpt, Debug)]
#[structopt(setting(structopt::clap::AppSettings::ColoredHelp))]
#[structopt(usage = "dove tx [call] [OPTIONS]\n
    Examples:
    $ dove tx 'script_name<0x01::Dfinance::USD>([10,10], true, 68656c6c6f776f726c64, 100)'
    $ dove tx 'script_name()' --parameters [10,10] true 68656c6c6f776f726c64 100 0x1 --type 0x01::Dfinance::USD
    $ dove tx '0x1::Module::script_name<0x01::Dfinance::USD>()'
")]
pub struct CreateTransactionCmd {
    #[structopt(flatten)]
    call: CallDeclarationCmd,

    #[structopt(help = "Output file name.", long = "output", short = "o")]
    output: Option<String>,
}

impl Cmd for CreateTransactionCmd {
    fn apply(&mut self, ctx: &mut Context) -> anyhow::Result<()>
    where
        Self: Sized,
    {
        run_internal_build(ctx)?;
        let tx = make_transaction(ctx, self.call.take(), Config::for_tx())?;
        let output_filename = self.output.as_ref().take();
        match tx {
            EnrichedTransaction::Local { .. } => unreachable!(),
            EnrichedTransaction::Global { bi, tx, name } => {
                store_transaction(ctx, output_filename.unwrap_or(&name), bi.bytecode_ref(), tx)
            }
        }
    }
}

fn store_transaction(
    ctx: &Context,
    name: &str,
    rf: &BytecodeRef,
    tx: Transaction,
) -> Result<(), Error> {
    let tx_dir = ctx.tx_output_path(get_package_from_path(&rf.0));
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
    println!("Store transaction: {:?}", tx_file);
    Ok(fs::write(&tx_file, bcs::to_bytes(&tx)?)?)
}

fn get_package_from_path<A: AsRef<Path>>(path: A) -> Option<String> {
    let path: &Path = path.as_ref();
    path.parent()
        .and_then(|p| p.parent())
        .and_then(|p| p.file_name())
        .map(|name| name.to_string_lossy().to_string())
}
