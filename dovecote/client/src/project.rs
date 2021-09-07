use wasm_bindgen::JsValue;
use crate::context::api_url;
use crate::{api, js_err};
use proto::Empty;
use proto::project::ID;
use wasm_bindgen::prelude::*;
use crate::console_log;

#[wasm_bindgen]
pub async fn project_list() -> Result<JsValue, JsValue> {
    api(proto::project_list(&api_url(), Empty).await)
}

#[wasm_bindgen]
pub async fn project_info(id: String) -> Result<JsValue, JsValue> {
    api(proto::project_info(&api_url(), id).await)
}

#[wasm_bindgen]
pub async fn get_file(id: String) -> Result<JsValue, JsValue> {
    api(proto::get_file(&api_url(), id).await)
}

