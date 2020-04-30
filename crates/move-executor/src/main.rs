use std::path::PathBuf;

use libra_types::account_address::AccountAddress;

use structopt::StructOpt;

use analysis::db::FilePath;
use analysis::utils::io;
use analysis::utils::io::leaked_fpath;

mod executor;

fn parse_address(s: &str) -> AccountAddress {
    AccountAddress::from_hex_literal(s).unwrap()
}

fn load_module_files(module_folders: Vec<PathBuf>) -> Vec<(FilePath, String)> {
    let mut deps = vec![];
    for module_folder in module_folders {
        for dep in io::get_module_files(module_folder.as_path()) {
            deps.push(dep);
        }
    }
    deps
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
}

fn main() {
    let options: Options = Options::from_args();

    let fname = leaked_fpath(options.script.to_str().unwrap());
    let script_text = std::fs::read_to_string(fname).unwrap();
    let deps = load_module_files(options.modules.unwrap_or_else(|| vec![]));

    let vm_result = executor::compile_and_run((fname, script_text), deps, options.sender);
    println!("{:?}", vm_result);
}
