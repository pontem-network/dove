use std::collections::HashMap;
use crate::Id;
use crate::ProjectInfo;

pub type ProjectId = Id;
pub type FId = Id;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FileIdentifier {
    pub project_id: Id,
    pub file_id: Id,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct File {
    pub content: String,
    pub tp: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CreateFsEntry {
    pub project_id: Id,
    pub path: String,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RenameFile {
    pub project_id: Id,
    pub file_id: Id,
    pub new_name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RenameDirectory {
    pub project_id: Id,
    pub path: String,
    pub old_name: String,
    pub new_name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RemoveDirectory {
    pub project_id: Id,
    pub path: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Flush {
    pub project_map: HashMap<ProjectId, HashMap<FId, Vec<Diff>>>,
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
    pub errors: HashMap<ProjectId, HashMap<FId, String>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateFileResult {
    pub project_info: ProjectInfo,
    pub file_id: FId,
    pub file: File,
}
