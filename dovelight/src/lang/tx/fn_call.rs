use anyhow::{Error, bail};
use itertools::{Itertools, Either};
use move_core_types::language_storage::TypeTag;
use move_core_types::identifier::Identifier;
use move_core_types::account_address::AccountAddress;
use move_lang::compiled_unit::CompiledUnit;
use lang::tx::fn_call::{select_function, prepare_function_signature};
use lang::tx::model::{Signers, Transaction, Call, EnrichedTransaction};
use crate::lang::compiler::build_base;
use crate::lang::compiler::interact::CompilerInteract;
use crate::lang::tx::Context;
use crate::lang::tx::resolver::{find_script, find_module_function};
use crate::lang::compiler::source_map::SourceMap;

pub(crate) fn make_script_call(
    // Project Code
    source_map: &SourceMap,
    // Launch data
    context: &Context,
    // script name
    name: Identifier,
    // Generics for script
    type_tag: Vec<TypeTag>,
    // arguments for the function
    args: Vec<String>,
    // At what index is the script located
    index_in_source_map: Option<String>,
) -> Result<EnrichedTransaction, Error> {
    let scripts = find_script(source_map, context, &name, index_in_source_map)?;

    let (finded_index, meta) = select_function(
        scripts.clone(),
        context.account_address(),
        &type_tag,
        &args,
        &context.cfg,
    )?;
    let (signers, args) = prepare_function_signature(
        &meta.parameters,
        &args,
        !context.cfg.deny_signers_definition,
        context.account_address(),
    )?;
    // Creating transaction
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

    // Building project
    let sender = context.account_address_as_string();
    let resolver = context.resolver()?;
    let mut interact = CompilerInteract::new(
        context.dialect.as_ref(),
        &sender,
        source_map.clone(),
        resolver,
    );
    let (modules, script): (Vec<_>, Vec<_>) = build_base(&mut interact, source_map.clone())?
        .into_iter()
        .filter_map(|unit| match unit {
            CompiledUnit::Module { module, .. } => Some(Either::Left(module)),
            CompiledUnit::Script {
                loc, key, script, ..
            } => {
                if loc.file == finded_index && key == name.as_str() {
                    Some(Either::Right(script))
                } else {
                    None
                }
            }
        })
        .partition_map(|u| u);

    if script.is_empty() {
        bail!("The script {:?} could not be compiled", finded_index);
    }

    let mut buff = Vec::new();
    script[0].serialize(&mut buff)?;
    match &mut tx.inner_mut().call {
        Call::Script { code, .. } => *code = buff,
        Call::ScriptFunction { .. } => {
            // no-op
        }
    }

    Ok(if context.cfg.exe_context {
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
pub fn make_function_call(
    // Project Code
    source_map: &SourceMap,
    // Launch data
    conext: &Context,
    // Module address
    module_address: AccountAddress,
    // module name
    module_name: Identifier,
    // function name
    function_name: Identifier,
    // Generics for function
    type_tag: Vec<TypeTag>,
    // arguments for function
    args: Vec<String>,
    // At what index is the script located
    source_index: Option<String>,
) -> Result<EnrichedTransaction, Error> {
    let functions = find_module_function(
        source_map,
        conext,
        &module_address,
        &module_name,
        &function_name,
        source_index.as_ref(),
    )?;
    let account_address = conext.account_address.clone();
    let (_, meta) = select_function(functions, account_address, &type_tag, &args, &conext.cfg)?;

    let (signers, args) = prepare_function_signature(
        &meta.parameters,
        &args,
        !conext.cfg.deny_signers_definition,
        account_address,
    )?;
    let tx_name = format!("{}_{}", module_name, function_name);
    let (_signers, tx) = match signers {
        Signers::Explicit(signers) => (
            signers,
            Transaction::new_func_tx(
                vec![],
                module_address,
                module_name,
                function_name,
                args,
                type_tag,
            )?,
        ),
        Signers::Implicit(signers) => (
            vec![],
            Transaction::new_func_tx(
                signers,
                module_address,
                module_name,
                function_name,
                args,
                type_tag,
            )?,
        ),
    };

    Ok(if conext.cfg.exe_context {
        anyhow::bail!("@todo make_function_call exe_context")
    } else {
        EnrichedTransaction::Global { tx, name: tx_name }
    })
}
