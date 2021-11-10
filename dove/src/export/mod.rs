use std::path::Path;
use anyhow::Error;
use crate::export::dove_manifest::Dependence as DoveDependence;
use crate::export::movetoml::Dependency;
use crate::export::git_dependency::convert_git_dependency;
use crate::export::local_dependency::convert_local_dependency;

/// manifest Dove.toml
pub mod dove_manifest;
mod git_dependency;
mod local_dependency;
/// manifest Move.toml
pub mod movetoml;

/// Create internal directories for the "move" project
pub fn create_project_directories(project_dir: &Path) -> Result<(), Error> {
    for path in [
        project_dir.join("sources"),
        project_dir.join("examples"),
        project_dir.join("scripts"),
        project_dir.join("doc_templates"),
        project_dir.join("tests"),
    ]
    .iter()
    .filter(|path| !path.exists())
    {
        std::fs::create_dir_all(path)?;
    }
    Ok(())
}

/// Move files to the "source" folder
pub fn move_modules(project_dir: &Path) -> Result<(), Error> {
    let modules_path = project_dir.join("modules");
    if modules_path.exists() {
        let source_path = project_dir.join("sources");
        modules_path
            .read_dir()?
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter_map(|path| {
                path.file_name()
                    .and_then(|name| name.to_str())
                    .map(|name| name.to_string())
                    .map(|name| (path, name))
            })
            .for_each(
                |(path, name)| {
                    if std::fs::rename(path, source_path.join(name)).is_err() {}
                },
            );
        std::fs::remove_dir_all(modules_path)?;
    }
    Ok(())
}

/// Move.toml dependencies from Dove.toml
pub fn dependency_create_from(dove_dep: &DoveDependence) -> Option<DependenceExport> {
    match dove_dep {
        // Git dependency.
        DoveDependence::Git(git_dep) => Some(convert_git_dependency(git_dep)),
        // Local dependency.
        DoveDependence::Path(local_dep) => Some(convert_local_dependency(local_dep)),
        // Chain dependency.
        DoveDependence::Chain(_) => None,
    }
}

/// The result of converting a dependency from Dove.toml to Move.toml
pub struct DependenceExport {
    /// Package/Dependency Name
    pub name: String,
    /// Move.toml dependency.
    /// If something could not be obtained, the parameter value will be the default
    pub dep: Dependency,
    /// Conversion error
    pub error: Option<Error>,
}
