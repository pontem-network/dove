use anyhow::Error;
use itertools::{Itertools, Either};
use move_core_types::account_address::AccountAddress;
use move_core_types::identifier::Identifier;
use move_core_types::language_storage::TypeTag;
use move_lang::compiled_unit::CompiledUnit;
use dove_lib::tx::model::{Transaction, Signers, Call};
use dove_lib::tx::fn_call::{prepare_function_signature, select_function, Config};
use crate::tx::resolver::{find_module_function, find_script};
use crate::tx::builder::move_build;
use crate::tx::model::{EnrichedTransaction};
use crate::context::Context;

pub(crate) fn make_script_call(
    ctx: &Context,
    addr: AccountAddress,
    name: Identifier,
    type_tag: Vec<TypeTag>,
    args: Vec<String>,
    file: Option<String>,
    cfg: Config,
) -> Result<EnrichedTransaction, Error> {
    let scripts = find_script(ctx, &name, file)?;

    let (path, meta) = select_function(scripts, addr, &type_tag, &args, &cfg)?;

    let (signers, args) =
        prepare_function_signature(&meta.parameters, &args, !cfg.deny_signers_definition, addr)?;

    let (signers, mut tx) = match signers {
        Signers::Explicit(signers) => (
            signers,
            Transaction::new_script_tx(vec![], vec![], args, type_tag)?,
        ),
        Signers::Implicit(signers) => (
            vec![],
            Transaction::new_script_tx(signers, vec![], args, type_tag)?,
        ),
    };

    let (_, interface) = ctx.build_index()?;

    let (mut modules, script): (Vec<_>, Vec<_>) = move_build(
        ctx,
        &[
            path.to_string_lossy().to_string(),
            ctx.str_path_for(&ctx.manifest.layout.modules_dir)?,
        ],
        &[interface.dir.to_string_lossy().into_owned()],
    )?
    .into_iter()
    .filter_map(|u| match u {
        CompiledUnit::Module { module, .. } => Some(Either::Left(module)),
        CompiledUnit::Script {
            loc, key, script, ..
        } => {
            if loc.file == path.to_string_lossy().as_ref() && key == name.as_str() {
                Some(Either::Right(script))
            } else {
                None
            }
        }
    })
    .partition_map(|u| u);
    if script.is_empty() {
        bail!("The script {:?} could not be compiled", path);
    }

    let mut buff = Vec::new();
    script[0].serialize(&mut buff)?;
    match &mut tx.inner_mut().call {
        Call::Script { code, .. } => *code = buff,
        Call::ScriptFunction { .. } => {
            // no-op
        }
    }

    Ok(if cfg.exe_context {
        modules.extend(interface.load_mv()?);
        EnrichedTransaction::Local {
            tx,
            signers,
            deps: modules,
        }
    } else {
        EnrichedTransaction::Global {
            tx,
            name: name.into_string(),
        }
    })
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn make_function_call(
    ctx: &Context,
    address: AccountAddress,
    module: Identifier,
    func: Identifier,
    type_tag: Vec<TypeTag>,
    args: Vec<String>,
    file: Option<String>,
    cfg: Config,
) -> Result<EnrichedTransaction, Error> {
    let functions =
        find_module_function(ctx, &address, &module, &func, &file, cfg.script_func_only)?;

    let addr = ctx.account_address()?;
    let (_, meta) = select_function(functions, addr, &type_tag, &args, &cfg)?;

    let (signers, args) =
        prepare_function_signature(&meta.parameters, &args, !cfg.deny_signers_definition, addr)?;

    let tx_name = format!("{}_{}", module, func);
    let (signers, tx) = match signers {
        Signers::Explicit(signers) => (
            signers,
            Transaction::new_func_tx(vec![], address, module, func, args, type_tag)?,
        ),
        Signers::Implicit(signers) => (
            vec![],
            Transaction::new_func_tx(signers, address, module, func, args, type_tag)?,
        ),
    };

    Ok(if cfg.exe_context {
        let modules_dir = ctx.str_path_for(&ctx.manifest.layout.modules_dir)?;

        let (_, interface) = ctx.build_index()?;
        let mut deps = move_build(
            ctx,
            &[modules_dir],
            &[interface.dir.to_string_lossy().into_owned()],
        )?
        .into_iter()
        .filter_map(|m| match m {
            CompiledUnit::Module { module, .. } => Some(module),
            CompiledUnit::Script { .. } => None,
        })
        .collect::<Vec<_>>();
        deps.extend(interface.load_mv()?);

        EnrichedTransaction::Local { tx, signers, deps }
    } else {
        EnrichedTransaction::Global { tx, name: tx_name }
    })
}
