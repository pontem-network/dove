#[macro_use]
extern crate serde;
use wasm_bindgen::prelude::*;

use proto;
use std::fmt::Display;
use serde::Serialize;

mod code;
mod context;
mod file;
mod html;
mod project;

#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => (crate::log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
    pub fn alert(s: &str);
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

// TODO Remove id. This is bug demo.
#[wasm_bindgen]
pub fn there_be_a_bug() -> Result<JsValue, JsValue> {
    let res = JsValue::from_serde(&Id {
        id: 3000490687877993158,
    })
    .map_err(js_err);
    console_log!("{:?}", res);
    res
}

#[derive(Serialize)]
pub struct Id {
    id: u64,
}
