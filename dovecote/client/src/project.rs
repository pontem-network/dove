use wasm_bindgen::JsValue;
use crate::context::api_url;
use crate::api;
use proto::Empty;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn project_list() -> Result<JsValue, JsValue> {
    api(proto::project_list(&api_url(), &Empty).await)
}

#[wasm_bindgen]
pub async fn project_info(id: String) -> Result<JsValue, JsValue> {
    api(proto::project_info(&api_url(), &id).await)
}

#[wasm_bindgen]
pub async fn sync_project(id: String) -> Result<JsValue, JsValue> {
    api(proto::sync_project(&api_url(), &id).await)
}
