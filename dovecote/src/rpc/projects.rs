use dove::home::Home;
use anyhow::Error;
use proto::project::{ProjectList, ProjectShortInfo, ID};
use std::collections::HashMap;
use parking_lot::RwLock;
use std::sync::atomic::{AtomicI64, Ordering};
use chrono::Utc;
use crate::rpc::project::Project;

const PROJECT_LIFETIME: i64 = 60 * 10;

#[derive(Debug)]
pub struct Projects {
    dove_home: Home,
    map: RwLock<HashMap<ID, (Project, AtomicI64)>>,
}

impl Projects {
    pub fn new() -> Result<Projects, Error> {
        Ok(Projects {
            dove_home: Home::get()?,
            map: RwLock::new(Default::default()),
        })
    }

    pub fn reset_project(&self, id: ID) {
        let mut list = self.map.write();
        list.remove(&id);
    }

    pub fn list(&self) -> Result<ProjectList, Error> {
        let projects = self
            .dove_home
            .load_project_list()?
            .into_iter()
            .map(|p| ProjectShortInfo {
                id: p.id,
                name: p.name,
            })
            .collect();

        Ok(ProjectList { projects })
    }

    pub fn on_project<F, T>(&self, id:ID, on_proj: F) -> Result<T, Error>
        where F: FnOnce(&Project) -> Result<T, Error> {
        let current_time = get_unix_timestamp();
        {
            let map = self.map.read();
            if let Some((project, last_read)) = map.get(&id) {
                last_read.store(current_time, Ordering::Relaxed);
                return on_proj(project);
            }
        }
        let mut map = self.map.write();
        if let Some((project, last_read)) = map.get(&id) {
            last_read.store(current_time, Ordering::Relaxed);
            return on_proj(project);
        }

        let path = self
            .dove_home
            .get_project_path(&id)?
            .ok_or_else(|| anyhow::anyhow!("Project with id :'{}' was not found.", id))?;
        let project = Project::load(&path)?;

        let res = on_proj(&project);
        map.insert(id, (project, AtomicI64::new(current_time)));
        res
    }

    pub fn on_project_mut<F, T>(&self, id:ID, on_proj: F) -> Result<T, Error>
        where F: FnOnce(&mut Project) -> Result<T, Error> {
        let current_time = get_unix_timestamp();
        let mut map = self.map.write();
        if let Some((project, last_read)) = map.get_mut(&id) {
            last_read.store(current_time, Ordering::Relaxed);
            return on_proj(project);
        }

        let path = self
            .dove_home
            .get_project_path(&id)?
            .ok_or_else(|| anyhow::anyhow!("Project with id :'{}' was not found.", id))?;
        let mut project = Project::load(&path)?;

        let res = on_proj(&mut project);
        map.insert(id, (project, AtomicI64::new(current_time)));
        res
    }

    // pub fn project_info_by_id(&self, id: ID) -> Result<ProjectInfo, Error> {
    //     let current_time = get_unix_timestamp();
    //
    //     {
    //         let map = self.map.read();
    //         if let Some((project, last_read)) = map.get(&id) {
    //             last_read.store(current_time, Ordering::Relaxed);
    //             return Ok(project.info());
    //         }
    //     }
    //     let mut map = self.map.write();
    //     if let Some((project, last_read)) = map.get(&id) {
    //         last_read.store(current_time, Ordering::Relaxed);
    //         return Ok(project.info());
    //     }
    //
    //     let path = self
    //         .dove_home
    //         .get_project_path(&id)?
    //         .ok_or_else(|| anyhow::anyhow!("Project with id :'{}' was not found.", id))?;
    //     let project = Project::load(&path)?;
    //     let info = project.info();
    //     map.insert(id, (project, AtomicI64::new(current_time)));
    //     Ok(info)
    // }

    pub fn clean_up(&self) {
        let mut map = self.map.write();
        let current_timestamp = get_unix_timestamp();
        map.retain(|_, (project, last_read)| {
            let last_read = last_read.load(Ordering::Relaxed);
            let must_live = PROJECT_LIFETIME > current_timestamp - last_read;
            if !must_live {
                debug!("Remove project from cache:'{}'", project.info.path);
            }
            must_live
        });
    }
}

pub fn get_unix_timestamp() -> i64 {
    let now = Utc::now();
    now.timestamp()
}
