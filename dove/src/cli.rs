extern crate structopt;

use std::env;

use anyhow::Result;
use structopt::StructOpt;
use crate::cmd::new::New;
use std::ffi::OsString;
use std::path::PathBuf;
use move_cli::{Command as DiemCommand, experimental, Move, package, run_cli, sandbox};
use crate::cmd::{Cmd};
use move_cli::DEFAULT_SOURCE_DIR;
use move_core_types::account_address::AccountAddress;
use move_core_types::errmap::ErrorMapping;
use crate::{DOVE_VERSION, DOVE_HASH, PONT_STDLIB_VERSION, DIEM_VERSION, DIEM_HASH};
use crate::cmd::build::Build;
use crate::cmd::clean::Clean;
use crate::cmd::docgen::DocGen;
use crate::cmd::export::Export;
use crate::cmd::init::Init;
use crate::cmd::run::Run;
use crate::cmd::test::Test;
use crate::cmd::tx::CreateTransactionCmd;

#[derive(StructOpt)]
#[structopt(
name = "Dove",
version = git_hash::crate_version_with_git_hash_short ! (),
long_version = create_long_version(),
)]
struct Opt {
    #[structopt(flatten)]
    pub move_args: Move,
    #[structopt(subcommand)]
    pub cmd: Command,
}

pub enum CommonCommand {
    Diem(DiemCommand),
    Dove(Box<dyn Cmd>),
}

#[derive(StructOpt)]
pub enum Command {
    #[structopt(name = "package")]
    Package {
        /// Path to package. If none is supplied the current directory will be used.
        #[structopt(long = "path", short = "p", global = true, parse(from_os_str))]
        path: Option<PathBuf>,

        #[structopt(flatten)]
        config: move_package::BuildConfig,

        #[structopt(subcommand)]
        cmd: package::cli::PackageCommand,
    },
    /// Compile and emit Move bytecode for the specified scripts and/or modules.
    #[structopt(name = "compile")]
    Compile {
        /// The source files to check.
        #[structopt(
        name = "PATH_TO_SOURCE_FILE",
        default_value = DEFAULT_SOURCE_DIR,
        )]
        source_files: Vec<String>,
        /// Do not emit source map information along with the compiled bytecode.
        #[structopt(long = "no-source-maps")]
        no_source_maps: bool,
        /// Type check and verify the specified scripts and/or modules. Does not emit bytecode.
        #[structopt(long = "check")]
        check: bool,
    },
    /// Execute a sandbox command.
    #[structopt(name = "sandbox")]
    Sandbox {
        #[structopt(subcommand)]
        cmd: sandbox::cli::SandboxCommand,
    },
    /// (Experimental) Run static analyses on Move source or bytecode.
    #[structopt(name = "experimental")]
    Experimental {
        #[structopt(subcommand)]
        cmd: experimental::cli::ExperimentalCommand,
    },

    #[structopt(about = "Init directory as move project")]
    Init {
        #[structopt(flatten)]
        cmd: Init,
    },
    /// Creates new project.
    #[structopt(about = "Create a new move project(Dove)")]
    New {
        /// Command.
        #[structopt(flatten)]
        cmd: New,
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
    #[structopt(about = "Export dove.toml => move .toml")]
    Export {
        #[structopt(flatten)]
        cmd: Export,
    },
}

impl Command {
    pub fn select_backend(self) -> CommonCommand {
        match self {
            Command::Package { path, config, cmd } => {
                CommonCommand::Diem(DiemCommand::Package { path, config, cmd })
            }
            Command::Compile { source_files, no_source_maps, check } => {
                CommonCommand::Diem(DiemCommand::Compile { source_files, no_source_maps, check })
            }
            Command::Sandbox { cmd } => {
                CommonCommand::Diem(DiemCommand::Sandbox { cmd })
            }
            Command::Experimental { cmd } => {
                CommonCommand::Diem(DiemCommand::Experimental { cmd })
            }
            Command::New { cmd } => CommonCommand::Dove(Box::new(cmd)),
            Command::Init { cmd } => CommonCommand::Dove(Box::new(cmd)),
            Command::Build { cmd } => CommonCommand::Dove(Box::new(cmd)),
            Command::Clean { cmd } => CommonCommand::Dove(Box::new(cmd)),
            Command::Test { cmd } => CommonCommand::Dove(Box::new(cmd)),
            Command::Run { cmd } => CommonCommand::Dove(Box::new(cmd)),
            Command::Tx { cmd } => CommonCommand::Dove(Box::new(cmd)),
            Command::Prove { cmd } => CommonCommand::Dove(Box::new(cmd)),
            Command::DocGen { cmd } => CommonCommand::Dove(Box::new(cmd)),
            Command::Export { cmd } => CommonCommand::Dove(Box::new(cmd)),
        }
    }
}

/// Public interface for the CLI (useful for testing).
pub fn execute<Args>(args: Args, cwd: PathBuf) -> Result<()>
    where
        Args: IntoIterator,
        Args::Item: Into<OsString> + Clone,
{
    let Opt { move_args, cmd } = Opt::from_iter(args);
    let commands = cmd.select_backend();
    match commands {
        CommonCommand::Diem(cmd) => {
            let error_descriptions: ErrorMapping = bcs::from_bytes(move_stdlib::error_descriptions())?;
            run_cli(
                move_stdlib::natives::all_natives(AccountAddress::from_hex_literal("0x1").unwrap()),
                &error_descriptions, &move_args, &cmd)
        }
        CommonCommand::Dove(cmd) => {
            let ctx = cmd.context(cwd, move_args)?;
            cmd.apply(ctx)
        }
    }
}

fn create_long_version() -> &'static str {
    let dove: String = [DOVE_VERSION, DOVE_HASH]
        .iter()
        .filter(|str| !str.is_empty())
        .map(|str| str.to_string())
        .collect::<Vec<String>>()
        .join("-");
    let diem: String = [DIEM_VERSION, DIEM_HASH]
        .iter()
        .filter(|str| !str.is_empty())
        .map(|str| str.to_string())
        .collect::<Vec<String>>()
        .join("-");

    Box::leak(
        format!(
            "{}\n\
            Diem {}\n\
            Move-Stdlib {}",
            dove, diem, PONT_STDLIB_VERSION
        )
            .into_boxed_str(),
    )
}
