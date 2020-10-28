use std::path::PathBuf;

use anyhow::{Context, Result};

use clap::{App, Arg};

use move_executor::execute_script;
use move_executor::explain::{
    PipelineExecutionResult, StepExecutionResult, ResourceChange, AddressResourceChanges,
};
use move_lang::name_pool::ConstPool;
use lang::compiler::file::MoveFile;
use lang::compiler::file;
use lang::compiler::error::CompilerError;
use move_lang::errors::report_errors;

fn formatted_resource_change(change: &ResourceChange) -> String {
    let ResourceChange(ty, val) = change;
    match val {
        Some(val) => format!("{} =\n    {}", ty, val),
        None => ty.to_string(),
    }
}

fn cli() -> App<'static, 'static> {
    App::new("Move Executor")
        .version(git_hash::crate_version_with_git_hash_short!())
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
        .arg(
            Arg::from_usage("--args [SCRIPT_ARGS]")
                .help(r#"Number of script main() function arguments in quotes, e.g. "10 20 30""#),
        )
}

fn main() -> Result<()> {
    let cli_arguments = cli().get_matches();
    let _pool = ConstPool::new();

    let script =
        MoveFile::load(cli_arguments.value_of("SCRIPT").unwrap()).with_context(|| {
            format!(
                "Cannot open {:?}",
                cli_arguments.value_of("SCRIPT").unwrap()
            )
        })?;

    let modules_fpaths = cli_arguments
        .values_of("modules")
        .unwrap_or_default()
        .map(|path| path.into())
        .collect::<Vec<PathBuf>>();

    let deps = file::load_move_files(&modules_fpaths)?;

    let dialect = cli_arguments.value_of("dialect").unwrap();
    let sender = cli_arguments.value_of("sender").unwrap();
    let args: Vec<String> = cli_arguments
        .value_of("args")
        .unwrap_or_default()
        .split_ascii_whitespace()
        .map(String::from)
        .collect();

    let res = execute_script(script, deps, dialect, sender, args);
    match res {
        Ok(exec_result) => {
            let PipelineExecutionResult { step_results } = exec_result;
            for (i, (name, gas, writeset_size, step_result)) in
                step_results.into_iter().enumerate()
            {
                if i > 0 {
                    println!();
                }
                println!(
                    "{}(gas: {}, writeset bytes: {}): ",
                    name, gas, writeset_size
                );
                let step_indent = "    ";
                let content_indent = "        ";
                match step_result {
                    StepExecutionResult::Error(error)
                    | StepExecutionResult::ExpectedError(error) => {
                        print!("{}", textwrap::indent(&error, step_indent));
                    }
                    StepExecutionResult::Success(effects) => {
                        for changes in effects.resources() {
                            let AddressResourceChanges { address, changes } = changes;
                            print!("{}", textwrap::indent(address, step_indent));
                            for (operation, change) in changes {
                                print!(
                                    "{}",
                                    textwrap::indent(
                                        &format!(
                                            "{} {}",
                                            operation,
                                            formatted_resource_change(change)
                                        ),
                                        content_indent
                                    )
                                );
                            }
                        }
                        if !effects.events().is_empty() {
                            print!("{}", textwrap::indent("Events:", content_indent));
                            for event_change in effects.events() {
                                print!(
                                    "{}",
                                    textwrap::indent(
                                        &formatted_resource_change(event_change),
                                        &(content_indent.to_owned() + "    ")
                                    )
                                );
                            }
                        }
                    }
                }
            }
            Ok(())
        }
        Err(err) => match err.downcast::<CompilerError>() {
            Ok(compiler_error) => report_errors(compiler_error.source_map, compiler_error.errors),
            Err(error) => Err(error),
        },
    }
}
