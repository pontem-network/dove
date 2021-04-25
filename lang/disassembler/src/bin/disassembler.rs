extern crate clap;

use std::fs::{canonicalize, File, read};
use std::io::Write;
use std::path::PathBuf;

use anyhow::Error;
use clap::Clap;

#[derive(Clap, Debug)]
#[clap(name = "Move decompiler", version = disassembler::VERSION)]
struct Opt {
    #[clap(about = "Path to input file", long, short)]
    /// Path to compiled Move binary
    input: PathBuf,
    #[clap(about = "Dialect name", long, short, default_value = "pontem")]
    /// Dialect name.
    dialect: String,
    #[clap(about = "Path to output file", long, short)]
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
    let opts = Opt::parse();

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

    let cfg = disassembler::Config {
        light_version: false,
    };

    let out = disassembler::disasm_str(&bytes, cfg)?;

    if let Some(output) = opts.output {
        File::create(output)?.write_all(out.as_bytes())?;
    } else {
        println!("{}", out);
    }

    Ok(())
}
