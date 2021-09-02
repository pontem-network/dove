use std::path::{PathBuf, Path};
use anyhow::Error;
use std::fs;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use serde::{Serialize, Deserialize};
use crate::manifest::{MANIFEST, read_manifest};

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
        let id = project_id(path);
        let rf = bcs::to_bytes(&Ref {
            path: path.to_string_lossy().to_string(),
        })?;
        fs::write(projects_dir.join(format!("{}", id)), rf)?;
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
}

fn project_id(path: &Path) -> u64 {
    let mut s = DefaultHasher::new();
    path.hash(&mut s);
    s.finish()
}

#[derive(Serialize, Deserialize)]
struct Ref {
    path: String,
}

/// Project short info.
#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    /// Project id.
    pub id: u64,
    /// Project name.
    pub name: String,
    /// Project path.
    pub path: String,
}

fn load_project_by_ref<P: AsRef<Path>>(path: P) -> Result<Project, Error> {
    let rf = bcs::from_bytes::<Ref>(&fs::read(&path)?)?;
    match load_project(rf.path) {
        Ok(project) => Ok(project),
        Err(err) => {
            fs::remove_file(path)?;
            Err(err)
        }
    }
}

fn load_project<P: AsRef<Path>>(path: P) -> Result<Project, Error> {
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
        .or_else(|| {
            project_path
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
        })
        .unwrap_or_else(|| "untitled".to_string());

    Ok(Project {
        id: project_id(project_path),
        name,
        path: project_path.to_string_lossy().to_string(),
    })
}
