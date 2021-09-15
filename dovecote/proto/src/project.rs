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
