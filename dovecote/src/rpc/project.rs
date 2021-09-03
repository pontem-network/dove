use dove::home::{Home, load_project, Project as DoveProject, path_id};
use anyhow::Error;
use proto::project::{ProjectList, ProjectShortInfo, ProjectInfo, ID, Tree};
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use std::fs;
use parking_lot::RwLock;
use std::sync::Arc;
use std::hash::Hash;

#[derive(Debug)]
pub struct Projects {
    dove_home: Home,
    map: RwLock<HashMap<ID, Project>>,
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

    pub fn by_id(&self, id: ID) -> Result<ProjectInfo, Error> {
        {
            let map = self.map.read();
            if let Some(project) = map.get(&id) {
                return Ok(project.info());
            }
        }
        let mut map = self.map.write();
        if let Some(project) = map.get(&id) {
            return Ok(project.info());
        }

        let path = self
            .dove_home
            .get_project_path(&id)?
            .ok_or_else(|| anyhow::anyhow!("Project with id :'{}' was not found.", id))?;
        let project = Project::load(&path)?;
        let info = project.info();
        map.insert(id, project);
        Ok(info)
    }
}

#[derive(Debug)]
pub struct Project {
    pub tree: Arc<Tree>,
    pub info: DoveProject,
    pub file_map: HashMap<String, Arc<PathBuf>>,
}

impl Project {
    pub fn load(path: &Path) -> Result<Project, Error> {
        let info = load_project(path)?;
        let exclude_dir = path.join(&info.toml.layout.artifacts);
        let mut file_map = HashMap::default();

        Ok(Project {
            tree: Arc::new(Tree::Dir(
                info.name.clone(),
                load_tree(path, &exclude_dir, &mut file_map)?,
            )),
            info,
            file_map,
        })
    }

    pub fn info(&self) -> ProjectInfo {
        ProjectInfo {
            short: ProjectShortInfo {
                id: self.info.id.clone(),
                name: self.info.name.clone(),
            },
            tree: self.tree.clone(),
        }
    }
}

fn load_tree(
    path: &Path,
    exclude_dir: &Path,
    file_map: &mut HashMap<String, Arc<PathBuf>>,
) -> Result<Vec<Tree>, Error> {
    let mut tree = vec![];
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let name = Arc::new(entry.file_name().to_string_lossy().into_owned());
        let path = entry.path();
        if path.is_dir() {
            if exclude_dir == path.as_path() {
                continue;
            }

            tree.push(Tree::Dir(name, load_tree(&path, exclude_dir, file_map)?));
        } else {
            let id = path_id(&path);
            file_map.insert(id.clone(), Arc::new(path));
            tree.push(Tree::File(Arc::new(id), name));
        }
    }
    Ok(tree)
}

trait Key: Eq + Hash {}