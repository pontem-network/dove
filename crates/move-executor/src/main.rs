use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};
use libra_types::account_address::AccountAddress;
use structopt::StructOpt;

use analysis::utils::io::leaked_fpath;

use crate::serialization::ResourceChange;

mod executor;
mod io;
mod serialization;

fn parse_address(s: &str) -> AccountAddress {
    AccountAddress::from_hex_literal(s).unwrap()
}

#[derive(StructOpt)]
struct Options {
    // required positional
    #[structopt()]
    script: PathBuf,

    #[structopt(short, long, parse(from_str = parse_address))]
    sender: AccountAddress,

    #[structopt(long)]
    modules: Option<Vec<PathBuf>>,

    #[structopt(long)]
    genesis: Option<PathBuf>,
}

fn parse_genesis_json(fpath: Option<PathBuf>) -> Result<Option<Vec<ResourceChange>>> {
    let genesis = match fpath {
        None => None,
        Some(fpath) => {
            let text = fs::read_to_string(fpath.clone())?;
            let val = serde_json::from_str(&text)?;
            let changes = serde_json::from_value::<Vec<ResourceChange>>(val)
                .with_context(|| format!("{:?} contains invalid genesis data", fpath))?;
            Some(changes)
        }
    };
    Ok(genesis)
}

fn main() -> Result<()> {
    let options: Options = Options::from_args();

    let script_text = fs::read_to_string(&options.script)?;
    let deps = io::load_module_files(options.modules.unwrap_or_default())?;

    let genesis = parse_genesis_json(options.genesis)?;

    let vm_result = executor::compile_and_run(
        (leaked_fpath(options.script), script_text),
        deps,
        options.sender,
        genesis,
    );
    let out = match vm_result {
        Ok(changes) => serde_json::to_string_pretty(&changes).unwrap(),
        Err(vm_status) => {
            let vm_status = serialization::VMStatusVerbose::from(vm_status);
            serde_json::to_string_pretty(&vm_status).unwrap()
        }
    };
    println!("{}", out);
    Ok(())
}
