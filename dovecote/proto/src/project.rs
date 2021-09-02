pub type ID = u64;

#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectList {
    pub projects: Vec<ProjectShortInfo>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectShortInfo {
    pub id: ID,
    pub name: String,
    pub path: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectInfo {
    pub short: ProjectShortInfo,
}
