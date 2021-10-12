use anyhow::{Error, bail};
use itertools::{Itertools, Either};
use move_core_types::language_storage::TypeTag;
use move_core_types::identifier::Identifier;
use move_lang::compiled_unit::CompiledUnit;
use lang::tx::fn_call::{select_function, prepare_function_signature, Config};
use lang::tx::model::{Signers, Transaction, Call, EnrichedTransaction};
use crate::compiler::build_base;
use crate::compiler::interact::CompilerInteract;
use crate::storage::web::WebStorage;
use crate::loader::Loader;
use crate::deps::resolver::DependencyResolver;
use crate::tx::ProjectData;
use crate::tx::resolver::find_script;

/// Create a transaction from a script
pub(crate) fn make_script_call(
    // Node address. http://localhost:9933/
    chain_api: &str,
    // Project data
    project_data: &ProjectData,
    // script name
    name: Identifier,
    // Generics for script
    type_tag: Vec<TypeTag>,
    // Script name
    args: Vec<String>,
    // At what index is the script located
    file: Option<String>,
    // config
    cfg: Config,
) -> Result<EnrichedTransaction, Error> {
    let store = WebStorage::new_in_family("dove_cache_")?;
    let loader = Loader::new(chain_api.to_string());

    let account_address = project_data.address.clone();
    let scripts = find_script(project_data, &name, file)?;

    let (finded_index, meta) =
        select_function(scripts.clone(), account_address, &type_tag, &args, &cfg)?;
    let (signers, args) = prepare_function_signature(
        &meta.parameters,
        &args,
        !cfg.deny_signers_definition,
        account_address.clone(),
    )?;
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

    // @todo Used to run
    // let (_, interface) = ctx.build_index()?;

    let sender = account_address.to_string();
    let resolver = DependencyResolver::new(project_data.dialect.as_ref(), loader, store);
    let mut interact = CompilerInteract::new(
        project_data.dialect.as_ref(),
        &sender,
        project_data.source_map.clone(),
        resolver,
    );

    let (modules, script): (Vec<_>, Vec<_>) =
        build_base(&mut interact, project_data.source_map.clone())?
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

    Ok(if cfg.exe_context {
        // @todo Used to run
        // modules.extend(interface.load_mv()?);
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
