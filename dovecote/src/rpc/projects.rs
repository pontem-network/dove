use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicI64, Ordering};

use anyhow::Error;
use chrono::Utc;
use parking_lot::RwLock;

use dove::home::Home;
use proto::project::{Id, ProjectList, ProjectShortInfo, IdRef};

use crate::rpc::project::Project;

const PROJECT_LIFETIME: i64 = 60 * 10;

type ProjectMut = Arc<RwLock<Project>>;

#[derive(Debug)]
pub struct Projects {
    dove_home: Home,
    map: RwLock<HashMap<Id, (AtomicI64, ProjectMut)>>,
}

impl Projects {
    pub fn new() -> Result<Projects, Error> {
        Ok(Projects {
            dove_home: Home::get()?,
            map: RwLock::new(HashMap::new()),
        })
    }

    pub fn reset_project(&self, id: Id) {
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

    fn get_project(&self, id: IdRef) -> Result<ProjectMut, Error> {
        let current_time = get_unix_timestamp();
        {
            let map = self.map.read();
            if let Some((last_read, project)) = map.get(id) {
                last_read.store(current_time, Ordering::Relaxed);
                return Ok(project.clone());
            }
        }

        let mut map = self.map.write();
        if let Some((last_read, project)) = map.get(id) {
            last_read.store(current_time, Ordering::Relaxed);
            return Ok(project.clone());
        }

        let path = self
            .dove_home
            .get_project_path(id)?
            .ok_or_else(|| anyhow::anyhow!("Project with id :'{}' was not found.", id))?;

        let project = Arc::new(RwLock::new(Project::load(&path)?));
        map.insert(
            id.to_owned(),
            (AtomicI64::new(current_time), project.clone()),
        );
        Ok(project)
    }

    pub fn on_project<F, T>(&self, id: IdRef, on_proj: F) -> Result<T, Error>
    where
        F: FnOnce(&Project) -> Result<T, Error>,
    {
        let projects = self.get_project(id)?;
        let project = projects.read();
        on_proj(&project)
    }

    pub fn on_project_mut<F, T>(&self, id: IdRef, on_proj: F) -> Result<T, Error>
    where
        F: FnOnce(&mut Project) -> Result<T, Error>,
    {
        let projects = self.get_project(id)?;
        let mut project = projects.write();
        on_proj(&mut project)
    }

    pub fn reload(&self, id: &Id) -> Result<Arc<RwLock<Project>>, Error> {
        {
            let mut map = self.map.write();
            map.remove(id);
        }
        self.get_project(id)
    }

    pub fn clean_up(&self) {
        let mut map = self.map.write();
        let current_timestamp = get_unix_timestamp();
        map.retain(|_, (last_read, project)| {
            let last_read = last_read.load(Ordering::Relaxed);
            let must_live = PROJECT_LIFETIME > current_timestamp - last_read;
            if !must_live {
                debug!("Remove project from cache:'{}'", project.read().info.path);
            }
            must_live
        });
    }
}

pub fn get_unix_timestamp() -> i64 {
    let now = Utc::now();
    now.timestamp()
}

unsafe impl Send for Projects {}

unsafe impl Sync for Projects {}
