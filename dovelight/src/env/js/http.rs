use wasm_bindgen::prelude::*;
use anyhow::Error;
use crate::env::http::{Request, Response};
use super::js_result;

pub fn http_request(req: Request) -> Result<Response, Error> {
    let req = JsValue::from_serde(&req)?;
    js_result(send_http_request(req))
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn send_http_request(request: JsValue) -> JsValue;
}
