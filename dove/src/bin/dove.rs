use std::process::exit;
use anyhow::Error;

use dove::cli::execute;

fn main() {
    let args = std::env::args_os()
        .map(|arg| arg.into_string().unwrap())
        .collect::<Vec<_>>();
    let cwd = std::env::current_dir().expect("Current directory exists and accessible");

    let res = execute(args, cwd);
    handle_error(res)
}

fn handle_error<T>(res: Result<T, Error>) -> T {
    match res {
        Ok(t) => t,
        Err(err) => {
            eprintln!("\nERROR: {:?}", err);
            exit(1);
        }
    }
}
