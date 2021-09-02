#[macro_use]
extern crate serde;

use wasm_bindgen::prelude::*;

use proto;
use proto::project::{ProjectList, ProjectShortInfo};
use crate::state::{set_base_url, api_url};
use proto::Empty;
use std::error::Error;
use std::fmt::Display;

mod state;

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
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");
    set_base_url(
        &window
            .location()
            .origin()
            .expect("no location origin exists"),
    );

    let val = document.create_element("p")?;
    val.set_inner_html("Hello from Rust!");
    body.append_child(&val)?;

    Ok(())
}

#[wasm_bindgen]
pub async fn project_list() -> Result<JsValue, JsValue> {
    JsValue::from_serde(
        &proto::project_list(api_url(), Empty)
            .await
            .map_err(js_err)?,
    )
    .map_err(js_err)
}

// #[test]
// pub fn test() {
//     #[derive(Serialize, Deserialize, Debug)]
//     struct F {
//         #[serde(with = "serde_bytes")]
//         be: Vec<u8>,
//    }
//     serde_bytes::serialize()
//     dbg!(serde_json::to_string(&F { be: vec![0,1,2,3,2,3,64,4,23,35,35,35,35,35,35,35,1,4,8,5] }).unwrap());
// }

fn js_err<E: Display>(err: E) -> JsValue {
    JsValue::from_str(&err.to_string())
}
