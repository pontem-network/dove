use move_prover::{cli::Options, run_move_prover_errors_to_stderr};
use structopt::StructOpt;
use anyhow::{ensure, Result};
use crate::context::Context;

use super::Cmd;

#[cfg(target_family = "unix")]
const BOOGIE_EXE: &str = "boogie";

#[cfg(target_family = "windows")]
const BOOGIE_EXE: &str = "boogie.exe";

/// Run move-prover on project files.
/// Prints output to stderr.
#[derive(Debug, StructOpt)]
pub struct Prove {
    /// Override path to boogie executable.
    #[structopt(long)]
    boogie_exe: Option<String>,
}

impl Cmd for Prove {
    fn apply(self, ctx: Context) -> Result<()>
    where
        Self: std::marker::Sized,
    {
        let boogie_exe = self.boogie_exe.unwrap_or_else(|| BOOGIE_EXE.to_string());
        ensure!(is_boogie_available(&boogie_exe), "boogie executable not found in PATH. Please install it from https://github.com/boogie-org/boogie");

        let dirs = ctx.paths_for(&[
            &ctx.manifest.layout.scripts_dir,
            &ctx.manifest.layout.modules_dir,
        ]);

        let mut index = ctx.build_index()?;
        let move_deps = index
            .make_dependency_set(&dirs)?
            .into_iter()
            .map(|s| s.to_string())
            .collect();
        let move_sources = dirs
            .into_iter()
            .map(|p| p.to_string_lossy().to_string())
            .collect();

        let options = Options {
            backend: boogie_backend_v2::options::BoogieOptions {
                boogie_exe,
                ..Default::default()
            },
            move_deps,
            move_sources,
            ..Default::default()
        };
        run_move_prover_errors_to_stderr(options)
    }
}

/// Checks if `boogie` executable is available in path by running it with `/help` flag.
fn is_boogie_available(boogie_exe: &str) -> bool {
    let status = std::process::Command::new(boogie_exe).arg("/help").status();
    match status {
        Ok(status) => status.success(),
        Err(_) => false,
    }
}
