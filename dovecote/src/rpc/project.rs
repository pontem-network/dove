use core::mem;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::sync::Arc;

use anyhow::Error;

use dove::home::{load_project, path_id, Project as DoveProject};
use proto::project::{
    Id, IdRef, ProjectInfo, ProjectShortInfo, Tree, ActionType, ProjectActionResponse,
};

use crate::rpc::m_file::MFile;
use std::time::Instant;
use std::process::Command;

#[derive(Debug)]
pub struct Project {
    pub tree: Arc<Tree>,
    pub info: DoveProject,
    pub file_paths: HashMap<Id, Arc<PathBuf>>,
    pub file_map: HashMap<Id, MFile>,
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

    pub fn load_file(&mut self, id: IdRef) -> Result<&mut MFile, Error> {
        if !self.file_map.contains_key(id) {
            if let Some(path) = self.file_paths.get(id) {
                let m_file = MFile::load(path.clone())?;
                self.file_map.insert(id.to_string(), m_file);
            } else {
                bail!("File with id:{} was not found.", id)
            }
        }

        if let Some(file) = self.file_map.get_mut(id) {
            Ok(file)
        } else {
            bail!("File with id:{} was not found.", id)
        }
    }

    pub fn remove_file(&mut self, id: IdRef) -> Result<(), Error> {
        self.file_map.remove(id);
        if let Some(path) = self.file_paths.remove(id) {
            fs::remove_file(path.as_ref())?;
        }
        Ok(())
    }

    pub fn create_file(&mut self, path: String, name: String) -> Result<Id, Error> {
        let project_path = PathBuf::from_str(&self.info.path)?;
        let dir = project_path.join(path);
        fs::canonicalize(&dir)?;
        if !dir.starts_with(&self.info.path) {
            bail!("Invalid file path. The path must be located in the project.");
        }
        let new_file_path = dir.join(&name);
        File::create(&new_file_path)?;
        mem::swap(self, &mut Self::load(&project_path)?);
        Ok(path_id(&new_file_path))
    }

    pub fn create_dir(&mut self, path: String, name: String) -> Result<(), Error> {
        let project_path = PathBuf::from_str(&self.info.path)?;
        let dir = project_path.join(path);
        fs::canonicalize(&dir)?;
        if !dir.starts_with(&self.info.path) {
            bail!("Invalid file path. The path must be located in the project.");
        }
        let new_dir_path = dir.join(&name);
        fs::create_dir_all(&new_dir_path)?;
        mem::swap(self, &mut Self::load(&project_path)?);
        Ok(())
    }

    pub fn rename_file(&mut self, file_id: Id, new_name: String) -> Result<Id, Error> {
        let project_path = PathBuf::from_str(&self.info.path)?;

        let file_path = self
            .file_paths
            .get(&file_id)
            .ok_or_else(|| anyhow!("File with id {} was not found", file_id))?;

        let file_dir = file_path
            .parent()
            .ok_or_else(|| anyhow!("Failed to get file {} directory.", file_id))?
            .to_path_buf();

        let new_name = file_dir.join(new_name);
        fs::rename(file_path.as_ref(), &new_name)?;
        mem::swap(self, &mut Self::load(&project_path)?);
        Ok(path_id(&new_name))
    }

    pub fn rename_directory(
        &mut self,
        path: String,
        old_name: String,
        new_name: String,
    ) -> Result<(), Error> {
        let project_path = PathBuf::from_str(&self.info.path)?;
        let dir = project_path.join(path);

        let old_dir = dir.join(old_name);
        fs::canonicalize(&old_dir)?;
        if !old_dir.starts_with(&self.info.path) {
            bail!("Invalid file path. The path must be located in the project.");
        }

        let new_dir = dir.join(new_name);
        fs::canonicalize(&new_dir)?;
        if !new_dir.starts_with(&self.info.path) {
            bail!("Invalid file path. The path must be located in the project.");
        }

        fs::rename(old_dir, new_dir)?;
        mem::swap(self, &mut Self::load(&project_path)?);
        Ok(())
    }

    pub fn remove_directory(&mut self, path: String) -> Result<(), Error> {
        let project_path = PathBuf::from_str(&self.info.path)?;
        let dir = project_path.join(path);
        fs::canonicalize(&dir)?;
        if !dir.starts_with(&self.info.path) {
            bail!("Invalid file path. The path must be located in the project.");
        }
        fs::remove_dir_all(&dir)?;
        mem::swap(self, &mut Self::load(&project_path)?);
        Ok(())
    }

