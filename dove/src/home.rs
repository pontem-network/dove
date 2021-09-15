use std::path::{PathBuf, Path};
use anyhow::Error;
use std::fs;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use serde::{Serialize, Deserialize};
use crate::manifest::{MANIFEST, read_manifest, DoveToml};
use std::sync::Arc;

const PROJECTS: &str = "projects";

/// Dove home.
#[derive(Debug)]
pub struct Home {
    path: PathBuf,
}

impl Home {
    /// Returns dove home.
    pub fn get() -> Result<Home, Error> {
        let dove_home = PathBuf::from(std::env::var("HOME")?).join(".dove");
        if !dove_home.exists() {
            fs::create_dir_all(&dove_home)?;
        }
        Ok(Home { path: dove_home })
    }

    /// Reg dove project.
    pub fn reg_package(&self, path: &Path) -> Result<(), Error> {
        let projects_dir = self.path.join(PROJECTS);
        if !projects_dir.exists() {
            fs::create_dir_all(&projects_dir)?;
        }
        let id = path_id(path);
        let rf = bcs::to_bytes(&Ref {
            path: path.to_string_lossy().to_string(),
        })?;
        fs::write(projects_dir.join(id), rf)?;
        Ok(())
    }

    /// Return project list.
    pub fn load_project_list(&self) -> Result<Vec<Project>, Error> {
        let projects_dir = self.path.join(PROJECTS);
        if !projects_dir.exists() {
            fs::create_dir_all(&projects_dir)?;
        }

        Ok(fs::read_dir(projects_dir)?
            .into_iter()
            .filter_map(|path| path.map(|entry| entry.path()).ok())
            .filter_map(|path| {
                if path.is_file() {
                    load_project_by_ref(path).ok()
                } else {
                    load_project(path).ok()
                }
            })
            .collect())
    }

    ///Returns project path by id.
    pub fn get_project_path(&self, id: &str) -> Result<Option<PathBuf>, Error> {
        let projects_dir = self.path.join(PROJECTS);
        if !projects_dir.exists() {
            fs::create_dir_all(&projects_dir)?;
        }

        let ref_path = projects_dir.join(id);
        if !ref_path.exists() {
            Ok(None)
        } else if ref_path.is_dir() {
            Ok(Some(ref_path))
        } else {
            let rf = read_ref(&ref_path)?;
            let project_path = rf.as_ref();
            if !project_path.exists() {
                fs::remove_file(ref_path)?;
                Ok(None)
            } else {
                Ok(Some(project_path.to_path_buf()))
            }
        }
    }
}

/// Make path id.
pub fn path_id(path: &Path) -> String {
    let mut s = DefaultHasher::new();
    path.hash(&mut s);
    let id = s.finish();
    hex::encode(&id.to_le_bytes())
}

#[derive(Serialize, Deserialize)]
struct Ref {
    path: String,
}

impl AsRef<Path> for Ref {
    fn as_ref(&self) -> &Path {
        self.path.as_ref()
    }
}

/// Project short info.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Project {
    /// Project id.
    pub id: Arc<String>,
    /// Dove toml.
    pub toml: DoveToml,
    /// Project name.
    pub name: Arc<String>,
    /// Project path.
    pub path: String,
}

fn read_ref<P: AsRef<Path>>(path: P) -> Result<Ref, Error> {
    Ok(bcs::from_bytes::<Ref>(&fs::read(&path)?)?)
}

fn load_project_by_ref<P: AsRef<Path>>(path: P) -> Result<Project, Error> {
    let rf = read_ref(&path)?;
    match load_project(rf) {
        Ok(project) => Ok(project),
        Err(err) => {
            fs::remove_file(path)?;
            Err(err)
        }
    }
}

/// Load project.
pub fn load_project<P: AsRef<Path>>(path: P) -> Result<Project, Error> {
    let project_path = path.as_ref();

    let manifest = project_path.join(MANIFEST);
    let manifest = if !manifest.exists() {
        bail!("could not find `{}` in `{:?}`.", MANIFEST, project_path)
    } else {
        read_manifest(&manifest)?
    };

    let name = manifest
        .package
        .name
        .clone()
        .or_else(|| {
            project_path
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
        })
        .unwrap_or_else(|| "untitled".to_string());

    Ok(Project {
        id: Arc::new(path_id(project_path)),
        name: Arc::new(name),
        toml: manifest,
        path: project_path.to_string_lossy().to_string(),
    })
}
