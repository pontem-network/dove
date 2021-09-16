use std::collections::HashMap;

use anyhow::Error;

use proto::{Empty, OnRequest};
use proto::file::{
    CreateFileResult, CreateFsEntry, FId, File, FileIdentifier, Flush, FlushResult,
    RenameDirectory, RenameFile, RemoveDirectory,
};
use proto::project::{Id, ProjectInfo, ProjectList, ProjectActionRequest, ProjectActionResponse};

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

    fn project_info(&self, id: Id) -> Result<ProjectInfo, anyhow::Error> {
        self.projects.on_project(&id, |p| Ok(p.info()))
    }

    fn get_file(&self, req: FileIdentifier) -> Result<File, Error> {
        let FileIdentifier {
            project_id,
            file_id,
        } = req;
        self.projects.on_project_mut(&project_id, |p| {
            p.load_file(file_id.as_ref()).map(|f| f.make_model())
        })
    }

    fn remove_file(&self, req: FileIdentifier) -> Result<Empty, Error> {
        let FileIdentifier {
            project_id,
            file_id,
        } = req;
        self.projects.on_project_mut(&project_id, |p| {
            p.remove_file(file_id.as_ref())?;
            Ok(Empty)
        })
    }

    fn create_file(&self, req: CreateFsEntry) -> Result<CreateFileResult, Error> {
        let CreateFsEntry {
            project_id,
            path,
            name,
        } = req;
        self.projects.on_project_mut(&project_id, |p| {
            let id = p.create_file(path, name)?;
            let file = p.load_file(id.as_ref()).map(|f| f.make_model())?;

            Ok(CreateFileResult {
                project_info: p.info(),
                file_id: id,
                file,
            })
        })
    }

    fn create_directory(&self, req: CreateFsEntry) -> Result<ProjectInfo, Error> {
        let CreateFsEntry {
            project_id,
            path,
            name,
        } = req;
        self.projects.on_project_mut(&project_id, |p| {
            p.create_dir(path, name)?;
            Ok(p.info())
        })
    }

    fn rename_file(&self, req: RenameFile) -> Result<FId, Error> {
        let RenameFile {
            project_id,
            file_id,
            new_name,
        } = req;
        self.projects.on_project_mut(&project_id, |p| {
            let id = p.rename_file(file_id, new_name)?;
            Ok(id)
        })
    }

    fn rename_directory(&self, req: RenameDirectory) -> Result<ProjectInfo, Error> {
        let RenameDirectory {
            project_id,
            path,
            old_name,
            new_name,
        } = req;
        self.projects.on_project_mut(&project_id, |p| {
            p.rename_directory(path, old_name, new_name)?;
            Ok(p.info())
        })
    }

    fn remove_directory(&self, req: RemoveDirectory) -> Result<ProjectInfo, Error> {
        let RemoveDirectory { project_id, path } = req;
        self.projects.on_project_mut(&project_id, |p| {
            p.remove_directory(path)?;
            Ok(p.info())
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
                                    (
                                        p.load_file(f_id.as_ref()).map(|f| f.update_file(diff)),
                                        f_id,
                                    )
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

    fn sync_project(&self, id: Id) -> Result<ProjectInfo, Error> {
        let project = self.projects.reload(&id)?;
        let project = project.read();
        Ok(project.info())
    }

    fn project_action(
        &self,
        req: ProjectActionRequest,
    ) -> Result<ProjectActionResponse, anyhow::Error> {
        let ProjectActionRequest { project_id, action } = req;
        self.projects
            .on_project_mut(&project_id, |project| project.action(action))
    }
}
