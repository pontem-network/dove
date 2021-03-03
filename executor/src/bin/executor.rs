use std::path::PathBuf;

use anyhow::{Context, Result};

use clap::{App, Arg};

use diem::move_lang::name_pool::ConstPool;
use lang::compiler::file::MoveFile;
use lang::compiler::file;
use lang::compiler::dialects::DialectName;
use move_executor::executor::{Executor, render_execution_result};
use std::str::FromStr;

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

    let dialect = DialectName::from_str(dialect)?.get_dialect();
    let sender = dialect
        .normalize_account_address(sender)
        .with_context(|| format!("Not a valid {:?} address: {:?}", dialect.name(), sender))?;

    let executor = Executor::new(dialect.as_ref(), sender, deps);

    render_execution_result(executor.execute_script(script, args))
}
