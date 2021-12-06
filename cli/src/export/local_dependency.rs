use std::path::PathBuf;
use std::str::FromStr;
use rand::Rng;
use anyhow::Error;
use crate::export::dove_manifest::DepPath;
use crate::export::movetoml::{Dependency, DependencyLocal};
use crate::export::DependenceExport;

pub fn convert_local_dependency(local: &DepPath) -> DependenceExport {
    match create(local) {
        Ok((name, dep)) => DependenceExport {
            name,
            dep,
            error: None,
        },
        Err(err) => DependenceExport {
            name: format!("NoName_{}", rand::thread_rng().gen_range(1, 9999999)),
            dep: Dependency::Local(DependencyLocal {
                local: local.path.clone(),
            }),
            error: Some(err),
        },
    }
}
fn create(local: &DepPath) -> Result<(String, Dependency), Error> {
    let local = PathBuf::from_str(&local.path)?;
    if !local.exists() {
        bail!("Project not found \nLocal: {}", &local.display());
    }

    let move_toml_path = local.join("Move.toml");
    if !move_toml_path.exists() {
        bail!("Move.toml not found \nLocal: {}", &local.display());
    }

    let dep = Dependency::Local(DependencyLocal {
        local: local.to_string_lossy().to_string(),
    });

    let move_toml_content = std::fs::read_to_string(move_toml_path)
        .map_err(|err| anyhow!("{}\nLocal: {}", err.to_string(), &local.display()))?;
    let move_toml = toml::from_str::<toml::Value>(&move_toml_content).map_err(|err| {
        anyhow!(
            "Error when parsing move.tool. \n{}\nLocal: {}",
            err,
            &local.display(),
        )
    })?;

    let name = move_toml
        .get("package")
        .and_then(|pack| pack.get("name"))
        .ok_or_else(|| {
            anyhow!(
                "In Move.tool \"name\" not found \nLocal: {}",
                &local.display()
            )
        })
        .map(|name| name.as_str().unwrap_or("").to_string())?;
    Ok((name, dep))
}
