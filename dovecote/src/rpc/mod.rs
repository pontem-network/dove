mod project;

use proto::{OnRequest, Empty};
use proto::project::{ProjectList, ID, ProjectInfo, ProjectShortInfo};
use std::path::PathBuf;
use dove::home::Home;
use anyhow::Error;

#[derive(Debug)]
pub struct Rpc {
    pub dove_home: Home,
}

impl Rpc {
    pub fn new() -> Result<Rpc, Error> {
        Ok(Rpc {
            dove_home: Home::get()?
        })
    }
}

impl OnRequest for Rpc {
    fn project_list(&self, _: Empty) -> Result<ProjectList, anyhow::Error> {
        let projects = self.dove_home.load_project_list()?.into_iter()
            .map(|p| ProjectShortInfo {
                id: p.id,
                name: p.name,
                path: p.path,
            }).collect();

        Ok(ProjectList { projects })
    }

    fn project_info(&self, req: ID) -> Result<ProjectInfo, anyhow::Error> {
        todo!()
    }
}