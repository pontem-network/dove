use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use anyhow::Error;

use dove::home::{load_project, path_id, Project as DoveProject};
use proto::project::{ID, ProjectInfo, ProjectShortInfo, Tree};

use crate::rpc::m_file::MFile;

#[derive(Debug)]
pub struct Project {
    pub tree: Arc<Tree>,
    pub info: DoveProject,
    pub file_paths: HashMap<ID, Arc<PathBuf>>,
    pub file_map: HashMap<ID, MFile>,
}

impl Project {
    pub fn load(path: &Path) -> Result<Project, Error> {
        let info = load_project(path)?;
        let exclude_dir = path.join(&info.toml.layout.artifacts);
        let mut file_paths = HashMap::default();

        Ok(Project {
            tree: Arc::new(Tree::Dir(
                info.name.clone(),
                load_tree(path, &exclude_dir, &mut file_paths)?,
            )),
            info,
            file_paths,
            file_map: Default::default(),
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

    pub fn load_file(&mut self, id: &ID) -> Result<&mut MFile, Error> {
        if !self.file_map.contains_key(id) {
            if let Some(path) = self.file_paths.get(id) {
                let m_file = MFile::load(path.clone())?;
                self.file_map.insert(id.to_owned(), m_file);
            } else {
                bail!("File with id:{} was not found.", id)
            }
        }

        if let Some(file) = self.file_map.get_mut(id) {
            return Ok(file);
        } else {
            bail!("File with id:{} was not found.", id)
        }
    }
}

fn load_tree(
    path: &Path,
    exclude_dir: &Path,
    file_map: &mut HashMap<ID, Arc<PathBuf>>,
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
