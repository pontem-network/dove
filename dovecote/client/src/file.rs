use wasm_bindgen::prelude::*;

use wasm_bindgen::JsValue;
use crate::context::*;
use crate::js_err;
use proto::project::ID;
use proto::file::GetFile;
use crate::console_log;

#[wasm_bindgen]
pub async fn get_file(
    project_id: ID,
    file_id: ID,
) -> Result<JsValue, JsValue> {
    console_log!(
        "get_file:{}-{}",
        project_id,
        file_id,
    );
    let get_file = GetFile {
        project_id,
        file_id,
    };

    let file = proto::get_file(&api_url(), get_file)
        .await
        .map_err(js_err)?;

    wasm_bindgen::JsValue::from_serde(&file).map_err(js_err)
}

#[wasm_bindgen]
pub async fn on_file_change(project_id: ID, file_id: ID, event: JsValue) {
    console_log!("on_file_change:{}-{};{:?}",project_id,file_id, event);
    let event = event.into_serde::<ModelContentChangedEvent>();
    console_log!("{:?}", event);
}

#[wasm_bindgen]
pub async fn flush() {}

#[derive(Serialize, Deserialize, Debug)]
pub struct ModelContentChangedEvent {
    pub changes: Vec<ModelContentChange>,
    ///The (new) end-of-line character.
    pub eol: String,
    /// The new version id the model has transitioned to.
    #[serde(alias = "versionId")]
    pub version_id: u64,
    /// Flag that indicates that this event was generated while undoing.
    #[serde(alias = "isUndoing")]
    pub is_undoing: bool,
    /// Flag that indicates that this event was generated while redoing.
    #[serde(alias = "isRedoing")]
    pub is_redoing: bool,
    /// Flag that indicates that all decorations were lost with this edit.
    /// The model has been reset to a new value.
    #[serde(alias = "isFlush")]
    pub is_flush: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ModelContentChange {
    /// The range that got replaced.
    pub range: Range,
    /// The offset of the range that got replaced.
    #[serde(alias = "rangeOffset")]
    pub range_offset: u64,
    /// The length of the range that got replaced.
    #[serde(alias = "rangeLength")]
    pub range_length: u64,
    /// The new text for the range.
    pub text: String,
}

///A range in the editor. This interface is suitable for serialization.
#[derive(Serialize, Deserialize, Debug)]
pub struct Range {
    /// Line number on which the range starts (starts at 1).
    #[serde(alias = "startLineNumber")]
    pub start_line_number: u64,

    /// Column on which the range starts in line `startLineNumber` (starts at 1).
    #[serde(alias = "startColumn")]
    pub start_column: u64,

    /// Line number on which the range ends.
    #[serde(alias = "endLineNumber")]
    pub end_line_number: u64,

    /// Column on which the range ends in line `endLineNumber`.
    #[serde(alias = "endColumn")]
    pub end_column: u64,
}
