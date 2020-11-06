use anyhow::Result;
use clap::{App, Arg};
use move_executor::execute_script;
use std::path::PathBuf;
use move_executor::explain::StepExecutionResult;
use lang::compiler::{ConstPool, file};
use lang::compiler::error::CompilerError;
use move_lang::errors::report_errors;
use move_executor::format::format_step_result;

fn cli() -> App<'static, 'static> {
    App::new("Test runner")
        .version("0.1.0")
        .arg(Arg::from_usage("--verbose"))
        .arg(Arg::from_usage("-k --name-pattern [NAME_PATTERN]").help("Specify test name to run (or substring)"))
        .arg(
            Arg::from_usage("-s --sender [SENDER_ADDRESS]")
                .required(true)
                .help("Address of the current user"),
        )
        .arg(
            Arg::from_usage("-m --modules [MODULE_PATHS]")
                .multiple(true)
                .number_of_values(1)
                .help(
                    "Path to module file / modules folder to use as dependency. \nCould be used more than once: '-m ./stdlib -m ./modules'",
                ),
        )
        .arg(
            Arg::with_name("TEST DIRECTORY")
                .required(true)
                .help("Where to find tests"),
        )
}

pub fn main() -> Result<()> {
    let cli_arguments = cli().get_matches();
    let _pool = ConstPool::new();

    let verbose_output = cli_arguments.is_present("verbose");
    let test_name_pattern = cli_arguments.value_of("name-pattern");

    let modules_fpaths = cli_arguments
        .values_of("modules")
        .unwrap_or_default()
        .map(|path| path.into())
        .collect::<Vec<PathBuf>>();
    let deps = file::load_move_files(&modules_fpaths)?;
    if verbose_output {
        println!(
            "Found deps: {:#?}",
            deps.iter().map(|n| n.name()).collect::<Vec<_>>()
        );
    }

    let sender = cli_arguments.value_of("sender").unwrap();

    let test_directories = match cli_arguments.value_of("TEST DIRECTORY") {
        Some(dir) => vec![PathBuf::from(dir)],
        None => vec![],
    };
    let test_files = file::load_move_files(&test_directories)?;
    if verbose_output {
        println!(
            "Found tests: {:#?}",
            test_files.iter().map(|n| n.name()).collect::<Vec<_>>()
        );
    }

    let mut has_failures = false;
    for test_file in test_files {
        let test_file_name = PathBuf::from(test_file.name())
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        let test_name = test_file_name.strip_suffix(".move").unwrap();

        if let Some(pattern) = test_name_pattern {
            if !test_name.contains(pattern) {
                continue;
            }
        }

        let exec_result = execute_script(test_file, deps.clone(), "dfinance", sender, vec![])
            .map_err(|err| match err.downcast::<CompilerError>() {
                Ok(compiler_error) => {
                    report_errors(compiler_error.source_map, compiler_error.errors)
                }
                Err(error) => error,
            })?;

        match exec_result.last() {
            None => {
                println!("{} ....... SCRIPT_NOT_FOUND", test_name);
            }
            Some(step_result) => match step_result {
                StepExecutionResult::Error(_) => {
                    has_failures = true;
                    println!("{} .......", test_name);

                    for step_result in exec_result.step_results {
                        print!(
                            "{}",
                            textwrap::indent(
                                &format_step_result(step_result, true, false),
                                "    "
                            )
                        );
                    }
                    println!();
                }
                StepExecutionResult::ExpectedError(_) | StepExecutionResult::Success(_) => {
                    println!("{} ....... ok", test_name);
                }
            },
        }
    }

    if has_failures {
        std::process::exit(1);
    }
    Ok(())
}
