use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};

use clap::{App, Arg};
use dialects::shared::errors::ExecCompilerError;

use move_executor::compile_and_run_scripts_in_file;
use move_executor::explain::{PipelineExecutionResult, StepExecutionResult};
use utils::{io, leaked_fpath};
use lang::compiler::print_compiler_errors_and_exit;
use move_executor::exec_utils::get_files_for_error_reporting;

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
                .number_of_values(1)
                .help("Path to module file / modules folder to use as dependency. \nCould be used more than once: '-m ./stdlib -m ./modules'"),
        )
        .arg(Arg::from_usage("--show-changes").help("Show what changes has been made to the network after script is executed"))
        .arg(Arg::from_usage("--show-events").help("Show which events was emitted"))
        .arg(
            Arg::from_usage("--args [SCRIPT_ARGS]")
                .help(r#"Number of script main() function arguments in quotes, e.g. "10 20 30""#),
        )
        .get_matches();

    let script_fpath = leaked_fpath(cli_arguments.value_of("SCRIPT").unwrap());
    let script_source_text = fs::read_to_string(script_fpath)
        .with_context(|| format!("Cannot open {:?}", script_fpath))?;

    let modules_fpaths = cli_arguments
        .values_of("modules")
        .unwrap_or_default()
        .map(|path| path.into())
        .collect::<Vec<PathBuf>>();
    let deps = io::load_move_files(modules_fpaths)?;

    let show_network_effects = cli_arguments.is_present("show-changes");
    let show_events = cli_arguments.is_present("show-events");

    let dialect = cli_arguments.value_of("dialect").unwrap();
    let sender = cli_arguments.value_of("sender").unwrap();
    let args: Vec<String> = cli_arguments
        .value_of("args")
        .unwrap_or_default()
        .split_ascii_whitespace()
        .map(String::from)
        .collect();

    let res = compile_and_run_scripts_in_file(
        (script_fpath, script_source_text.clone()),
        &deps,
        dialect,
        sender,
        args,
    );
    match res {
        Ok(exec_result) => {
            let PipelineExecutionResult {
                gas_spent,
                step_results,
            } = exec_result;
            println!("Gas used: {}", gas_spent);

            for (name, step_result) in step_results {
                println!("{}: ", name);
                match step_result {
                    StepExecutionResult::Error(error) => {
                        print!("{}", textwrap::indent(&error, "    "));
                    }
                    StepExecutionResult::Success(effects) => {
                        if show_events {
                            for event in effects.events() {
                                print!("{}", textwrap::indent(event, "    "));
                            }
                        }
                        if show_network_effects {
                            for change in effects.resources() {
                                print!("{}", textwrap::indent(&format!("{}", change), "    "));
                            }
                        }
                    }
                }
            }
            Ok(())
        }
        Err(error) => {
            let error = match error.downcast::<ExecCompilerError>() {
                Ok(compiler_error) => {
                    let files_mapping =
                        get_files_for_error_reporting((script_fpath, script_source_text), deps);
                    let transformed_errors = compiler_error.transform_with_source_map();
                    print_compiler_errors_and_exit(files_mapping, transformed_errors);
                }
                Err(error) => error,
            };
            Err(error)
        }
    }
}
