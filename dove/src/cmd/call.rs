use structopt::StructOpt;
use std::fmt::Debug;
use std::fs;
use std::path::Path;
use anyhow::Error;
use lang::bytecode::accessor::BytecodeRef;
use crate::cmd::deploy::run_dove_package_build;
use crate::cmd::Cmd;
use crate::context::Context;
use crate::call::cmd::CallDeclarationCmd;
use crate::call::fn_call::Config;
use crate::call::make_transaction;
use crate::call::model::{EnrichedTransaction, Transaction};

#[derive(StructOpt, Debug)]
#[structopt(setting(structopt::clap::AppSettings::ColoredHelp))]
#[structopt(usage = "dove tx [call] [OPTIONS]\n
    Examples:
    $ dove tx 'script_name<0x01::Dfinance::USD>([10,10], true, 68656c6c6f776f726c64, 100)'
    $ dove tx 'script_name()' --parameters [10,10] true 68656c6c6f776f726c64 100 0x1 --type 0x01::Dfinance::USD
    $ dove tx '0x1::Module::script_name<0x01::Dfinance::USD>()'
")]
pub struct ExecuteTransaction {
    #[structopt(flatten)]
    call: CallDeclarationCmd,

    #[structopt(help = "Output file name.", long = "output", short = "o")]
    output: Option<String>,
}

impl Cmd for ExecuteTransaction {
    fn apply(&mut self, ctx: &mut Context) -> anyhow::Result<()>
    where
        Self: Sized,
    {
        run_dove_package_build(ctx)?;
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
