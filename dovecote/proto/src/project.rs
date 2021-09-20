use std::sync::Arc;

pub type Id = String;
pub type IdRef<'a> = &'a str;

#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectList {
    pub projects: Vec<ProjectShortInfo>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ProjectShortInfo {
    pub id: Arc<Id>,
    pub name: Arc<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectInfo {
    pub short: ProjectShortInfo,
    pub tree: Arc<Tree>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Tree {
    Dir(Arc<String>, Vec<Tree>),
    File(Arc<Id>, Arc<String>),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ProjectActionRequest {
    pub project_id: Id,
    pub action: ActionType,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum ActionType {
    Build,
    Clean,
    Test,
    Check,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ProjectActionResponse {
    pub content: String,
    pub code: u8,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CreateProject {
    pub name: String,
    pub dialect: String,
}
