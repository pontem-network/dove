use core::mem;
use std::collections::HashMap;

use once_cell::sync::Lazy;
use parking_lot::Mutex;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::*;

use proto::file::{Diff, FileId, Flush, GetFile, ProjectId};
use proto::project::ID;

use crate::console_log;
use crate::context::*;
use crate::js_err;

const PROJECTS_DIFF: Lazy<Mutex<Flush>> = Lazy::new(|| Mutex::new(Default::default()));

#[wasm_bindgen]
pub async fn get_file(project_id: ID, file_id: ID) -> Result<JsValue, JsValue> {
    console_log!("get_file:{}-{}", project_id, file_id,);
    let get_file = GetFile {
        project_id,
        file_id,
    };

    let file = proto::get_file(&api_url(), &get_file)
        .await
        .map_err(js_err)?;

    wasm_bindgen::JsValue::from_serde(&file).map_err(js_err)
}

#[wasm_bindgen]
pub fn on_file_change(project_id: ID, file_id: ID, event: JsValue) -> Result<(), JsValue> {
    let event = event
        .into_serde::<ModelContentChangedEvent>()
        .map_err(js_err)?;
    let diff_vec = event.changes.into_iter().map(|change| Diff {
        range_offset: change.range_offset,
        range_length: change.range_length,
        text: change.text,
    });

    PROJECTS_DIFF
        .lock()
        .project_map
        .entry(project_id)
        .or_default()
        .entry(file_id)
        .or_default()
        .extend(diff_vec);
    Ok(())
}

#[wasm_bindgen]
pub async fn flush() -> JsValue {
    let projects_diff = &*PROJECTS_DIFF;
    let mut projects_diff = projects_diff.lock();

    let res = match proto::flush(&api_url(), &projects_diff).await {
        Ok(res) => res,
        Err(err) => {
            return JsValue::from_serde(&FlushResult::Error(err.to_string()))
                .unwrap_or_else(|_| JsValue::null());
        }
    };

    if res.errors.is_empty() {
        JsValue::from_serde(&FlushResult::Ok).unwrap_or_else(|_| JsValue::null())
    } else {
        let (mut non_commited_diff, errors) = res.errors.into_iter().fold(
            (
                HashMap::<ProjectId, HashMap<FileId, Vec<Diff>>>::default(),
                HashMap::<ProjectId, HashMap<FileId, String>>::default(),
            ),
            |(mut non_commited_diff, mut errors), (project_id, files)| {
                let mut project_diff = projects_diff.project_map.remove(&project_id);

                let project_errs = errors.entry(project_id.clone()).or_default();
                let project_non_commited_diff =
                    non_commited_diff.entry(project_id.clone()).or_default();

                for (id, err) in files {
                    project_errs.insert(id.clone(), err);
                    if let Some(project_diff) = project_diff.as_mut() {
                        if let Some(diff) = project_diff.remove(&id) {
                            project_non_commited_diff.insert(id, diff);
                        }
                    }
                }

                (non_commited_diff, errors)
            },
        );
        mem::swap(&mut non_commited_diff, &mut projects_diff.project_map);
        JsValue::from_serde(&FlushResult::Errors(errors)).unwrap_or_else(|_| JsValue::null())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum FlushResult {
    Ok,
    Error(String),
    Errors(HashMap<ProjectId, HashMap<FileId, String>>),
}

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
