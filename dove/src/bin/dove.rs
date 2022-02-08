use std::process::exit;
use anyhow::Result;

use dove::cli::execute;

fn main() {
    let res = try_main();
    match res {
        Ok(t) => t,
        Err(err) => {
            eprintln!("\nERROR: {:?}", err);
            exit(1);
        }
    }
}

fn try_main() -> Result<()> {
    let args = std::env::args().collect();
    let cwd = std::env::current_dir()?;

    execute(args, cwd)
}
