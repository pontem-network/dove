mod projects;
mod project;
mod m_file;

use proto::{OnRequest, Empty};
use proto::project::{ProjectList, ID, ProjectInfo};
use anyhow::Error;
use crate::rpc::projects::Projects;
use proto::file::{GetFile, File};

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
        self.projects.on_project(id, |p| Ok(p.info()))
    }

    fn get_file(&self, req: GetFile) -> Result<File, Error> {
        let GetFile { project_id, file_id } = req;
        self.projects.on_project_mut(project_id, |p| p.load_file(file_id))
    }
}
