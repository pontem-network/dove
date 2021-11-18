use anyhow::{Error, anyhow};
use wasm_bindgen::JsValue;
use serde::de::DeserializeOwned;

pub mod http;
pub mod log;
pub mod store;

fn js_result<OK: DeserializeOwned>(val: JsValue) -> Result<OK, Error> {
    if val.is_null() {
        return Err(anyhow!("Null pointer error"));
    }
    val.into_serde::<Result<OK, String>>()?.map_err(Error::msg)
}
