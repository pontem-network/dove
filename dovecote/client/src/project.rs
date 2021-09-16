use wasm_bindgen::JsValue;
use crate::context::api_url;
use crate::api;
use proto::Empty;
use proto::project::{ActionType, ProjectActionRequest};
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

#[wasm_bindgen]
pub async fn project_build(project_id: String) -> Result<JsValue, JsValue> {
    let action = ProjectActionRequest {
        project_id,
        action: ActionType::Build,
    };
    api(proto::project_action(&api_url(), &action).await)
}

#[wasm_bindgen]
pub async fn project_test(project_id: String) -> Result<JsValue, JsValue> {
    let action = ProjectActionRequest {
        project_id,
        action: ActionType::Test,
    };
    api(proto::project_action(&api_url(), &action).await)
}

#[wasm_bindgen]
pub async fn project_check(project_id: String) -> Result<JsValue, JsValue> {
    let action = ProjectActionRequest {
        project_id,
        action: ActionType::Check,
    };
    api(proto::project_action(&api_url(), &action).await)
}
