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

        let artifacts_dir = ctx
            .path_for(&ctx.manifest.layout.artifacts)
            .join("move-prover");
        if artifacts_dir.exists() {
            std::fs::remove_dir_all(&artifacts_dir)?;
        }
        std::fs::create_dir_all(artifacts_dir.join("modules"))?;
        std::fs::create_dir_all(artifacts_dir.join("scripts"))?;

        let dialect = &*ctx.dialect;
        let sender = Some(ctx.manifest.package.account_address.clone());
        prepare_sources(dialect, &sender, &ctx.path_for(&ctx.manifest.layout.modules_dir), &artifacts_dir.join("modules"))?;
        prepare_sources(dialect, &sender, &ctx.path_for(&ctx.manifest.layout.scripts_dir), &artifacts_dir.join("scripts"))?;

        let options = Options {
            backend: boogie_backend_v2::options::BoogieOptions {
                boogie_exe,
                z3_exe: "z3".into(),
                ..Default::default()
            },
            move_deps,
            move_sources: vec![artifacts_dir.to_string_lossy().to_string()],
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

        let relative_path = file_path.strip_prefix(&dir).expect("File path does not contain parent dir");
        let output_path = output_dir.join(relative_path);
        std::fs::write(&output_path, content)?;
    }

    Ok(())
}

/// Checks if `boogie` executable is available in path by running it with `/help` flag.
fn is_boogie_available(boogie_exe: &str) -> bool {
    let status = std::process::Command::new(boogie_exe)
        .arg("/help")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();
    match status {
        Ok(status) => status.success(),
        Err(_) => false,
    }
}
