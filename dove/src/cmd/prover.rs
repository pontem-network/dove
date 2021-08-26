use std::ffi::OsStr;
use anyhow::{ensure, Result};
use move_prover::{cli::Options, run_move_prover_errors_to_stderr};
use structopt::StructOpt;
use serde::Deserialize;
use lang::compiler::{file::find_move_files};

use crate::context::Context;

use super::Cmd;
use boogie_backend::options::BoogieOptions;
use lang::compiler::preprocessor::BuilderPreprocessor;
use std::path::PathBuf;
use std::fs::read_to_string;

#[cfg(target_family = "unix")]
const BOOGIE_EXE: &str = "boogie";
#[cfg(target_family = "unix")]
const Z3_EXE: &str = "z3";
#[cfg(target_family = "unix")]
const CVC4_EXE: &str = "cvc4";

#[cfg(target_family = "windows")]
const BOOGIE_EXE: &str = "boogie.exe";
#[cfg(target_family = "windows")]
const Z3_EXE: &str = "z3.exe";
#[cfg(target_family = "windows")]
const CVC4_EXE: &str = "cvc4.exe";

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
    /// Override path to cvc4 executable.
    #[structopt(long)]
    cvc4_exe: Option<String>,
}

impl Cmd for Prove {
    fn apply(self, ctx: Context) -> Result<()>
    where
        Self: std::marker::Sized,
    {
        let (boogie_exe, z3_exe, cvc4_exe) = self.get_prover_conf(&ctx).map(|conf| {
            (
                conf.boogie_exe.unwrap_or_default(),
                conf.z3_exe.unwrap_or_default(),
                conf.cvc4_exe.unwrap_or_default(),
            )
        })?;

        ensure!(is_boogie_available(&boogie_exe), "boogie executable not found in PATH. Please install it from https://github.com/boogie-org/boogie");
        ensure!(is_z3_available(&z3_exe), "z3 executable not found in PATH. Please install it from https://github.com/Z3Prover/z3");
        ensure!(is_cvc4_available(&cvc4_exe), "cvc4 executable not found in PATH. Please install it from https://github.com/CVC4/CVC4-archived");

        let move_deps = find_move_files(&ctx.build_index(false)?.0.into_deps_roots())
            .map(|p| p.to_string_lossy().to_string())
            .collect();

        let dirs = ctx.paths_for(&[
            &ctx.manifest.layout.scripts_dir,
            &ctx.manifest.layout.modules_dir,
        ]);
        let move_sources = find_move_files(&dirs)
            .map(|path| path.to_string_lossy().to_string())
            .collect::<Vec<_>>();
        let boogie_options = ctx.manifest.boogie_options.clone().unwrap_or_default();
        let options = Options {
            backend: BoogieOptions {
                boogie_exe,
                z3_exe,
                cvc4_exe,
                ..boogie_options
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
impl Prove {
    fn get_prover_conf(&self, ctx: &Context) -> Result<ProverConfig, anyhow::Error> {
        let mut conf = ProverConfig::new()?;
        // prover-env.toml
        let toml_conf = ctx.path_for(&ctx.manifest.layout.prover_toml);
        if toml_conf.exists() {
            let toml_conf = toml::from_str::<ProverConfig>(&read_to_string(&toml_conf)?)?;
            conf.boogie_exe = toml_conf.boogie_exe.or_else(|| conf.boogie_exe.clone());
            conf.z3_exe = toml_conf.z3_exe.or_else(|| conf.z3_exe.clone());
            conf.cvc4_exe = toml_conf.cvc4_exe.or_else(|| conf.cvc4_exe.clone());
        }
        // cmd
        conf.boogie_exe = self.boogie_exe.clone().or_else(|| conf.boogie_exe.clone());
        conf.z3_exe = self.z3_exe.clone().or_else(|| conf.z3_exe.clone());
        conf.cvc4_exe = self.z3_exe.clone().or_else(|| conf.cvc4_exe.clone());

        Ok(conf)
    }
}

fn is_boogie_available(boogie_exe: &str) -> bool {
    is_executable_available(boogie_exe, &["/help"])
}

fn is_z3_available(z3_exe: &str) -> bool {
    is_executable_available(z3_exe, &["-h"])
}

fn is_cvc4_available(cvc4_exe: &str) -> bool {
    is_executable_available(cvc4_exe, &["-V"])
}

/// Checks if executable is available in path by running it.
fn is_executable_available<S: AsRef<OsStr>, I: IntoIterator<Item = S>>(
    executable: &str,
    args: I,
) -> bool {
    let result = std::process::Command::new(executable).args(args).output();

    match result {
        Ok(result) => {
            let status_success = result.status.success();
            if !status_success {
                println!("Warning: {}", String::from_utf8(result.stderr).unwrap());
            }
            result.status.success()
        }
        Err(_) => false,
    }
}

/// get path to z3
fn get_path_z3() -> Result<String, anyhow::Error> {
    let env_path = std::env::var("PATH")?;

    #[cfg(not(target_family = "windows"))]
    let separator = ':';

    #[cfg(target_family = "windows")]
    let separator = ';';

    env_path
        .split(separator)
        .map(PathBuf::from)
        .map(|path| path.join(Z3_EXE))
        .find(|path|path.exists())
        .and_then(|path|path.to_str().map(|path|path.to_string()))
        .ok_or_else(|| anyhow!("z3 executable not found in PATH. Please install it from https://github.com/Z3Prover/z3"))
}

/// Paths to the boogie, z3 and cvc4 binaries
#[derive(Debug, Deserialize)]
struct ProverConfig {
    #[serde(rename = "boogie")]
    pub boogie_exe: Option<String>,
    #[serde(rename = "z3")]
    pub z3_exe: Option<String>,
    #[serde(rename = "cvc4")]
    pub cvc4_exe: Option<String>,
}
impl ProverConfig {
    fn new() -> Result<Self, anyhow::Error> {
        Ok(ProverConfig {
            boogie_exe: Some(BOOGIE_EXE.to_string()),
            z3_exe: Some(get_path_z3()?),
            cvc4_exe: Some(CVC4_EXE.to_string()),
        })
    }
}
