use std::sync::Arc;

pub type ID = String;

#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectList {
    pub projects: Vec<ProjectShortInfo>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ProjectShortInfo {
    pub id: Arc<ID>,
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
    File(Arc<ID>, Arc<String>),
}
