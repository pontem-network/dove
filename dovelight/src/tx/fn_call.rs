use anyhow::{Error, bail};
use move_core_types::language_storage::TypeTag;
use move_core_types::identifier::Identifier;
use dove_lib::tx::fn_call::{select_function, prepare_function_signature, Config};
use dove_lib::tx::model::{Signers, Transaction};
use crate::tx::ProjectData;
use crate::tx::resolver::find_script;
use crate::storage::web::WebStorage;
use crate::loader::Loader;

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
) -> Result<(), Error> {
    let _cache = WebStorage::new_in_family("dove_cache_")?;
    let _loader = Loader::new(chain_api.to_string());

    let config_for_tx = Config::for_tx();
    let account_address = project_data.address.clone();
    let scripts = find_script(project_data, &name, file)?;

    let (_path, meta) =
        select_function(scripts, account_address, &type_tag, &args, &config_for_tx)?;
    let (signers, args) = prepare_function_signature(
        &meta.parameters,
        &args,
        !config_for_tx.deny_signers_definition,
        account_address,
    )?;
    let (_signers, mut _tx) = match signers {
        Signers::Explicit(signers) => (
            signers,
            Transaction::new_script_tx(vec![], vec![], args, type_tag)?,
        ),
        Signers::Implicit(signers) => (
            vec![],
            Transaction::new_script_tx(signers, vec![], args, type_tag)?,
        ),
    };

    // @todo remove?
    // let (_, interface) = ctx.build_index()?;

    // let _units = compiler::build(
    //     loader,
    //     cache,
    //     project_data.source_map.clone(),
    //     &project_data.dialect.name().to_string(),
    //     &project_data.address.to_string(),
    // )?;

    bail!("@todo make_script_call")

    //
    // let (mut modules, script): (Vec<_>, Vec<_>) = move_build(
    //     ctx,
    //     &[
    //         path.to_string_lossy().to_string(),
    //         ctx.str_path_for(&ctx.manifest.layout.modules_dir)?,
    //     ],
    //     &[interface.dir.to_string_lossy().into_owned()],
    // )?
    // .into_iter()
    // .filter_map(|u| match u {
    //     CompiledUnit::Module { module, .. } => Some(Either::Left(module)),
    //     CompiledUnit::Script {
    //         loc, key, script, ..
    //     } => {
    //         if loc.file == path.to_string_lossy().as_ref() && key == name.as_str() {
    //             Some(Either::Right(script))
    //         } else {
    //             None
    //         }
    //     }
    // })
    // .partition_map(|u| u);
    // if script.is_empty() {
    //     bail!("The script {:?} could not be compiled", path);
    // }
    //
    // let mut buff = Vec::new();
    // script[0].serialize(&mut buff)?;
    // match &mut tx.inner_mut().call {
    //     Call::Script { code, .. } => *code = buff,
    //     Call::ScriptFunction { .. } => {
    //         // no-op
    //     }
    // }
    //
    // Ok(if cfg.exe_context {
    //     modules.extend(interface.load_mv()?);
    //     EnrichedTransaction::Local {
    //         tx,
    //         signers,
    //         deps: modules,
    //     }
    // } else {
    //     EnrichedTransaction::Global {
    //         tx,
    //         name: name.into_string(),
    //     }
    // })
}
