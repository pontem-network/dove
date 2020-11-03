extern crate clap;

use std::fs::{File, canonicalize, read};
use std::path::PathBuf;
use std::io::Write;
use anyhow::Error;
use clap::Clap;

#[derive(Clap, Debug)]
#[clap(name = "Move decompiler", version = disassembler::VERSION)]
struct Opt {
    #[clap(about = "Path to input file", long, short)]
    /// Path to compiled Move binary
    input: PathBuf,

    #[clap(about = "Path to output file", long, short)]
    /// Optional path to output file.
    /// Prints results to stdout by default.
    output: Option<PathBuf>,

    #[clap(about = "Enables compatibility mode", long, short)]
    compat: bool,
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

    if opts.compat {
        compat::adapt(&mut bytes)?;
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
