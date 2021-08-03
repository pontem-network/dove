extern crate structopt;

use std::env;

use anyhow::Result;
use structopt::StructOpt;

use crate::cmd::build::Build;
use crate::cmd::clean::Clean;
use crate::cmd::tx::CreateTransactionCmd;
use crate::cmd::fetch::Fetch;
use crate::cmd::init::Init;
use crate::cmd::metadata::Metadata;
use crate::cmd::new::New;
use crate::cmd::run::Run;
use crate::cmd::test::Test;
use crate::cmd::Cmd;
use std::ffi::OsString;
use std::path::PathBuf;
use crate::cmd::docgen::DocGen;

#[derive(StructOpt, Debug)]
#[structopt(name = "Dove", version = git_hash::crate_version_with_git_hash_short!())]
enum Opt {
    #[structopt(about = "Init directory as move project")]
    Init {
        #[structopt(flatten)]
        cmd: Init,
    },
    #[structopt(about = "Create a new move project")]
    New {
        #[structopt(flatten)]
        cmd: New,
    },
    #[structopt(about = "Print metadata")]
    Metadata {
        #[structopt(flatten)]
        cmd: Metadata,
    },
    #[structopt(about = "Fetch project dependencies")]
    Fetch {
        #[structopt(flatten)]
        cmd: Fetch,
    },
    #[structopt(about = "Build project")]
    Build {
        #[structopt(flatten)]
        cmd: Build,
    },
    #[structopt(about = "Remove the target directory")]
    Clean {
        #[structopt(flatten)]
        cmd: Clean,
    },
    #[structopt(about = "Run move tests")]
    Test {
        #[structopt(flatten)]
        cmd: Test,
    },
    #[structopt(about = "Run move script")]
    Run {
        #[structopt(flatten)]
        cmd: Run,
    },
    #[structopt(about = "Create transaction")]
    Tx {
        #[structopt(flatten)]
        cmd: CreateTransactionCmd,
    },
    #[cfg(feature = "prover")]
    #[structopt(about = "Run move prover")]
    Prove {
        #[structopt(flatten)]
        cmd: crate::cmd::prover::Prove,
    },
    #[structopt(about = "Generate documentation")]
    DocGen {
        #[structopt(flatten)]
        cmd: DocGen,
    },
}

/// Public interface for the CLI (useful for testing).
pub fn execute<Args>(args: Args, cwd: PathBuf) -> Result<()>
where
    Args: IntoIterator,
    Args::Item: Into<OsString> + Clone,
{
    let matches = Opt::from_iter(args);

    match matches {
        Opt::Clean { cmd } => cmd.execute(cwd),
        Opt::New { cmd } => cmd.execute(cwd),
        Opt::Init { cmd } => cmd.execute(cwd),
        Opt::Metadata { cmd } => cmd.execute(cwd),
        Opt::Fetch { cmd } => cmd.execute(cwd),
        Opt::Build { cmd } => cmd.execute(cwd),
        Opt::Test { cmd } => cmd.execute(cwd),
        Opt::Run { cmd } => cmd.execute(cwd),
        Opt::Tx { cmd } => cmd.execute(cwd),
        #[cfg(feature = "prover")]
        Opt::Prove { cmd } => cmd.execute(cwd),
        Opt::DocGen { cmd } => cmd.execute(cwd),
    }
}
