#[macro_use]
extern crate serde;
use wasm_bindgen::prelude::*;

use std::fmt::Display;
use serde::Serialize;

mod context;
mod file;
mod project;

#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => (crate::log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    Ok(())
}

pub fn api<T: Serialize, E: Display>(result: Result<T, E>) -> Result<JsValue, JsValue> {
    match result {
        Ok(val) => JsValue::from_serde(&val).map_err(js_err),
        Err(err) => Err(js_err(err)),
    }
}

pub fn js_err<E: Display>(err: E) -> JsValue {
    JsValue::from_str(&err.to_string())
}
