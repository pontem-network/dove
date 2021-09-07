use std::sync::Arc;
use crate::ID;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GetFile {
    pub project_id: ID,
    pub file_id: ID,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct File {
    pub content: Arc<String>,
    pub hash: Arc<String>,
}