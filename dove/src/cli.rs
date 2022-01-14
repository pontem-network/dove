use std::env;
use std::fs;
use std::ffi::OsString;
use std::path::{PathBuf, Path};

use anyhow::{Result, Error};
use structopt::StructOpt;
use semver::{Version, VersionReq};

use move_cli::{Command as DiemCommand, experimental, Move, package, run_cli, sandbox};
use move_core_types::account_address::AccountAddress;
use move_core_types::errmap::ErrorMapping;

use crate::{DOVE_VERSION, DOVE_HASH, PONT_STDLIB_VERSION, DIEM_VERSION, DIEM_HASH};
use crate::cmd::Cmd;
use crate::cmd::new::New;
use crate::cmd::build::Build;
use crate::cmd::clean::{Clean, run_internal_clean};
use crate::cmd::export::Export;
use crate::cmd::init::Init;
use crate::cmd::run::Run;
use crate::cmd::test::Test;
use crate::cmd::tx::CreateTransactionCmd;
use crate::context::Context;
use move_cli::DEFAULT_STORAGE_DIR;

const HASH_FILE_NAME: &str = ".version";

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

/// Common command. Contains move-cli and dove commands.
pub enum CommonCommand {
    /// Diem(move-cli) commands.
    Diem(DiemCommand),
    /// Dove commands.
    Dove(Box<dyn Cmd>),
}

/// Move cli and dove commands.
#[derive(StructOpt)]
pub enum Command {
    /// Execute a package command. Executed in the current directory or the closest containing Move
    /// package.
    #[structopt(name = "package")]
    Package {
        /// cmd.
        #[structopt(subcommand)]
        cmd: package::cli::PackageCommand,
    },
    /// Execute a sandbox command.
    #[structopt(name = "sandbox")]
    Sandbox {
        /// Directory storing Move resources, events, and module bytecodes produced by module publishing
        /// and script execution.
        #[structopt(long, default_value = DEFAULT_STORAGE_DIR, parse(from_os_str))]
        storage_dir: PathBuf,
        /// cmd
        #[structopt(subcommand)]
        cmd: sandbox::cli::SandboxCommand,
    },
    /// (Experimental) Run static analyses on Move source or bytecode.
    #[structopt(name = "experimental")]
    Experimental {
        /// Directory storing Move resources, events, and module bytecodes produced by module publishing
        /// and script execution.
        #[structopt(long, default_value = DEFAULT_STORAGE_DIR, parse(from_os_str))]
        storage_dir: PathBuf,
        /// cmd
        #[structopt(subcommand)]
        cmd: experimental::cli::ExperimentalCommand,
    },

