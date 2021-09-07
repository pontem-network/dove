use wasm_bindgen::prelude::*;

use wasm_bindgen::JsValue;
use crate::context::api_url;
use crate::{api, js_err};
use proto::project::ID;
use proto::Request;
use proto::file::GetFile;

#[wasm_bindgen]
pub async fn open_file( project_id: ID, file_id: ID, container_id: String, config: JsValue) -> Result<(), JsValue> {
    let get_file = GetFile {
        project_id,
        file_id,
    };
    let config = config.into_serde().map_err(js_err)?;
    let file = proto::get_file(&api_url(), get_file).await.map_err(js_err)?;

    Ok(())
}

// #[wasm_bindgen]
// pub async fn open_file() {
//    // api(proto::project_info(&api_url(), id).await)
//     //Ok(JsValue::NULL)
// }

// #[wasm_bindgen]
// pub async fn open_file(
//     // project_id: ID, file_id: ID, container_id: String,
//                        // val: JsValue
// ) -> Result<JsValue, JsValue> {
//     // let get_file = GetFile {
//     //     project_id,
//     //     file_id,
//     // };
//     // let config = val.into_serde().map_err(js_err)?;
//     //
//     // let file = proto::get_file(&api_url(), get_file).await.map_err(js_err)?;
//     Ok(JsValue::NULL)
// }

#[derive(Serialize, Deserialize)]
pub struct RenderConfig {
    pub line_height: Option<u32>,
}

