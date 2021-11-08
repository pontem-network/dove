use std::str::FromStr;
use anyhow::Error;
use move_core_types::account_address::AccountAddress;
use move_lang::errors::{report_errors_to_color_buffer, Errors};
use lang::compiler::dialects::{Dialect, DialectName};
use lang::tx::parser::{parse_call, Call};
use lang::tx::fn_call::Config;
use lang::tx::model::EnrichedTransaction;
use crate::api::Unit;
use crate::lang::compiler::source_map::SourceMap;
use crate::lang::tx::fn_call::{make_script_call, make_function_call};
use crate::loader::Loader;
use crate::lang::deps::resolver::DependencyResolver;
use crate::storage::EnvStorage;

pub mod fn_call;
pub mod resolver;

/// Creating a transaction
pub fn make_transaction(
    // Project Code
    source_map: &SourceMap,
    // Launch data
    context: &Context,
    // Call String. NAME_SCRIPT<U8, BOOL>(1,[2,3])
    call: &str,
    // At what index is the script located
    file: Option<String>,
) -> Result<Unit, Error> {
    let call = parse_call(
        context.dialect.as_ref(),
        &context.account_address.to_string(),
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
            source_map,
            &context,
            address.unwrap_or(context.account_address.clone()),
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
        } => make_script_call(source_map, &context, name, type_tag, args, file),
    }?;

    match etx {
        EnrichedTransaction::Local { .. } => unreachable!(),
        EnrichedTransaction::Global { tx, name } => Ok(Unit {
            name,
            bytecode: bcs::to_bytes(&tx)?,
        }),
    }
}

pub struct Context {
    pub dialect: Box<dyn Dialect>,
    pub account_address: AccountAddress,
    pub chain_api: String,
    pub cfg: Config,
}

impl Context {
    pub fn web_storage() -> Result<EnvStorage, Error> {
        EnvStorage::new_in_family("dove_cache_")
    }
    pub fn loader(&self) -> Loader {
        Loader::new(self.chain_api.clone())
    }
    pub fn account_address(&self) -> AccountAddress {
        self.account_address.clone()
    }
    pub fn account_address_as_string(&self) -> String {
        self.account_address.to_string()
    }
    pub fn resolver(&self) -> Result<DependencyResolver<Loader, EnvStorage>, Error> {
        Ok(DependencyResolver::new(
            self.dialect.as_ref(),
            self.loader(),
            Self::web_storage()?,
        ))
    }
}
impl Default for Context {
    fn default() -> Self {
        Context {
            dialect: DialectName::from_str("pont").unwrap().get_dialect(),
            account_address: AccountAddress::from_hex_literal("0x1").unwrap(),
            chain_api: "http://localhost:9933/".to_string(),
            cfg: Config::for_run(),
        }
    }
}

pub fn report_errors(source_map: &SourceMap, errors: Errors) -> Error {
    let error = report_errors_to_color_buffer(source_map.to_files_source_text(), errors);
    let err = String::from_utf8_lossy(&error).to_string();
    Error::msg(err)
}
