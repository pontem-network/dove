use wasm_bindgen::JsValue;
use crate::context::api_url;
use crate::api;



#[wasm_bindgen]
pub async fn get_file(id: String) -> Result<JsValue, JsValue> {
    api(proto::get_file(&api_url(), id).await)
}

