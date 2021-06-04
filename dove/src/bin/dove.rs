use std::process::exit;
use std::io::stdout;
use anyhow::Error;
use dove::cli::execute;
use dove::stdout::set_buffer;
use dove::stdout::colorize::error;

fn main() {
    let args = std::env::args_os();
    let cwd = std::env::current_dir().expect("Current directory exists and accessible");
    set_buffer(stdout()).expect("Failed to set stdout");
    let res = execute(args, cwd);
    handle_error(res)
}

fn handle_error<T>(res: Result<T, Error>) -> T {
    match res {
        Ok(t) => t,
        Err(err) => {
            eprintln!("\n{}: {:?}.", error("ERROR"), err);
            exit(1);
        }
    }
}
