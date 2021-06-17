use std::ffi::OsStr;
use lang::compiler::dialects::Dialect;
use lang::compiler::{file::find_move_files, parser};
use lang::compiler::mut_string::MutString;
use move_prover::{cli::Options, run_move_prover_errors_to_stderr};
use structopt::StructOpt;
use anyhow::{ensure, Result};

use crate::context::Context;

use std::path::{PathBuf, Path};
use std::process::Stdio;

use super::Cmd;

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

        let output_dir = ctx.path_for(&ctx.manifest.layout.move_prover_output);
        if output_dir.exists() {
            std::fs::remove_dir_all(&output_dir)?;
        }
        let output_modules_dir = output_dir.join("modules");
        let output_scripts_dir = output_dir.join("scripts");
        std::fs::create_dir_all(&output_modules_dir)?;
        std::fs::create_dir_all(&output_scripts_dir)?;

        let dialect = &*ctx.dialect;
        let sender = Some(ctx.manifest.package.account_address.clone());
        prepare_sources(
            dialect,
            &sender,
            &ctx.path_for(&ctx.manifest.layout.modules_dir),
            &output_modules_dir,
        )?;
        prepare_sources(
            dialect,
            &sender,
            &ctx.path_for(&ctx.manifest.layout.scripts_dir),
            &output_scripts_dir,
        )?;

        let options = Options {
            backend: boogie_backend_v2::options::BoogieOptions {
                boogie_exe,
                z3_exe,
                ..Default::default()
            },
            move_deps,
            move_sources: vec![output_dir.to_string_lossy().to_string()],
            account_address: ctx.manifest.package.account_address.clone(),
            ..Default::default()
        };
        options.setup_logging();
        run_move_prover_errors_to_stderr(options)
    }
}

/// Normalizes move files sources and outputs them into `output_dir`.
///
/// Namely, replaces `{{sender}}` placeholders with the actual value and translates addresses
/// to current dialect.
fn prepare_sources(
    dialect: &dyn Dialect,
    sender: &Option<String>,
    dir: &Path,
    output_dir: &Path,
) -> Result<()> {
    let move_files = find_move_files(dir)?.into_iter().map(PathBuf::from);
    for file_path in move_files {
        let content = std::fs::read_to_string(&file_path)?;
        let mut mut_content = MutString::new(&content);
        parser::normalize_source_text(dialect, (&content, &mut mut_content), &sender);
        let content = mut_content.freeze();

        let relative_path = file_path
            .strip_prefix(&dir)
            .expect("File path does not contain parent dir");
        let output_path = output_dir.join(relative_path);
        std::fs::write(&output_path, content)?;
    }

    Ok(())
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
