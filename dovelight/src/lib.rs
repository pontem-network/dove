use std::fmt::Display;
use std::str::FromStr;

use move_core_types::account_address::AccountAddress;
use move_core_types::identifier::Identifier;
use move_core_types::language_storage::ModuleId;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use lang::compiler::dialects::DialectName;
use loader::Loader;
use storage::web::WebStorage;

use crate::abi::make_module_abi;
use crate::deps::index::id_to_str;
use crate::deps::resolver::DependencyResolver;

pub mod abi;
mod compiler;
pub mod deps;
pub mod loader;
pub mod storage;

#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => (crate::log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[wasm_bindgen]
pub fn build(
    chain_api: String,
    source_map: JsValue,
    dialect: String,
    sender: String,
) -> Result<JsValue, JsValue> {
    let cache = WebStorage::new_in_family("dove_cache_").map_err(js_err)?;
    let loader = Loader::new(chain_api);

    let source_map = source_map.into_serde().map_err(js_err)?;
    let units = compiler::build(loader, cache, source_map, &dialect, &sender)
        .map_err(js_err)?
        .into_iter()
        .map(|(name, bytecode)| Unit { name, bytecode })
        .collect();
    Ok(JsValue::from_serde(&Units { units }).map_err(js_err)?)
}

#[wasm_bindgen]
pub fn module_abi(
    chain_api: String,
    dialect: String,
    address: String,
    module_name: String,
) -> Result<JsValue, JsValue> {
    let store = WebStorage::new_in_family("dove_cache_").map_err(js_err)?;
    let loader = Loader::new(chain_api);
    let dialect = DialectName::from_str(&dialect)
        .map_err(js_err)?
        .get_dialect();
    let resolver = DependencyResolver::new(dialect.as_ref(), loader, store);
    let module_id = ModuleId::new(
        AccountAddress::from_hex_literal(&address).map_err(js_err)?,
        Identifier::new(module_name).map_err(js_err)?,
    );
    let bytecode = resolver
        .load_bytecode(&id_to_str(&module_id))
        .map_err(js_err)?;
    make_module_abi(&bytecode)
        .map_err(js_err)
        .and_then(|abi| JsValue::from_serde(&abi).map_err(js_err))
}

#[wasm_bindgen]
pub fn make_abi(
    chain_api: String,
    source_map: JsValue,
    dialect: String,
    address: String,
) -> Result<JsValue, JsValue> {
    let units: Units = build(chain_api, source_map, dialect, address)?
        .into_serde()
        .map_err(js_err)?;
    JsValue::from_serde(
        &units
            .units
            .into_iter()
            .map(|u| make_module_abi(&u.bytecode))
            .collect::<Result<Vec<_>, _>>()
            .map_err(js_err)?,
    )
    .map_err(js_err)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Units {
    pub units: Vec<Unit>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Unit {
    pub name: String,
    pub bytecode: Vec<u8>,
}

pub fn js_err<D: Display>(err: D) -> JsValue {
    JsValue::from_str(&err.to_string())
}
