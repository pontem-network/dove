use std::collections::HashMap;

use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::*;

use proto::file::{
    Diff, FId, Flush, FileIdentifier, ProjectId, CreateFsEntry, RenameFile, RenameDirectory,
    RemoveDirectory,
};
use proto::project::Id;
use crate::context::*;
use crate::{js_err, api};

#[wasm_bindgen]
pub async fn get_file(project_id: Id, file_id: Id) -> Result<JsValue, JsValue> {
    let get_file = FileIdentifier {
        project_id,
        file_id,
    };

    let file = proto::get_file(&api_url(), &get_file)
        .await
        .map_err(js_err)?;

    wasm_bindgen::JsValue::from_serde(&file).map_err(js_err)
}

#[wasm_bindgen]
pub async fn flush(events: JsValue) -> JsValue {
    if events.is_undefined() || events.is_null() {
        return JsValue::null();
    }

    let project_map = match events.into_serde::<HashMap<String, HashMap<String, Vec<Diff>>>>() {
        Ok(diff) => diff,
        Err(err) => {
            return JsValue::from_serde(&FlushResult::Error(format!(
                "Invalid events format. Error: {}",
                err
            )))
            .unwrap_or_else(|_| JsValue::null());
        }
    };

    if project_map.is_empty() {
        return JsValue::from_serde(&FlushResult::Ok).unwrap_or_else(|_| JsValue::null());
    }

    let flush = Flush { project_map };
    match proto::flush(&api_url(), &flush).await {
        Ok(res) => JsValue::from_serde(&FlushResult::Errors(res.errors)),
        Err(err) => JsValue::from_serde(&FlushResult::Error(err.to_string())),
    }
    .unwrap_or_else(|_| JsValue::null())
}

#[derive(Serialize, Deserialize, Debug)]
pub enum FlushResult {
    Ok,
    Error(String),
    Errors(HashMap<ProjectId, HashMap<FId, String>>),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ModelContentChange {
    /// The range that got replaced.
    pub range: Range,
    /// The offset of the range that got replaced.
    #[serde(alias = "rangeOffset")]
    pub range_offset: u32,
    /// The length of the range that got replaced.
    #[serde(alias = "rangeLength")]
    pub range_length: u32,
    /// The new text for the range.
    pub text: String,
}

///A range in the editor. This interface is suitable for serialization.
#[derive(Serialize, Deserialize, Debug)]
pub struct Range {
    /// Line number on which the range starts (starts at 1).
    #[serde(alias = "startLineNumber")]
    pub start_line_number: u32,

    /// Column on which the range starts in line `startLineNumber` (starts at 1).
    #[serde(alias = "startColumn")]
    pub start_column: u32,

    /// Line number on which the range ends.
    #[serde(alias = "endLineNumber")]
    pub end_line_number: u32,

    /// Column on which the range ends in line `endLineNumber`.
    #[serde(alias = "endColumn")]
    pub end_column: u32,
}

#[wasm_bindgen]
pub async fn create_directory(
    project_id: Id,
    path: String,
    name: String,
) -> Result<JsValue, JsValue> {
    let req = CreateFsEntry {
        project_id,
        path,
        name,
    };
    api(proto::create_directory(&api_url(), &req).await)
}

#[wasm_bindgen]
pub async fn rename_directory(
    project_id: Id,
    path: String,
    old_name: String,
    new_name: String,
) -> Result<JsValue, JsValue> {
    let req = RenameDirectory {
        project_id,
        path,
        old_name,
        new_name,
    };
    api(proto::rename_directory(&api_url(), &req).await)
}

#[wasm_bindgen]
pub async fn remove_directory(project_id: Id, path: String) -> Result<JsValue, JsValue> {
    let req = RemoveDirectory { project_id, path };
    api(proto::remove_directory(&api_url(), &req).await)
}

#[wasm_bindgen]
pub async fn remove_file(project_id: Id, file_id: Id) -> Result<JsValue, JsValue> {
    let req = FileIdentifier {
        project_id,
        file_id,
    };
    api(proto::remove_file(&api_url(), &req).await)
}

#[wasm_bindgen]
pub async fn create_file(project_id: Id, path: String, name: String) -> Result<JsValue, JsValue> {
    let req = CreateFsEntry {
        project_id,
        path,
        name,
    };
    api(proto::create_file(&api_url(), &req).await)
}

#[wasm_bindgen]
pub async fn rename_file(
    project_id: Id,
    file_id: Id,
    new_name: String,
) -> Result<JsValue, JsValue> {
    let req = RenameFile {
        project_id,
        file_id,
        new_name,
    };
    api(proto::rename_file(&api_url(), &req).await)
}
