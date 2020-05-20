use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};

use dialects::DialectName;

use clap::{App, Arg};
use shared::errors::ExecCompilerError;
use shared::results::{ExecutionError, ResourceChange};
use std::str::FromStr;
use utils::{io, leaked_fpath, File, FilePath, FilesSourceText};

type ChainStateChanges = serde_json::Value;

pub fn compile_and_execute_script(
    script: File,
    deps: &[File],
    dialect: String,
    sender: String,
    genesis_json_contents: ChainStateChanges,
) -> Result<ChainStateChanges> {
    let dialect = DialectName::from_str(&dialect)?.get_dialect();
    let initial_chain_state =
        serde_json::from_value::<Vec<ResourceChange>>(genesis_json_contents)
            .with_context(|| "Genesis contains invalid data")?;

    let execution_changes = dialect.compile_and_run(script, deps, sender, initial_chain_state)?;
    Ok(serde_json::to_value(execution_changes).unwrap())
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
    let cli_arguments = App::new("Move Executor")
        .version("0.1.0")
        .arg(
            Arg::with_name("SCRIPT")
                .required(true)
                .help("Path to script to execute"),
        )
        .arg(
            Arg::from_usage("-d --dialect=[DIALECT]")
                .possible_values(&["libra", "dfinance"])
                .default_value("libra")
                .help("Move language dialect"),
        )
        .arg(
            Arg::from_usage("-s --sender [SENDER_ADDRESS]")
                .required(true)
                .help("Address of the current user"),
        )
        .arg(
            Arg::from_usage("-m --modules [MODULE_PATHS]")
                .multiple(true)
                .help("Number of paths to module files / folders to use as dependencies"),
        )
        .arg(
            Arg::from_usage("--genesis [GENESIS_JSON_FILE]")
                .help("Path to .json file to use as pre-loaded chain state"),
        )
        .get_matches();

    let script_fpath = leaked_fpath(cli_arguments.value_of("SCRIPT").unwrap());
    let script_source_text = fs::read_to_string(script_fpath)?;

    let modules_fpaths = cli_arguments
        .values_of("modules")
        .unwrap_or_default()
        .map(|path| path.into())
        .collect::<Vec<PathBuf>>();
    let deps = io::load_move_module_files(modules_fpaths)?;

    let genesis_json_contents = match cli_arguments.value_of("genesis") {
        Some(fpath) => {
            let contents = fs::read_to_string(fpath)?;
            serde_json::to_value(contents)?
        }
        None => serde_json::json!([]),
    };

    let dialect = cli_arguments.value_of("dialect").unwrap();
    let sender_address = cli_arguments.value_of("sender").unwrap().to_string();

    let res = compile_and_execute_script(
        (script_fpath, script_source_text.clone()),
        &deps,
        dialect.to_string(),
        sender_address,
        genesis_json_contents,
    );

    // let script_source_text = fs::read_to_string()
    // let options: Options = Options::from_args();

    // let script_text = fs::read_to_string(&options.script)?;
    // let deps = io::load_move_module_files(options.modules.unwrap_or_default())?;
    //
    // let genesis_changes = parse_genesis_json(options.genesis)?;
    // let dialect = options.dialect.get_dialect();
    // let sender_address = dialect.preprocess_and_validate_account_address(&options.sender)?;
    //
    // let script_fpath = leaked_fpath(options.script);
    // let res = dialect.compile_and_run(
    //     (script_fpath, script_text.clone()),
    //     &deps,
    //     sender_address,
    //     genesis_changes,
    // );
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
                        get_file_sources_mapping((script_fpath, script_source_text), deps);
                    let dialect = DialectName::from_str(&dialect).unwrap().get_dialect();
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
    // Ok(())
}
