#[macro_use]
extern crate serde;

use wasm_bindgen::prelude::*;

use proto;
use crate::context::api_url;
use proto::Empty;
use std::fmt::Display;

mod context;

#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
//     let window = web_sys::window().expect("no global `window` exists");
//     let document = window.document().expect("should have a document on window");
//     let body = document.body().expect("document should have a body");
//     set_base_url(
//         &window
//             .location()
//             .origin()
//             .expect("no location origin exists"),
//     );
//
//     let val = document.create_element("p")?;
//     val.set_inner_html("Hello from Rust!");
//     body.append_child(&val)?;
//
    Ok(())
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub async fn project_list() -> Result<JsValue, JsValue> {
    JsValue::from_serde(
        &proto::project_list(&api_url(), Empty)
            .await
            .map_err(js_err)?,
    )
    .map_err(js_err)
}

fn js_err<E: Display>(err: E) -> JsValue {
    JsValue::from_str(&err.to_string())
}
