use std::ffi::OsStr;
use anyhow::{ensure, Result};
// use move_prover::{cli::Options, run_move_prover_errors_to_stderr};
use structopt::StructOpt;
use serde::Deserialize;
// use lang::compiler::{file::find_move_files};

use crate::context::Context;

use super::Cmd;
// use boogie_backend::options::BoogieOptions;
// use lang::compiler::preprocessor::BuilderPreprocessor;
use std::path::PathBuf;
use std::fs::read_to_string;
use std::env;
use std::str::FromStr;

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
    fn apply(&self, ctx: Context) -> Result<()> where Self: Sized {
        todo!()
    }
    // fn apply(mut self, ctx: Context) -> Result<()>
    // where
    //     Self: std::marker::Sized,
    // {
    //     let (boogie_exe, z3_exe, cvc4_exe) = self.make_config(&ctx).map(|conf| {
    //         (
    //             conf.boogie_exe.unwrap_or_default(),
    //             conf.z3_exe.unwrap_or_default(),
    //             conf.cvc4_exe.unwrap_or_default(),
    //         )
    //     })?;
    //
    //     ensure!(is_boogie_available(&boogie_exe), "boogie executable not found in PATH. Please install it from https://github.com/boogie-org/boogie");
    //     ensure!(is_z3_available(&z3_exe), "z3 executable not found in PATH. Please install it from https://github.com/Z3Prover/z3");
    //
    //     if !cvc4_exe.is_empty() {
    //         ensure!(is_cvc4_available(&cvc4_exe), "cvc4 executable not found in PATH. Please install it from https://github.com/CVC4/CVC4-archived");
    //     }
    //
    //     let move_deps = find_move_files(&ctx.build_index()?.0.into_deps_roots())
    //         .map(|p| p.to_string_lossy().to_string())
    //         .collect();
    //
    //     let dirs = ctx.paths_for(&[
    //         &ctx.manifest.layout.scripts_dir,
    //         &ctx.manifest.layout.modules_dir,
    //     ]);
    //     let move_sources = find_move_files(&dirs)
    //         .map(|path| path.to_string_lossy().to_string())
    //         .collect::<Vec<_>>();
    //
    //     let mut boogie_options = ctx.manifest.boogie_options.clone().unwrap_or_default();
    //     if cvc4_exe.is_empty() && boogie_options.use_cvc4 {
    //         println!("Warning: cvc4 is not defined.");
    //         boogie_options.use_cvc4 = false;
    //     }
    //
    //     let options = Options {
    //         backend: BoogieOptions {
    //             boogie_exe,
    //             z3_exe,
    //             cvc4_exe,
    //             ..boogie_options
    //         },
    //         move_deps,
    //         move_sources,
    //         ..Default::default()
    //     };
    //     options.setup_logging();
    //     let address = ctx.account_address_str()?;
    //     let mut preprocessor = BuilderPreprocessor::new(ctx.dialect.as_ref(), &address);
    //
    //     run_move_prover_errors_to_stderr(options, &mut preprocessor)
    // }
}
//
// impl Prove {
//     fn make_config(&mut self, ctx: &Context) -> Result<ProverConfig, anyhow::Error> {
//         let mut conf = ProverConfig::new();
//         // prover-env.toml
//         let toml_conf = ctx.path_for(&ctx.manifest.layout.prover_toml);
//         if toml_conf.exists() {
//             let toml_conf = toml::from_str::<ProverConfig>(&read_to_string(&toml_conf)?)?;
//             if let Some(boogie_exe) = toml_conf.boogie_exe {
//                 conf.boogie_exe = Some(boogie_exe);
//             }
//             if let Some(z3_exe) = toml_conf.z3_exe {
//                 conf.z3_exe = Some(z3_exe);
//             }
//             if let Some(cvc4_exe) = toml_conf.cvc4_exe {
//                 conf.cvc4_exe = Some(cvc4_exe);
//             }
//         }
//         // cmd
//         if let Some(boogie_exe) = self.boogie_exe.take() {
//             conf.boogie_exe = Some(boogie_exe);
//         }
//         if let Some(z3_exe) = self.z3_exe.take() {
//             conf.z3_exe = Some(z3_exe);
//         }
//         if let Some(cvc4_exe) = self.cvc4_exe.take() {
//             conf.cvc4_exe = Some(cvc4_exe);
//         }
//         conf.normalize()?;
//         Ok(conf)
//     }
// }
//
// fn is_boogie_available(boogie_exe: &str) -> bool {
//     is_executable_available(boogie_exe, &["/help"])
// }
//
// fn is_z3_available(z3_exe: &str) -> bool {
//     is_executable_available(z3_exe, &["-h"])
// }
//
// fn is_cvc4_available(cvc4_exe: &str) -> bool {
//     is_executable_available(cvc4_exe, &["-V"])
// }
//
// /// Checks if executable is available in path by running it.
// fn is_executable_available<S: AsRef<OsStr>, I: IntoIterator<Item = S>>(
//     executable: &str,
//     args: I,
// ) -> bool {
//     let result = std::process::Command::new(executable).args(args).output();
//
//     match result {
//         Ok(result) => {
//             let status_success = result.status.success();
//             if !status_success {
//                 println!("Warning: {}", String::from_utf8(result.stderr).unwrap());
//             }
//             result.status.success()
//         }
//         Err(_) => false,
//     }
// }
//
// fn path() -> Option<Vec<PathBuf>> {
//     if let Ok(env_path) = std::env::var("PATH") {
//         #[cfg(not(target_family = "windows"))]
//         let separator = ':';
//
//         #[cfg(target_family = "windows")]
//         let separator = ';';
//
//         Some(env_path.split(separator).map(PathBuf::from).collect())
//     } else {
//         None
//     }
// }
//
// fn find_path(paths: &[PathBuf], name: &str) -> Option<String> {
//     paths
//         .iter()
//         .map(|path| path.join(name))
//         .find(|path| path.exists())
//         .and_then(|path| path.to_str().map(|path| path.to_string()))
// }
//
// /// Paths to the boogie, z3 and cvc4 binaries
// #[derive(Debug, Deserialize)]
// struct ProverConfig {
//     #[serde(rename = "boogie")]
//     pub boogie_exe: Option<String>,
//     #[serde(rename = "z3")]
//     pub z3_exe: Option<String>,
//     #[serde(rename = "cvc4")]
//     pub cvc4_exe: Option<String>,
// }
//
// impl ProverConfig {
//     fn new() -> Self {
//         if let Some(env_path) = path() {
//             ProverConfig {
//                 boogie_exe: find_path(&env_path, BOOGIE_EXE),
//                 z3_exe: find_path(&env_path, Z3_EXE),
//                 cvc4_exe: find_path(&env_path, CVC4_EXE),
//             }
//         } else {
//             ProverConfig {
//                 boogie_exe: None,
//                 z3_exe: None,
//                 cvc4_exe: None,
//             }
//         }
//     }
//
//     fn normalize(&mut self) -> Result<(), anyhow::Error> {
//         fn canonicalize(path: Option<&mut String>, home: &str) -> Result<(), anyhow::Error> {
//             if let Some(path) = path {
//                 if path.starts_with("~/") {
//                     *path = path.replacen("~", home, 1);
//                 }
//                 let mut path_buff = PathBuf::from_str(path)?;
//                 if path_buff.exists() {
//                     path_buff = path_buff.canonicalize()?;
//                 }
//                 *path = path_buff.to_string_lossy().to_string();
//             }
//             Ok(())
//         }
//
//         if let Some(home) = env::var_os("HOME") {
//             let home = home.to_string_lossy().to_string();
//
//             canonicalize(self.boogie_exe.as_mut(), &home)?;
//             canonicalize(self.z3_exe.as_mut(), &home)?;
//             canonicalize(self.cvc4_exe.as_mut(), &home)?;
//         }
//         Ok(())
//     }
// }
