use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};
use structopt::StructOpt;

use dialects::DialectName;

use shared::errors::ExecCompilerError;
use shared::results::{ExecutionError, ResourceChange};
use utils::{io, leaked_fpath, FilePath, FilesSourceText};

#[derive(StructOpt)]
struct Options {
    // required positional
    #[structopt()]
    script: PathBuf,

    #[structopt(short, long)]
    dialect: DialectName,

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
    let dialect = options.dialect.get_dialect();
    let sender_address = dialect.preprocess_and_validate_account_address(&options.sender)?;

    let script_fpath = leaked_fpath(options.script);
    let res = dialect.compile_and_run(
        (script_fpath, script_text.clone()),
        &deps,
        sender_address,
        genesis_changes,
    );
    match res {
        Ok(changes) => {
            let out =
                serde_json::to_string_pretty(&changes).expect("Should always be serializable");
            print!("{}", out);
            Ok(())
        }
        Err(error) => {
            let error = match error.downcast::<ExecCompilerError>() {
                Ok(compiler_error) => {
                    let files_mapping =
                        get_file_sources_mapping((script_fpath, script_text), deps);
                    dialect.print_compiler_errors_and_exit(
                        files_mapping,
                        compiler_error.apply_offsets(),
                    );
                }
                Err(error) => error,
            };
            let error = match error.downcast::<ExecutionError>() {
                Ok(exec_error) => {
                    let out = serde_json::to_string_pretty(&exec_error)
                        .expect("Should always be serializable");
                    print!("{}", out);
                    std::process::exit(1)
                }
                Err(error) => error,
            };
            Err(error)
        }
    }
}
