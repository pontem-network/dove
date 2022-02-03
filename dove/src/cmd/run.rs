use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use structopt::StructOpt;
use anyhow::{Error, Result};
use move_bytecode_source_map::source_map::SourceMap;
use lang::bytecode::info::BytecodeInfo;
use move_cli::{DEFAULT_STORAGE_DIR, Move, run_cli};
use move_cli::sandbox::cli::SandboxCommand;
use move_cli::Command;
use move_command_line_common::files::FileHash;

use move_package::BuildConfig;
use move_package::compilation::package_layout::CompiledPackageLayout;
use crate::cmd::deploy::run_dove_package_build;
use crate::context::Context;

use crate::call::cmd::CallDeclarationCmd;
use crate::call::fn_call::Config;
use crate::call::make_transaction;
use crate::call::model::EnrichedTransaction;

#[derive(StructOpt, Debug)]
#[structopt(setting(structopt::clap::AppSettings::ColoredHelp))]
#[structopt(usage = "dove run [call] [OPTIONS]\n
    Examples:
    $ dove run 'script_name([10,10], true, 68656c6c6f776f726c64, 100, 0x1)'
    $ dove run script_name --parameters [10,10] true 68656c6c6f776f726c64 100 0x1
    $ dove run 'script_name()'
    $ dove run 'Module::function()'
    $ dove run '0x1::Module::function()'
")]
pub struct Run {
    #[structopt(flatten)]
    call: CallDeclarationCmd,

    /// If set, the effects of executing `script_file` (i.e., published, updated, and
    /// deleted resources) will NOT be committed to disk.
    #[structopt(long = "dry-run")]
    dry_run: bool,

    #[structopt(long = "gas_budget", short = "g")]
    gas_budget: Option<u64>,
}

impl Run {
    pub fn apply(&mut self, ctx: &mut Context) -> Result<()> {
        run_dove_package_build(ctx)?;
        let tx = make_transaction(ctx, self.call.take(), Config::for_run())?;
        match tx {
            EnrichedTransaction::Local {
                bi,
                args,
                type_tag,
                func_name,
                signers,
            } => {
                let script_file = resolve_script_name(&bi)?;

                let args = args
                    .into_iter()
                    .map(|arg| arg.try_into())
                    .collect::<Result<_, Error>>()?;
                let cmd = SandboxCommand::Run {
                    script_file,
                    script_name: func_name,
                    signers: signers.iter().map(|addr| addr.to_hex_literal()).collect(),
                    args,
                    type_args: type_tag,
                    gas_budget: self.gas_budget,
                    dry_run: self.dry_run,
                };

                let named_addresses = ctx
                    .address_declarations()
                    .into_iter()
                    .filter_map(|(k, v)| v.map(|v| (k, v)))
                    .map(|(k, v)| (k.to_string(), v))
                    .collect();

                let move_args = Move {
                    package_path: ctx.project_root_dir.clone(),
                    verbose: ctx.move_args.verbose,
                    build_config: BuildConfig {
                        dev_mode: true,
                        test_mode: true,
                        generate_docs: false,
                        generate_abis: false,
                        install_dir: None,
                        force_recompilation: false,
                        additional_named_addresses: named_addresses,
                    },
                };
                run_cli(
                    ctx.native_functions.clone(),
                    &ctx.cost_table,
                    &ctx.error_descriptions,
                    &move_args,
                    &Command::Sandbox {
                        storage_dir: PathBuf::from(DEFAULT_STORAGE_DIR),
                        cmd,
                    },
                )
            }
            EnrichedTransaction::Global { .. } => unreachable!(),
        }
    }
}

fn resolve_script_name(bi: &BytecodeInfo) -> Result<PathBuf> {
    let path = PathBuf::from(&bi.bytecode_ref().0);
    let name = path
        .file_name()
        .map(|name| name.to_string_lossy().to_string())
        .ok_or_else(|| anyhow!("Failed to get file name:{:?}", path))?;
    let package = path
        .parent()
        .and_then(|p| p.parent())
        .ok_or_else(|| anyhow!("Failed to get package dir:{:?}", path))?;
    let mut source_map = package
        .join(CompiledPackageLayout::SourceMaps.path())
        .join(name);
    source_map.set_extension("mvsm");
    let source_map: SourceMap = bcs::from_bytes(&fs::read(source_map)?)?;

    let project_path = path
        .parent()
        .and_then(|parent| parent.parent())
        .and_then(|parent| parent.parent())
        .and_then(|parent| parent.parent())
        .ok_or_else(|| anyhow!("Failed to get project dir: {:?}", path))?;

    find_loc(project_path, &source_map)
}

fn find_loc(project_path: &Path, source_map: &SourceMap) -> Result<PathBuf> {
    let map = find_move_files_in_project(project_path);
    let hash = source_map.definition_location.file_hash();

    map.get(&hash)
        .cloned()
        .ok_or_else(|| anyhow!("Script location not found"))
}

/// Search "move" files in the project
/// Search is carried out in the directories: scripts, sources
///
fn find_move_files_in_project(project_path: &Path) -> HashMap<FileHash, PathBuf> {
    ["scripts", "sources"]
        .iter()
        .filter_map(|dir| find_move_files_in_dir(&project_path.join(dir)).ok())
        .flatten()
        .collect()
}

/// Search "move" files in directory
/// Recursive search
fn find_move_files_in_dir(dir_path: &Path) -> Result<Vec<(FileHash, PathBuf)>> {
    let list = fs::read_dir(dir_path)?
        .filter_map(|dir| dir.ok())
        .map(|r| r.path())
        .filter_map(|path| {
            if path.is_dir() {
                find_move_files_in_dir(&path).ok()
            } else if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("move")
            {
                let file = fs::read_to_string(&path).ok()?;
                let hash = FileHash::new(&file);
                Some(vec![(hash, path)])
            } else {
                None
            }
        })
        .flatten()
        .collect();
    Ok(list)
}
