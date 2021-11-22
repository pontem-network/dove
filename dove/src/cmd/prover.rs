use std::str::FromStr;
use std::ffi::OsStr;
use std::path::PathBuf;
use std::{env, fs};
use std::fs::read_dir;
use anyhow::{ensure, Result};
use structopt::StructOpt;
use serde::Deserialize;
use boogie_backend::options::BoogieOptions;
use super::Cmd;
use crate::context::Context;
use crate::cmd::build::run_internal_build;
use move_prover::{cli::Options, run_move_prover_errors_to_stderr};

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
    fn apply(&mut self, ctx: &mut Context) -> Result<()>
    where
        Self: Sized,
    {
        let (boogie_exe, z3_exe, cvc4_exe) = self.make_config(ctx).map(|conf| {
            (
                conf.boogie_exe.unwrap_or_default(),
                conf.z3_exe.unwrap_or_default(),
                conf.cvc4_exe.unwrap_or_default(),
            )
        })?;

        ensure!(is_boogie_available(&boogie_exe), "boogie executable not found in PATH. Please install it from https://github.com/boogie-org/boogie");
        ensure!(is_z3_available(&z3_exe), "z3 executable not found in PATH. Please install it from https://github.com/Z3Prover/z3");

        if !cvc4_exe.is_empty() {
            ensure!(is_cvc4_available(&cvc4_exe), "cvc4 executable not found in PATH. Please install it from https://github.com/CVC4/CVC4-archived");
        }

        // Build a project
        // In order for the dependencies to be loaded
        run_internal_build(ctx)?;

        // Get paths to all dependency files
        let move_deps = get_dependency_paths(ctx)?;
        // Get all project files "move"
        let move_sources = get_project_movefile_paths(ctx)?;

        let boogie_options_path = ctx.boogie_options_path();
        let mut boogie_options = if boogie_options_path.exists() {
            let boogie_options_string = fs::read_to_string(boogie_options_path)?;
            toml::from_str(&boogie_options_string)?
        } else {
            BoogieOptions::default()
        };

        if cvc4_exe.is_empty() && boogie_options.use_cvc4 {
            println!("Warning: cvc4 is not defined.");
            boogie_options.use_cvc4 = false;
        }

        // addresses
        let mut move_named_address_values: Vec<String> = ctx
            .manifest
            .addresses
            .as_ref()
            .map(|addresses| {
                addresses
                    .iter()
                    .map(|(name, value)| {
                        value
                            .map(|value| format!("{}={}", name, value.to_hex_literal()))
                            .unwrap_or_else(|| name.to_string())
                    })
                    .collect()
            })
            .unwrap_or_default();
        move_named_address_values.extend(Options::default().move_named_address_values);

        let options = Options {
            backend: BoogieOptions {
                boogie_exe,
                z3_exe,
                cvc4_exe,
                ..boogie_options
            },
            move_deps,
            move_sources,
            move_named_address_values,
            ..Default::default()
        };

        options.setup_logging();
        run_move_prover_errors_to_stderr(options)
    }
}

