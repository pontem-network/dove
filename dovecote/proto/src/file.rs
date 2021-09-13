use std::sync::Arc;
use std::collections::HashMap;
use crate::ID;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GetFile {
    pub project_id: ID,
    pub file_id: ID,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct File {
    pub content: Arc<String>,
    pub tp: String,
    pub hash: Arc<String>,
}

pub type ProjectId = ID;
pub type FileId = ID;

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
