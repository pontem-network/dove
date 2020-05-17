use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};
use structopt::StructOpt;

use dialects::{DFinanceDialect, Dialect};
use lang::changes::changes_into_writeset;
use lang::types::VMStatus;
use shared::changes::ResourceChange;
use utils::{io, leaked_fpath, FilePath, FilesSourceText};

#[derive(Debug, serde::Serialize)]
pub struct ExecStatus {
    pub vm_status: VMStatus,
    pub vm_status_description: String,
}

impl From<VMStatus> for ExecStatus {
    fn from(vm_status: VMStatus) -> Self {
        ExecStatus {
            vm_status_description: format!("{:?}", vm_status.major_status),
            vm_status,
        }
    }
}

#[derive(StructOpt)]
struct Options {
    // required positional
    #[structopt()]
    script: PathBuf,

    #[structopt(short, long)]
    sender: String,

    #[structopt(long)]
    modules: Option<Vec<PathBuf>>,

    #[structopt(long)]
    genesis: Option<PathBuf>,
}

fn parse_genesis_json(fpath: Option<PathBuf>) -> Result<Vec<ResourceChange>> {
    let genesis = match fpath {
        None => vec![],
        Some(fpath) => {
            let text = fs::read_to_string(fpath.clone())?;
            let val = serde_json::from_str(&text)?;
            serde_json::from_value::<Vec<ResourceChange>>(val)
                .with_context(|| format!("{:?} contains invalid genesis data", fpath))?
        }
    };
    Ok(genesis)
}

fn get_file_sources_mapping(
    script: (FilePath, String),
    deps: Vec<(FilePath, String)>,
) -> FilesSourceText {
    let mut mapping = FilesSourceText::with_capacity(deps.len() + 1);
    for (fpath, text) in vec![script].into_iter().chain(deps.into_iter()) {
        mapping.insert(fpath, text);
    }
    mapping
}

fn main() -> Result<()> {
    let options: Options = Options::from_args();

    let script_text = fs::read_to_string(&options.script)?;
    let deps = io::load_move_module_files(options.modules.unwrap_or_default())?;

    let genesis_changes = parse_genesis_json(options.genesis)?;
    let sender = DFinanceDialect::validate_sender_address(options.sender)?;

    let script_fpath = leaked_fpath(options.script);
    let genesis_write_set = changes_into_writeset(genesis_changes)?;
    let exec_res = lang::executor::compile_and_run(
        (script_fpath, script_text.clone()),
        &deps,
        sender,
        genesis_write_set,
    );
    let vm_result = match exec_res {
        Ok(vm_res) => vm_res,
        Err(errors) => {
            let files_mapping = get_file_sources_mapping((script_fpath, script_text), deps);
            lang::report_errors(files_mapping, errors);
        }
    };
    let out = match vm_result {
        Ok(changes) => serde_json::to_string_pretty(&changes).unwrap(),
        Err(vm_status) => {
            let exec_status = ExecStatus::from(vm_status);
            serde_json::to_string_pretty(&exec_status).unwrap()
        }
    };
    println!("{}", out);
    Ok(())
}
