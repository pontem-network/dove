extern crate structopt;

use std::env;

use anyhow::Result;
use structopt::StructOpt;

use lang::compiler::ConstPool;

use crate::cmd::build::Build;
use crate::cmd::clean::Clean;
use crate::cmd::ct::CreateTransactionCmd;
use crate::cmd::fetch::Fetch;
use crate::cmd::init::Init;
use crate::cmd::metadata::Metadata;
use crate::cmd::new::New;
use crate::cmd::run::Run;
use crate::cmd::test::Test;
use crate::cmd::Cmd;
use std::ffi::OsString;

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
    #[structopt(about = "Run move tests")]
    Run {
        #[structopt(flatten)]
        cmd: Run,
    },
    #[structopt(about = "Create transaction")]
    Ct {
        #[structopt(flatten)]
        cmd: CreateTransactionCmd,
    },
}

/// Public interface for the CLI (useful for testing).
pub fn execute<Args>(args: Args) -> Result<()>
where
    Args: IntoIterator,
    Args::Item: Into<OsString> + Clone,
{
    let matches = Opt::from_iter(args);

    let _pool = ConstPool::new();
    match matches {
        Opt::Clean { cmd } => cmd.execute(),
        Opt::New { cmd } => cmd.execute(),
        Opt::Init { cmd } => cmd.execute(),
        Opt::Metadata { cmd } => cmd.execute(),
        Opt::Fetch { cmd } => cmd.execute(),
        Opt::Build { cmd } => cmd.execute(),
        Opt::Test { cmd } => cmd.execute(),
        Opt::Run { cmd } => cmd.execute(),
        Opt::Ct { cmd } => cmd.execute(),
    }
}
