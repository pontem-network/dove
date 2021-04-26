use anyhow::{Result, Context};
use clap::{App, Arg};
use std::path::PathBuf;
use lang::compiler::{ConstPool, file};
use move_executor::executor::{Executor, render_test_result};
use lang::compiler::dialects::DialectName;
use std::str::FromStr;

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

    let dialect = DialectName::from_str("dfinance")?.get_dialect();
    let sender = dialect
        .parse_address(sender)
        .with_context(|| format!("Not a valid {:?} address: {:?}", dialect.name(), sender))?;

    let executor = Executor::new(dialect.as_ref(), sender, deps);

    let mut has_failures = false;
    for test_file in test_files {
        let test_name = Executor::script_name(&test_file).unwrap();

        if let Some(pattern) = test_name_pattern {
            if !test_name.contains(pattern) {
                continue;
            }
        }

        let is_test_fail =
            render_test_result(&test_name, executor.execute_script(test_file, None, vec![]))?;
        if is_test_fail {
            has_failures = true;
        }
    }

    if has_failures {
        std::process::exit(1);
    }
    Ok(())
}
