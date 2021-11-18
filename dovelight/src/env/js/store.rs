use wasm_bindgen::prelude::*;

use anyhow::Error;
use super::js_result;

pub fn store(key: String, val: Vec<u8>) -> Result<(), Error> {
    js_result(set_item(&key, &hex::encode(&val)))
}

pub fn load(key: String) -> Result<Option<Vec<u8>>, Error> {
    let item = get_item(&key);
    if item.is_null() {
        return Ok(None);
    }
    let item: String = js_result(item)?;
    Ok(Some(hex::decode(item)?))
}

pub fn drop(key: String) -> Result<(), Error> {
    js_result(remove_item(&key))
}

#[wasm_bindgen(module="wasm_resolver.js")]
extern "C" {
    #[wasm_bindgen(js_namespace = resolver)]
    pub fn set_item(key: &str, val: &str) -> JsValue;

    #[wasm_bindgen(js_namespace = resolver)]
    pub fn get_item(key: &str) -> JsValue;

    #[wasm_bindgen(js_namespace = resolver)]
    pub fn remove_item(key: &str) -> JsValue;
}
