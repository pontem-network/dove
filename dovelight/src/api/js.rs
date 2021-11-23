use std::fmt::Display;
use anyhow::Result;
use wasm_bindgen::prelude::*;
use crate::lang::compiler::source_map::SourceMap;
use crate::api;
use crate::loader::get_request_resource_by_xpath;

#[wasm_bindgen]
pub fn build(
    chain_api: String,
    source_map: JsValue,
    dialect: String,
    sender: String,
) -> Result<JsValue, JsValue> {
    let source_map = source_map.into_serde().map_err(js_err)?;
    let units = api::build(chain_api, source_map, dialect, sender).map_err(js_err)?;
    Ok(JsValue::from_serde(&units).map_err(js_err)?)
}

#[wasm_bindgen]
pub fn module_abi(
    chain_api: String,
    dialect: String,
    address: String,
    module_name: String,
) -> Result<JsValue, JsValue> {
    api::module_abi(chain_api, dialect, address, module_name)
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
    let source_map = source_map.into_serde().map_err(js_err)?;
    let abi = api::make_abi(chain_api, source_map, dialect, address).map_err(js_err)?;
    JsValue::from_serde(&abi).map_err(js_err)
}

/// Creating a transaction
#[wasm_bindgen]
pub fn tx(
    // Node address. http://localhost:9933/
    chain_api: String,
    // Project code. Scripts and modules
    source_map: JsValue,
    // Dialect of the project. diem, dfinance, pont
    dialect: String,
    // Call String. NAME_SCRIPT<U8, BOOL>(1,[2,3])
    call: String,
) -> Result<JsValue, JsValue> {
    let source_map: SourceMap = source_map.into_serde().map_err(js_err)?;
    let unit = api::tx(chain_api, source_map, dialect, call).map_err(js_err)?;
    JsValue::from_serde(&unit).map_err(js_err)
}

/// Creating a transaction
#[wasm_bindgen]
pub fn request_resource_json(
    // account address
    // example: 0x1
    account: String,
    // xpath
    // example: 0x1::Account::Balance<0x1::Coins::ETH>
    xpath: String,
) -> Result<JsValue, JsValue> {
    get_request_resource_by_xpath(&account, &xpath)
        .map_err(js_err)
        .and_then(|value| JsValue::from_serde(&value).map_err(js_err))
}

pub fn js_err<D: Display>(err: D) -> JsValue {
    JsValue::from_str(&err.to_string())
}
