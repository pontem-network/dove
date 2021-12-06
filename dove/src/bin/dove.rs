use std::process::exit;
use anyhow::Error;
use dove::cli::execute;
use dove::colorize::error;

fn main() {
    let args = std::env::args_os();
    let cwd = std::env::current_dir().expect("Current directory exists and accessible");
    let res = execute(args, cwd);
    handle_error(res)
}

fn handle_error<T>(res: Result<T, Error>) -> T {
    match res {
        Ok(t) => t,
        Err(err) => {
            eprintln!("\n{}: {:?}", error("ERROR"), err);
            exit(1);
        }
    }
}
