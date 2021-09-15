use std::collections::HashMap;

use anyhow::Error;

use proto::{Empty, OnRequest};
use proto::file::{File, Flush, FlushResult, GetFile};
use proto::project::{ID, ProjectInfo, ProjectList};

use crate::rpc::projects::Projects;

mod m_file;
mod project;
mod projects;

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
        self.projects.on_project(&id, |p| Ok(p.info()))
    }

    fn get_file(&self, req: GetFile) -> Result<File, Error> {
        let GetFile {
            project_id,
            file_id,
        } = req;
        self.projects.on_project_mut(&project_id, |p| {
            p.load_file(&file_id).map(|f| f.make_model())
        })
    }

    fn flush(&self, req: Flush) -> Result<FlushResult, Error> {
        let errors = req
            .project_map
            .into_iter()
            .map(|(p_id, files)| {
                (
                    self.projects
                        .on_project_mut(&p_id, |p| {
                            Ok(files
                                .into_iter()
                                .map(|(f_id, diff)| {
                                    (p.load_file(&f_id).map(|f| f.update_file(diff)), f_id)
                                })
                                .filter_map(|(res, f_id)| {
                                    if let Err(err) = res {
                                        Some((f_id, err.to_string()))
                                    } else {
                                        None
                                    }
                                })
                                .collect::<HashMap<_, _>>())
                        })
                        .unwrap_or_default(),
                    p_id,
                )
            })
            .map(|(errors, id)| (id, errors))
            .filter(|(_, errors)| !errors.is_empty())
            .collect();
        Ok(FlushResult { errors })
    }

    fn sync_project(&self, id: ID) -> Result<ProjectInfo, Error> {
        let project = self.projects.reload(&id)?;
        let project = project.read();
        Ok(project.info())
    }
}
