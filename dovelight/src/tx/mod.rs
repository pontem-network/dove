use std::str::FromStr;
use anyhow::{Error, bail};
use wasm_bindgen::JsValue;
use move_core_types::account_address::AccountAddress;
use lang::compiler::dialects::{Dialect, DialectName};
use dove_lib::tx::parser::{parse_call, Call};
use crate::Units;
use crate::compiler::source_map::SourceMap;
use crate::tx::fn_call::make_script_call;

pub mod fn_call;
pub mod resolver;

/// Creating a transaction
pub fn make_transaction(
    // Node address. http://localhost:9933/
    chain_api: &str,
    // Project data
    proejct_data: ProjectData,
    // Call String. NAME_SCRIPT<U8, BOOL>(1,[2,3])
    call: &str,
    // At what index is the script located
    file: Option<String>,
) -> Result<Units, Error> {
    let _addr = &proejct_data.address;
    let call = parse_call(
        proejct_data.dialect.as_ref(),
        &proejct_data.address.to_string(),
        call,
    )?;

    let _tx = match call {
        Call::Function {
            address: _,
            module: _,
            func: _,
            type_tag: _,
            args: _,
        } => {
            bail!("@todo Call::Function");
        }
        Call::Script {
            name,
            type_tag,
            args,
        } => make_script_call(chain_api, &proejct_data, name, type_tag, args, file),
    }?;

    bail!("@todo return tx 2")
}

pub struct ProjectData {
    pub dialect: Box<dyn Dialect>,
    pub address: AccountAddress,
    pub source_map: SourceMap,
}

impl ProjectData {
    pub fn from(dialect: &str, addr: &str, source_map: JsValue) -> Result<ProjectData, Error> {
        Ok(ProjectData {
            dialect: DialectName::from_str(&dialect)?.get_dialect(),
            source_map: source_map.into_serde()?,
            address: AccountAddress::from_hex_literal(addr)?,
        })
    }
}
