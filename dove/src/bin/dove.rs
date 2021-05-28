use std::process::exit;
use std::io::stdout;
use anyhow::Error;
use dove::cli::execute;

fn main() {
    let args = std::env::args_os();
    // TODO: use handle_error?
    let cwd = std::env::current_dir().expect("Current directory exists and accessible");

    let res = execute(args, cwd, stdout());
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
