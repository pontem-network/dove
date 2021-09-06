mod project;

use proto::{OnRequest, Empty};
use proto::project::{ProjectList, ID, ProjectInfo};
use anyhow::Error;
use crate::rpc::project::Projects;

#[derive(Debug)]
pub struct Rpc {
    pub projects: Projects,
}

impl Rpc {
    pub fn new() -> Result<Rpc, Error> {
        Ok(Rpc {
            projects: Projects::new()?,
        })
    }
}

impl OnRequest for Rpc {
    fn project_list(&self, _: Empty) -> Result<ProjectList, anyhow::Error> {
        self.projects.list()
    }

    fn project_info(&self, id: ID) -> Result<ProjectInfo, anyhow::Error> {
        self.projects.by_id(id)
    }
}
