use wasm_bindgen::JsValue;
use anyhow::{Error, anyhow};
use wasm_bindgen::prelude::*;
use web_sys::{Window, Document};

pub fn api_url() -> String {
    format!(
        "{}/api/rpc",
        web_sys::window()
            .and_then(|win| win.location().origin().ok())
            .unwrap_or_default()
    )
}

pub fn window() -> Result<Window, Error> {
    web_sys::window().ok_or_else(|| anyhow!("no global `window` exists"))
}

pub fn document(win: &Window) -> Result<Document, Error> {
    win.document()
        .ok_or_else(|| anyhow!("should have a document on window"))
}
