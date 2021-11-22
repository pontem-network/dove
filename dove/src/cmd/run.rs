use std::fs;
use std::path::{Path, PathBuf};
use structopt::StructOpt;
use crate::cmd::build::run_internal_build;
use anyhow::{Error, Result};
use bytecode_source_map::source_map::SourceMap;
use move_cli::{Move, run_cli};
use move_cli::sandbox::cli::SandboxCommand;
use move_core_types::account_address::AccountAddress;
use move_core_types::errmap::ErrorMapping;
use move_package::compilation::package_layout::CompiledPackageLayout;
use lang::bytecode::info::BytecodeInfo;
use move_cli::Command;
use move_lang::shared::{NumberFormat, NumericalAddress};
use crate::cmd::Cmd;
use crate::context::Context;
use crate::tx::cmd::CallDeclarationCmd;
use crate::tx::fn_call::Config;
use crate::tx::make_transaction;
use crate::tx::model::EnrichedTransaction;

/// Run move script
#[derive(StructOpt, Debug)]
#[structopt(setting(structopt::clap::AppSettings::ColoredHelp))]
#[structopt(usage = "dove run [call] [OPTIONS]\n
    Examples:
    $ dove run 'script_name([10,10], true, 68656c6c6f776f726c64, 100, 0x1)'
    $ dove run script_name -a [10,10] true 68656c6c6f776f726c64 100 0x1
    $ dove run 'script_name()'
    $ dove run 'Module::function()'
    $ dove run '0x1::Module::function()'
")]
pub struct Run {
    #[structopt(flatten)]
    call: CallDeclarationCmd,
    #[structopt(long, hidden = true)]
    color: Option<String>,
    /// If set, the effects of executing `script_file` (i.e., published, updated, and
    /// deleted resources) will NOT be committed to disk.
    #[structopt(long = "dry-run", short = "d")]
    dry_run: bool,
    /// Gas budget.
    #[structopt(long = "gas_budget", short = "g")]
    gas_budget: Option<u64>,
}

impl Cmd for Run {
    fn apply(&mut self, ctx: &mut Context) -> Result<()>
        where
            Self: Sized,
    {
        run_internal_build(ctx)?;
        let tx = make_transaction(ctx, self.call.take(), Config::for_run())?;
        match tx {
            EnrichedTransaction::Local { bi, args, type_tag, func_name, signers } => {
                let natives =
                    move_stdlib::natives::all_natives(AccountAddress::from_hex_literal("0x1")?);
                let script_file = resolve_script_name(&bi)?;
                let error_descriptions: ErrorMapping =
                    bcs::from_bytes(move_stdlib::error_descriptions())?;

                let args = args.into_iter().map(|arg| arg.try_into()).collect::<Result<_, Error>>()?;
                let cmd = SandboxCommand::Run {
                    script_file,
                    script_name: func_name,
                    signers: signers.iter().map(|addr| addr.to_hex_literal()).collect(),
                    args,
                    type_args: type_tag,
                    gas_budget: self.gas_budget,
                    dry_run: self.dry_run,
                };

                let named_addresses = ctx.named_address()
                    .into_iter()
                    .filter_map(|(k, v)| v.map(|v| (k, v)))
                    .map(|(k, v)|(k.to_string(), NumericalAddress::new(v.into_bytes(), NumberFormat::Hex)))
                    .collect();

                let move_args = Move {
                    named_addresses,
                    storage_dir: ctx.move_args.storage_dir.clone(),
                    build_dir: ctx.move_args.build_dir.clone(),
                    mode: ctx.move_args.mode,
                    dialect: ctx.move_args.dialect,
                    verbose: ctx.move_args.verbose
                };
                run_cli(
                    natives,
                    &error_descriptions,
                    &move_args,
                    &Command::Sandbox { cmd },
                )
            }
            EnrichedTransaction::Global { .. } => unreachable!(),
        }
    }
}

fn resolve_script_name(bi: &BytecodeInfo) -> Result<PathBuf> {
    let path: &Path = bi.bytecode_ref().0.as_ref();
    let path = path.to_path_buf();
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
    let source_map = bcs::from_bytes(&fs::read(source_map)?)?;
    find_loc(&source_map)
}

fn find_loc(source_map: &SourceMap) -> Result<PathBuf> {
    source_map
        .function_map
        .iter()
        .find_map(|(_, v)| Some(PathBuf::from(v.decl_location.file().as_str())))
        .or_else(|| {
            source_map
                .struct_map
                .iter()
                .find_map(|(_, v)| Some(PathBuf::from(v.decl_location.file().as_str())))
        })
        .ok_or_else(|| anyhow!("Script location not found"))
}
