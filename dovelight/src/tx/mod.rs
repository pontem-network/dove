use anyhow::Error;
use move_core_types::account_address::AccountAddress;
use lang::compiler::dialects::Dialect;
use lang::tx::parser::{parse_call, Call};
use lang::tx::fn_call::Config;
use lang::tx::model::EnrichedTransaction;
use crate::Unit;
use crate::compiler::source_map::SourceMap;
use crate::tx::fn_call::{make_script_call, make_function_call};

pub mod fn_call;
pub mod resolver;

/// Creating a transaction
pub fn make_transaction(
    // Project data
    project_data: ProjectData,
    // Call String. NAME_SCRIPT<U8, BOOL>(1,[2,3])
    call: &str,
    // At what index is the script located
    file: Option<String>,
) -> Result<Unit, Error> {
    let call = parse_call(
        project_data.dialect.as_ref(),
        &project_data.account_address.to_string(),
        call,
    )?;

    let etx = match call {
        Call::Function {
            address,
            module,
            func,
            type_tag,
            args,
        } => make_function_call(
            &project_data,
            address.unwrap_or(project_data.account_address.clone()),
            module,
            func,
            type_tag,
            args,
            file,
        ),
        Call::Script {
            name,
            type_tag,
            args,
        } => make_script_call(&project_data, name, type_tag, args, file),
    }?;

    match etx {
        EnrichedTransaction::Local { .. } => unreachable!(),
        EnrichedTransaction::Global { tx, name } => Ok(Unit {
            name,
            bytecode: bcs::to_bytes(&tx)?,
        }),
    }
}

pub struct ProjectData {
    pub dialect: Box<dyn Dialect>,
    pub account_address: AccountAddress,
    pub source_map: SourceMap,
    pub chain_api: String,
    pub cfg: Config,
}
