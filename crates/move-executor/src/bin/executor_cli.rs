use std::fs;
use std::path::PathBuf;

use anyhow::Result;

use dialects::DialectName;

use clap::{App, Arg};
use move_executor::compile_and_execute_script;
use shared::errors::ExecCompilerError;
use shared::results::ExecutionError;
use std::str::FromStr;
use utils::{io, leaked_fpath, FilePath, FilesSourceText};

fn get_files_for_error_reporting(
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
    let sender_address = cli_arguments.value_of("sender").unwrap();

    let res = compile_and_execute_script(
        (script_fpath, script_source_text.clone()),
        &deps,
        dialect,
        sender_address,
        genesis_json_contents,
        vec![],
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
                        get_files_for_error_reporting((script_fpath, script_source_text), deps);
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
            Err(error.context("Runtime error occurred"))
        }
    }
    // Ok(())
}