impl Prove {
    fn make_config(&mut self, ctx: &Context) -> Result<ProverConfig, anyhow::Error> {
        let mut conf = ProverConfig::new();
        // <PROJECT_DIR>/prover-env.toml
        let toml_conf = ctx.project_dir.join("prover-env.toml");
        if toml_conf.exists() {
            let toml_conf = toml::from_str::<ProverConfig>(&fs::read_to_string(&toml_conf)?)?;
            if let Some(boogie_exe) = toml_conf.boogie_exe {
                conf.boogie_exe = Some(boogie_exe);
            }
            if let Some(z3_exe) = toml_conf.z3_exe {
                conf.z3_exe = Some(z3_exe);
            }
            if let Some(cvc4_exe) = toml_conf.cvc4_exe {
                conf.cvc4_exe = Some(cvc4_exe);
            }
        }
        // cmd
        if let Some(boogie_exe) = self.boogie_exe.take() {
            conf.boogie_exe = Some(boogie_exe);
        }
        if let Some(z3_exe) = self.z3_exe.take() {
            conf.z3_exe = Some(z3_exe);
        }
        if let Some(cvc4_exe) = self.cvc4_exe.take() {
            conf.cvc4_exe = Some(cvc4_exe);
        }
        conf.normalize()?;
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

fn path() -> Option<Vec<PathBuf>> {
    if let Ok(env_path) = std::env::var("PATH") {
        #[cfg(not(target_family = "windows"))]
        let separator = ':';

        #[cfg(target_family = "windows")]
        let separator = ';';

        Some(env_path.split(separator).map(PathBuf::from).collect())
    } else {
        None
    }
}

fn find_path(paths: &[PathBuf], name: &str) -> Option<String> {
    paths
        .iter()
        .map(|path| path.join(name))
        .find(|path| path.exists())
        .and_then(|path| path.to_str().map(|path| path.to_string()))
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
    fn new() -> Self {
        if let Some(env_path) = path() {
            ProverConfig {
                boogie_exe: find_path(&env_path, BOOGIE_EXE),
                z3_exe: find_path(&env_path, Z3_EXE),
                cvc4_exe: find_path(&env_path, CVC4_EXE),
            }
        } else {
            ProverConfig {
                boogie_exe: None,
                z3_exe: None,
                cvc4_exe: None,
            }
        }
    }

    fn normalize(&mut self) -> Result<(), anyhow::Error> {
        fn canonicalize(path: Option<&mut String>, home: &str) -> Result<(), anyhow::Error> {
            if let Some(path) = path {
                if path.starts_with("~/") {
                    *path = path.replacen("~", home, 1);
                }
                let mut path_buff = PathBuf::from_str(path)?;
                if path_buff.exists() {
                    path_buff = path_buff.canonicalize()?;
                }
                *path = path_buff.to_string_lossy().to_string();
            }
            Ok(())
        }

        if let Some(home) = env::var_os("HOME") {
            let home = home.to_string_lossy().to_string();

            canonicalize(self.boogie_exe.as_mut(), &home)?;
            canonicalize(self.z3_exe.as_mut(), &home)?;
            canonicalize(self.cvc4_exe.as_mut(), &home)?;
        }
        Ok(())
    }
}

/// Paths to all dependency files
///     <PROJECT_PATH>/build/<DEPENDENCE_NAME>/sources/*.move
fn get_dependency_paths(ctx: &Context) -> Result<Vec<String>> {
    let build_path = ctx.project_dir.join("build");
    if !build_path.exists() {
        return Ok(Vec::new());
    }
    let list = fs::read_dir(&build_path)?
        .filter_map(|path| path.ok())
        .map(|path| path.path())
        .filter(|dir| {
            dir.is_dir()
                && dir.file_name().and_then(|name| name.to_str())
                    != Some(ctx.manifest.package.name.as_str())
        })
        .map(|dir| dir.join("sources"))
        .filter(|sources| sources.exists())
        .filter_map(|dir| {
            let files: Vec<PathBuf> = read_dir(dir)
                .ok()?
                .filter_map(|path| path.ok())
                .map(|path| path.path())
                .filter(|path| {
                    path.is_file() && path.extension().and_then(|ex| ex.to_str()) == Some("move")
                })
                .collect();
            Some(files)
        })
        .flatten()
        .map(|path| path.to_string_lossy().to_string())
        .collect();

    Ok(list)
}

/// Get all "move" project files
///     <PROJECT_PATH>/build/<PROJECT_NAME>/sources/*.move
fn get_project_movefile_paths(ctx: &Context) -> Result<Vec<String>> {
    let source_path = ctx
        .project_dir
        .join("build")
        .join(ctx.manifest.package.name.as_str())
        .join("sources");
    if !source_path.exists() {
        return Ok(Vec::new());
    }
    let files = fs::read_dir(&source_path)?
        .filter_map(|path| path.ok())
        .map(|path| path.path())
        .filter(|path| {
            path.is_file() && path.extension().and_then(|ex| ex.to_str()) == Some("move")
        })
        .map(|path| path.to_string_lossy().to_string())
        .collect();

    Ok(files)
}
