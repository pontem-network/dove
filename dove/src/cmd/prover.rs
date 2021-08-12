use std::ffi::OsStr;
use std::process::Stdio;

use anyhow::{ensure, Result};
use move_prover::{cli::Options, run_move_prover_errors_to_stderr};
use structopt::StructOpt;

use lang::compiler::{file::find_move_files};

use crate::context::Context;

use super::Cmd;
use boogie_backend::options::BoogieOptions;
use lang::compiler::preprocessor::BuilderPreprocessor;

#[cfg(target_family = "unix")]
const BOOGIE_EXE: &str = "boogie";
#[cfg(target_family = "unix")]
const Z3_EXE: &str = "z3";

#[cfg(target_family = "windows")]
const BOOGIE_EXE: &str = "boogie.exe";
#[cfg(target_family = "windows")]
const Z3_EXE: &str = "z3.exe";

/// Run move-prover on project files.
/// Prints output to stderr.
#[derive(Debug, StructOpt)]
pub struct Prove {
    /// Override path to boogie executable.
    #[structopt(long)]
    boogie_exe: Option<String>,
    /// Override path to z3 executable.
    #[structopt(long)]
    z3_exe: Option<String>,
}

impl Cmd for Prove {
    fn apply(self, ctx: Context) -> Result<()>
    where
        Self: std::marker::Sized,
    {
        let boogie_exe = self.boogie_exe.unwrap_or_else(|| BOOGIE_EXE.to_string());
        ensure!(is_boogie_available(&boogie_exe), "boogie executable not found in PATH. Please install it from https://github.com/boogie-org/boogie");
        let z3_exe = self.z3_exe.unwrap_or_else(|| Z3_EXE.to_string());
        ensure!(is_z3_available(&z3_exe), "z3 executable not found in PATH. Please install it from https://github.com/Z3Prover/z3");

        let move_deps = find_move_files(&ctx.build_index()?.into_deps_roots())
            .map(|p| p.to_string_lossy().to_string())
            .collect();

        let dirs = ctx.paths_for(&[
            &ctx.manifest.layout.scripts_dir,
            &ctx.manifest.layout.modules_dir,
        ]);
        let move_sources = find_move_files(&dirs)
            .map(|path| path.to_string_lossy().to_string())
            .collect::<Vec<_>>();

        let options = Options {
            backend: BoogieOptions {
                boogie_exe,
                z3_exe,
                ..Default::default()
            },
            move_deps,
            move_sources,
            ..Default::default()
        };
        options.setup_logging();
        let mut preprocessor =
            BuilderPreprocessor::new(ctx.dialect.as_ref(), Some(ctx.account_address()?));

        run_move_prover_errors_to_stderr(options, &mut preprocessor)
    }
}

fn is_boogie_available(boogie_exe: &str) -> bool {
    is_executable_available(boogie_exe, &["/help"])
}

fn is_z3_available(z3_exe: &str) -> bool {
    is_executable_available(z3_exe, &["-h"])
}

/// Checks if executable is available in path by running it.
fn is_executable_available<S: AsRef<OsStr>, I: IntoIterator<Item = S>>(
    executable: &str,
    args: I,
) -> bool {
    let status = std::process::Command::new(executable)
        .args(args)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();
    match status {
        Ok(status) => status.success(),
        Err(_) => false,
    }
}
