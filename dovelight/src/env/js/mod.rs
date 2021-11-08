use anyhow::Error;
use wasm_bindgen::JsValue;

pub mod http;
pub mod log;
pub mod store;

fn js_err(val: JsValue) -> Error {
    anyhow::anyhow!("{:?}", val)
}
