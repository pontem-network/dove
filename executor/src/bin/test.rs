use anyhow::Result;
use clap::{App, Arg};
use move_executor::compile_and_run_file_as_test;
use utils::io;
use std::path::PathBuf;
use dialects::shared::errors::ExecCompilerError;
use lang::compiler::print_compiler_errors_and_exit;
use move_executor::exec_utils::get_files_for_error_reporting;
use move_executor::explain::StepExecutionResult;

pub fn print_test_status(test_name: &str, status: &str) {
    println!("{} ....... {}", test_name, status);
}

pub fn main() -> Result<()> {
    let cli_arguments = App::new("Test runner")
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
        .get_matches();

    let verbose_output = cli_arguments.is_present("verbose");
    let test_name_pattern = cli_arguments.value_of("name-pattern");

    let modules_fpaths = cli_arguments
        .values_of("modules")
        .unwrap_or_default()
        .map(|path| path.into())
        .collect::<Vec<PathBuf>>();
    let deps = io::load_move_files(modules_fpaths)?;
    if verbose_output {
        println!(
            "Found deps: {:#?}",
            deps.iter().map(|(n, _)| n).collect::<Vec<_>>()
        );
    }

    let sender = cli_arguments.value_of("sender").unwrap();

    let test_directories = match cli_arguments.value_of("TEST DIRECTORY") {
        Some(dir) => vec![PathBuf::from(dir)],
        None => vec![],
    };
    let test_files = io::load_move_files(test_directories)?;
    if verbose_output {
        println!(
            "Found tests: {:#?}",
            test_files.iter().map(|(n, _)| n).collect::<Vec<_>>()
        );
    }

    let mut has_failures = false;
    for test_file in test_files {
        let test_file_name = PathBuf::from(test_file.0)
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

        let deps = deps.clone();
        let exec_result =
            compile_and_run_file_as_test(test_file.clone(), &deps, "dfinance", sender).map_err(
                |err| {
                    let error = match err.downcast::<ExecCompilerError>() {
                        Ok(compiler_error) => {
                            let files_mapping = get_files_for_error_reporting(test_file, deps);
                            let transformed_errors = compiler_error.transform_with_source_map();
                            print_compiler_errors_and_exit(files_mapping, transformed_errors);
                        }
                        Err(error) => error,
                    };
                    error
                },
            )?;

        match exec_result.last() {
            None => {
                print_test_status(test_name, "NO_SCRIPT");
            }
            Some(step_result) => match step_result {
                StepExecutionResult::Error(error) => {
                    print_test_status(test_name, "ERROR");

                    has_failures = true;
                    print!("{}", textwrap::indent(&error, "    "));
                    println!();
                }
                StepExecutionResult::Success(_) => {
                    print_test_status(test_name, "ok");
                }
            },
        }
    }

    if has_failures {
        std::process::exit(1);
    }
    Ok(())
}
