use std::fmt::Debug;
use std::fs;
use std::path::{Path, PathBuf};

use clap::Parser;
use anyhow::{Error, Result};

use lang::bytecode::accessor::BytecodeRef;
use crate::cmd::deploy::run_dove_package_build;
use crate::context::Context;
use crate::call::cmd::CallDeclarationCmd;
use crate::call::fn_call::Config;
use crate::call::make_transaction;
use crate::call::model::{EnrichedTransaction, Transaction};
use crate::publish::{NodeAccessParams, Publish};

#[derive(Parser, Debug)]
#[clap(about = "dove call [call] [OPTIONS]\n
    Examples:
    $ dove call 'script_name<0x01::Dfinance::USD>([10,10], true, ADDRESS_ALIAS, SS58_ADDRESS, 100, 0x1)'
    $ dove call 'script_name()' --args [10,10] true ADDRESS_ALIAS SS58_ADDRESS 100 0x1 --type 0x01::Dfinance::USD
    $ dove call '0x1::Module::script_name<0x01::Dfinance::USD>()'
    $ dove call 'script_name()' --account WALLET_KEY --gas 300
    $ dove call 'script_name()' --secret --url https://127.0.0.1:9933 --gas 400
    $ dove call 'script_name()' --account //Alice --gas 300
")]
pub struct ExecuteTransaction {
    #[clap(flatten)]
    call: CallDeclarationCmd,

    #[clap(flatten)]
    request: NodeAccessParams,
}

impl ExecuteTransaction {
    pub fn apply(&mut self, ctx: &mut Context) -> Result<()> {
        run_dove_package_build(ctx)?;
        let tx = make_transaction(ctx, self.call.take(), Config::for_tx())?;
        let path_transaction = match tx {
            EnrichedTransaction::Local { .. } => unreachable!(),
            EnrichedTransaction::Global { bi, tx, name } => {
                store_transaction(ctx, &name, bi.bytecode_ref(), tx)?
            }
        };

        if !self.request.need_to_publish() {
            return Ok(());
        }

        Publish::try_from((&self.request, path_transaction))?
            .apply()
            .map(|hash| {
                println!("Hash: {}", hash);
            })
    }
}

fn store_transaction(
    ctx: &Context,
    name: &str,
    rf: &BytecodeRef,
    tx: Transaction,
) -> Result<PathBuf, Error> {
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
    fs::write(&tx_file, bcs::to_bytes(&tx)?)?;

    Ok(tx_file)
}

fn get_package_from_path<A: AsRef<Path>>(path: A) -> Option<String> {
    let path: &Path = path.as_ref();
    path.parent()
        .and_then(|p| p.parent())
        .and_then(|p| p.file_name())
        .map(|name| name.to_string_lossy().to_string())
}