    /// Init new project with existing folder.
    #[structopt(about = "Init directory as move project")]
    Init {
        /// Command.
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
    /// Build package.
    #[structopt(about = "Build project")]
    Build {
        /// Command.
        #[structopt(flatten)]
        cmd: Build,
    },
    /// Clean project.
    #[structopt(about = "Remove the target directory")]
    Clean {
        /// Command.
        #[structopt(flatten)]
        cmd: Clean,
    },
    /// Test package.
    #[structopt(about = "Run move tests")]
    Test {
        /// Command.
        #[structopt(flatten)]
        cmd: Test,
    },
    /// Run script and modules script function.
    #[structopt(about = "Run move script")]
    Run {
        /// Command.
        #[structopt(flatten)]
        cmd: Run,
    },
    /// Create transaction.
    #[structopt(about = "Create transaction")]
    Tx {
        /// Command.
        #[structopt(flatten)]
        cmd: CreateTransactionCmd,
    },
    /// Run move prover.
    #[structopt(about = "Run move prover")]
    Prove {
        /// Command.
        #[structopt(flatten)]
        cmd: crate::cmd::prover::Prove,
    },
    /// Migrate from Dove project to the Move cli project.
    #[structopt(about = "Export dove.toml => move .toml")]
    Export {
        /// Command.
        #[structopt(flatten)]
        cmd: Export,
    },
}

impl Command {
    /// Creates `CommonCommand`.
    /// Split commands to two different execution backend (move-cli, dove).
    pub fn select_backend(self) -> CommonCommand {
        match self {
            Command::Package { cmd } => CommonCommand::Diem(DiemCommand::Package { cmd }),
            Command::Sandbox { storage_dir, cmd } => {
                CommonCommand::Diem(DiemCommand::Sandbox { storage_dir, cmd })
            }
            Command::Experimental { storage_dir, cmd } => {
                CommonCommand::Diem(DiemCommand::Experimental { storage_dir, cmd })
            }

            Command::New { cmd } => CommonCommand::Dove(Box::new(cmd)),
            Command::Init { cmd } => CommonCommand::Dove(Box::new(cmd)),
            Command::Build { cmd } => CommonCommand::Dove(Box::new(cmd)),
            Command::Clean { cmd } => CommonCommand::Dove(Box::new(cmd)),
            Command::Test { cmd } => CommonCommand::Dove(Box::new(cmd)),
            Command::Run { cmd } => CommonCommand::Dove(Box::new(cmd)),
            Command::Tx { cmd } => CommonCommand::Dove(Box::new(cmd)),
            Command::Prove { cmd } => CommonCommand::Dove(Box::new(cmd)),
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

    if let Some(minimal_version) = get_minimal_dove_version(&cwd) {
        check_dove_version(&minimal_version)?;
    }

    match commands {
        CommonCommand::Diem(cmd) => {
            let error_descriptions: ErrorMapping =
                bcs::from_bytes(move_stdlib::error_descriptions())?;
            run_cli(
                move_stdlib::natives::all_natives(
                    AccountAddress::from_hex_literal("0x1").unwrap(),
                ),
                &error_descriptions,
                &move_args,
                &cmd,
            )
        }
        CommonCommand::Dove(mut cmd) => {
            let mut ctx = cmd.context(cwd, move_args)?;

            if !check_manifest_hash(&ctx) {
                run_internal_clean(&mut ctx)?;
                store_manifest_checksum(&ctx)?;
            }
            cmd.apply(&mut ctx)
        }
    }
}

/// Check if Dove version is suitable for this project
fn check_dove_version(req_ver: &str) -> Result<(), Error> {
    let act_ver = env!("CARGO_PKG_VERSION");
    let req = VersionReq::parse(req_ver)
        .map_err(|err| Error::new(err).context("Failed to parse dove_version from Move.toml"))?;
    let actual = Version::parse(act_ver).expect("Expected valid dove version");
    if !req.matches(&actual) {
        Err(anyhow!("The dove version must meet the conditions '{}'. The current version of dove is '{}'.", req_ver, act_ver))
    } else {
        Ok(())
    }
}

/// Move.toml has been updated
fn check_manifest_hash(ctx: &Context) -> bool {
    let path_version = ctx.project_dir.join("build").join(HASH_FILE_NAME);
    if !path_version.exists() {
        return false;
    }

    let old_version = fs::read_to_string(&path_version)
        .unwrap_or_default()
        .parse::<u64>()
        .unwrap_or_default();

    ctx.manifest_hash == old_version
}

/// Writing the hash move.toml to file
fn store_manifest_checksum(ctx: &Context) -> Result<()> {
    let build_path = ctx.project_dir.join("build");
    if !build_path.exists() {
        fs::create_dir_all(&build_path)?;
    }
    let path_version = build_path.join(HASH_FILE_NAME);
    if path_version.exists() {
        fs::remove_file(&path_version)?;
    }
    fs::write(&path_version, ctx.manifest_hash.to_string())?;
    Ok(())
}

/// To display the full version of "Dove"
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

/// Get minimal version of Dove from Move.toml
fn get_minimal_dove_version(project_path: &Path) -> Option<String> {
    let move_toml_path = project_path.join("Move.toml");
    if !move_toml_path.exists() {
        return None;
    }
    let move_toml_content = std::fs::read_to_string(&move_toml_path).ok()?;
    let move_toml = toml::from_str::<toml::Value>(&move_toml_content).ok()?;
    move_toml
        .get("package")
        .and_then(|pack| pack.get("dove_version"))
        .and_then(|name| name.as_str().map(|t| t.to_string()))
}

#[cfg(test)]
mod tests {
    use semver::Version;
    use super::check_dove_version;

    #[test]
    fn test_dove_version() {
        Version::parse(env!("CARGO_PKG_VERSION")).unwrap();
    }

    #[test]
    fn test_check_dove_version() {
        check_dove_version(">=1.2.3, <1.8.0").unwrap();
        check_dove_version("<1.2.2").unwrap_err();
    }
}
