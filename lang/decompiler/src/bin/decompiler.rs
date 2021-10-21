use std::fs::{canonicalize, File, read};
use std::io::Write;
use std::path::PathBuf;
use structopt::StructOpt;

use anyhow::Error;

#[derive(StructOpt, Debug)]
#[structopt(name = "Move decompiler", version = decompiler::VERSION)]
struct Opt {
    #[structopt(about = "Path to input file", long = "input", short = "i")]
    /// Path to compiled Move binary
    input: PathBuf,
    #[structopt(
        about = "Dialect name",
        long = "dialect",
        short = "d",
        default_value = "pontem"
    )]
    /// Dialect name.
    dialect: String,
    #[structopt(about = "Path to output file", long = "output", short = "o")]
    /// Optional path to output file.
    /// Prints results to stdout by default.
    output: Option<PathBuf>,
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
    }
}

fn run() -> Result<(), Error> {
    let opts = Opt::from_args();

    let input = canonicalize(opts.input)?;
    let mut bytes = read(input)?;

    match opts.dialect.as_ref() {
        "dfinance" => {
            compat::adapt_to_basis(&mut bytes, compat::AddressType::Dfninance)?;
        }
        "diem" => {
            compat::adapt_to_basis(&mut bytes, compat::AddressType::Diem)?;
        }
        _ => {
            // no-op
        }
    }

    let cfg = decompiler::Config {
        light_version: false,
    };

    let out = decompiler::decompile_str(&bytes, cfg)?;

    if let Some(output) = opts.output {
        File::create(output)?.write_all(out.as_bytes())?;
    } else {
        println!("{}", out);
    }

    Ok(())
}
