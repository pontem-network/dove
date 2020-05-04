use std::fs;
use std::path::PathBuf;

use anyhow::Result;
use libra_types::account_address::AccountAddress;
use structopt::StructOpt;

use analysis::db::FilePath;
use analysis::utils::io;
use analysis::utils::io::leaked_fpath;

use crate::serialization::ResourceChange;

mod executor;
mod serialization;

fn parse_address(s: &str) -> AccountAddress {
    AccountAddress::from_hex_literal(s).unwrap()
}

fn load_module_files(module_paths: Vec<PathBuf>) -> Result<Vec<(FilePath, String)>> {
    let mut deps = vec![];
    for module_path in module_paths {
        anyhow::ensure!(
            module_path.exists(),
            "Cannot open {:?}: No such file or directory",
            module_path
        );
        if module_path.is_file() {
            let fpath = leaked_fpath(module_path.to_str().unwrap());
            let text = fs::read_to_string(fpath).unwrap();
            deps.push((fpath, text));
        } else {
            for dep in io::get_module_files(module_path.as_path()) {
                deps.push(dep);
            }
        }
    }
    Ok(deps)
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

fn main() {
    let options: Options = Options::from_args();

    let fname = leaked_fpath(options.script.to_str().unwrap());
    let script_text = fs::read_to_string(fname).unwrap();
    let deps = match load_module_files(options.modules.unwrap_or_else(|| vec![])) {
        Ok(deps) => deps,
        Err(err) => {
            println!("{}", err);
            return;
        }
    };
    let genesis = match options.genesis {
        None => None,
        Some(fpath) => {
            let text = fs::read_to_string(fpath.clone()).unwrap();
            let val = serde_json::from_str(&text).unwrap();
            match serde_json::from_value::<Vec<ResourceChange>>(val) {
                Ok(g) => Some(g),
                Err(err) => {
                    println!("{:?} contains invalid genesis data: {:?}", fpath, err);
                    return;
                }
            }
        }
    };

    let vm_result = executor::compile_and_run((fname, script_text), deps, options.sender, genesis);
    let out = match vm_result {
        Ok(changes) => serde_json::to_string_pretty(&changes).unwrap(),
        Err(vm_status) => {
            let vm_status = serialization::VMStatusVerbose::from(vm_status);
            serde_json::to_string_pretty(&vm_status).unwrap()
        }
    };
    println!("{}", out);
}