    pub fn action(&self, action: ActionType) -> Result<ProjectActionResponse, Error> {
        let args = match action {
            ActionType::Build => &["build"],
            ActionType::Clean => &["clean"],
            ActionType::Test => &["test"],
            ActionType::Check => &["check"],
        };
        let dove_project_path = self.info.path.clone();
        let start = Instant::now();
        let (code, output) = Command::new("dove")
            .args(args)
            .arg("--color=always")
            .current_dir(&dove_project_path)
            .output()
            .map_or_else(
                |err| (1, err.to_string()),
                |out| {
                    let mut cont = if out.status.success() {
                        String::from_utf8(out.stdout).unwrap_or_default()
                    } else {
                        String::from_utf8(out.stderr).unwrap_or_default()
                    };
                    cont = output_processing_path(&cont, &dove_project_path);
                    let duration = start.elapsed();
                    (
                        out.status.code().unwrap_or_default(),
                        format!("{}\nFinished targets in {}s", cont, duration.as_secs_f32()),
                    )
                },
            );

        Ok(ProjectActionResponse {
            content: output,
            code: code as u8,
        })
    }
}

fn load_tree(
    path: &Path,
    exclude_dir: &Path,
    file_map: &mut HashMap<Id, Arc<PathBuf>>,
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

fn output_processing_path(out: &str, project_path: &str) -> String {
    let project_path = PathBuf::from(project_path).parent().map_or_else(
        || project_path.to_string(),
        |path| path.to_string_lossy().to_string(),
    );
    out.split(&project_path)
        .enumerate()
        .map(|(index, part)| {
            if index == 0 || part.len() < 2 {
                return part.to_string();
            }

            let (mut path, text) = if let Some(pos) = part
                .chars()
                .enumerate()
                .find(|(_, char)| char.is_whitespace())
                .map(|(pos, _)| pos)
            {
                (format!("{}", &part[1..pos]), &part[pos..])
            } else {
                (format!("{}", &part[1..]), "")
            };

            let positions = if let Some(pos) = path.find(':') {
                let tmp = &path[pos..]
                    .split(':')
                    .map(|p| p.trim())
                    .filter(|p| p.len() != 0)
                    .filter_map(|p| p.parse::<u32>().ok())
                    .collect::<Vec<u32>>();
                path = { &path[..pos] }.to_string();
                Some((
                    tmp.get(0).cloned().unwrap_or_default(),
                    tmp.get(1).cloned().unwrap_or_default(),
                ))
            } else {
                None
            };

            if !path.ends_with(".toml") && !path.ends_with(".move") {
                return path + text;
            }

            if let Some(pos) = positions {
                format!(
                    r#"[path path="{}" line="{}" char="{}"]{}:{1}:{2}[/path]{}"#,
                    &path, pos.0, pos.1, path, text
                )
            } else {
                format!(r#"[path path="{}"]{}[/path]{}"#, &path, path, text)
            }
        })
        .collect::<Vec<String>>()
        .join("")
}

#[test]
fn test_output_processing_path() {
    let project_path = "/home/user/dove-project";
    assert_eq!(
        "without extension: dove-project/empty text",
        output_processing_path(
            "without extension: /home/user/dove-project/empty text",
            project_path
        )
    );
    assert_eq!(
        r#"File: [path path="dove-project/Dove.toml"]dove-project/Dove.toml[/path]"#,
        output_processing_path("File: /home/user/dove-project/Dove.toml", project_path)
    );
    assert_eq!(
        r#"File: [path path="dove-project/Dove.toml"]dove-project/Dove.toml[/path]
text"#,
        output_processing_path(
            "File: /home/user/dove-project/Dove.toml\ntext",
            project_path
        )
    );
    assert_eq!(
        r#"File: [path path="dove-project/scripts/main.move" line="5" char="49"]dove-project/scripts/main.move:5:49[/path]"#,
        output_processing_path(
            "File: /home/user/dove-project/scripts/main.move:5:49",
            project_path
        )
    );
    assert_eq!(
        r#"File: [path path="dove-project/scripts/main.move" line="15" char="149"]dove-project/scripts/main.move:15:149[/path] text"#,
        output_processing_path(
            "File: /home/user/dove-project/scripts/main.move:15:149 text",
            project_path
        )
    );
    assert_eq!(
        r#"error:

    ┌── [path path="dove-project/scripts/main.move" line="2" char="9"]dove-project/scripts/main.move:2:9[/path] ───
    │
    2 │     use 0x1::Debug;
    │         ^^^^^^^^^^ Invalid 'use'. Unbound module: '0x1::Debug'
    │

    error:

    ┌── [path path="dove-project/scripts/main.move" line="4" char="9"]dove-project/scripts/main.move:4:9[/path] ───
    │
    4 │         Debug::print<u8>(&0);
    │         ^^^^^ Unbound module alias 'Debug'
    │


    Finished targets in 0.018826656s"#,
        output_processing_path(
            "error:

    ┌── /home/user/dove-project/scripts/main.move:2:9 ───
    │
    2 │     use 0x1::Debug;
    │         ^^^^^^^^^^ Invalid 'use'. Unbound module: '0x1::Debug'
    │

    error:

    ┌── /home/user/dove-project/scripts/main.move:4:9 ───
    │
    4 │         Debug::print<u8>(&0);
    │         ^^^^^ Unbound module alias 'Debug'
    │


    Finished targets in 0.018826656s",
            project_path
        )
    );
}
