use std::fmt::Display;

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use storage::web::WebStorage;

use crate::compiler::loader::Loader;

mod compiler;
pub mod deps;
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

#[derive(Serialize, Deserialize)]
pub struct Units {
    pub units: Vec<Unit>,
}

#[derive(Serialize, Deserialize)]
pub struct Unit {
    pub name: String,
    pub bytecode: Vec<u8>,
}

pub fn js_err<D: Display>(err: D) -> JsValue {
    JsValue::from_str(&err.to_string())
}
