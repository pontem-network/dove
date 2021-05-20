use std::process::exit;

use anyhow::Error;

use dove::cli::execute;

fn main() {
    let args = std::env::args_os();

    let res = execute(args);
    handle_error(res)
}

fn handle_error<T>(res: Result<T, Error>) -> T {
    match res {
        Ok(t) => t,
        Err(err) => {
            eprintln!("error: {:?}.", err);
            exit(1);
        }
    }
}
