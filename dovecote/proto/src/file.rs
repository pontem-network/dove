use std::collections::HashMap;
use crate::Id;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GetFile {
    pub project_id: Id,
    pub file_id: Id,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct File {
    pub content: String,
    pub tp: String,
}

pub type ProjectId = Id;
pub type FileId = Id;

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Flush {
    pub project_map: HashMap<ProjectId, HashMap<FileId, Vec<Diff>>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Diff {
    /// The offset of the range that got replaced.
    #[serde(alias = "rangeOffset")]
    pub range_offset: u32,
    /// The length of the range that got replaced.
    #[serde(alias = "rangeLength")]
    pub range_length: u32,
    /// The new text for the range.
    pub text: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct FlushResult {
    pub errors: HashMap<ProjectId, HashMap<FileId, String>>,
}
