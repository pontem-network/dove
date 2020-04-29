use std::collections::HashMap;
use std::path::PathBuf;

use libra_types::access_path::AccessPath;
use libra_types::account_address::AccountAddress;
use move_lang::shared::Address;
use structopt::StructOpt;

use analysis::utils::io;
use analysis::utils::io::leaked_fpath;

mod executor;

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
}

fn main() {
    let options: Options = Options::from_args();

    let fname = leaked_fpath(options.script.to_str().unwrap());
    let script_text = std::fs::read_to_string(fname).unwrap();

    let mut deps = vec![];
    for module_folder in options.modules.unwrap_or_else(|| vec![]) {
        for dep in io::get_module_files(module_folder.as_path()) {
            deps.push(dep);
        }
    }
    let (compiled_script, compiled_modules) = executor::compile_script(
        fname,
        &script_text,
        deps,
        Address::new(options.sender.into()),
    )
    .unwrap();
    let mut network_state = HashMap::new();
    for module in compiled_modules {
        let mod_access_path = AccessPath::code_access_path(&module.self_id());
        let mut serialized = vec![];
        module.serialize(&mut serialized).unwrap();
        network_state.insert(mod_access_path, serialized);
    }
    let serialized_script = executor::serialize_script(compiled_script);
    let res = executor::execute_script(options.sender, network_state, serialized_script, vec![]);
    println!("{:?}", res);
}
