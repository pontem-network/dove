#![allow(dead_code)]

use std::path::{PathBuf, Path};
use std::fs::{remove_dir_all, create_dir};
use anyhow::{Result, ensure};

/// get tmp_folder, project_folder and remove project folder if exist
pub fn pre_start(project_name: &str) -> Result<(PathBuf, PathBuf)> {
    let tmp_folder = std::env::temp_dir();
    let project_folder = tmp_folder.join(project_name);
    delete_project(&project_folder)?;
    Ok((tmp_folder, project_folder))
}
/// get tmp_folder, project_folder and remove project folder if exist
pub fn pre_start_init(project_name: &str) -> Result<PathBuf> {
    let tmp_folder = std::env::temp_dir();
    let project_folder = tmp_folder.join(project_name);
    delete_project(&project_folder)?;
    create_dir(&project_folder)?;
    Ok(project_folder)
}

/// remove project
pub fn delete_project(project_path: &Path) -> Result<()> {
    if project_path.exists() {
        remove_dir_all(project_path)?;
    }
    Ok(())
}

/// run bin dove
pub fn execute_dove_at(args: &[&str], project_path: &Path) -> Result<String> {
    ensure!(
        project_path.exists(),
        "Project folder {:?} does not exist",
        project_path.display()
    );

    let output = std::process::Command::new(env!("CARGO_BIN_EXE_dove"))
        .current_dir(project_path)
        .args(args)
        .output()?;

    ensure!(
        output.status.success(),
        "Command {:?} failed with code {}. \n Output: \n{}",
        args,
        output.status,
        String::from_utf8(output.stderr).unwrap_or_default()
    );
    Ok(String::from_utf8(output.stdout)?)
}

/// Get the project name from "Move.toml"
pub fn get_project_name_from_toml(project_path: &Path) -> Option<String> {
    let move_toml_path = project_path.join("Move.toml");
    if !move_toml_path.exists() {
        return None;
    }
    let move_toml_content = std::fs::read_to_string(&move_toml_path).ok()?;
    let move_toml = toml::from_str::<toml::Value>(&move_toml_content).ok()?;
    move_toml
        .get("package")
        .and_then(|pack| pack.get("name"))
        .and_then(|name| name.as_str().map(|t| t.to_string()))
}

/// Get dialect name from "Move.toml"
pub fn get_project_dialect_from_toml(project_path: &Path) -> Option<String> {
    let move_toml_path = project_path.join("Move.toml");
    if !move_toml_path.exists() {
        return None;
    }
    let move_toml_content = std::fs::read_to_string(&move_toml_path).ok()?;
    let move_toml = toml::from_str::<toml::Value>(&move_toml_content).ok()?;
    move_toml
        .get("package")
        .and_then(|pack| pack.get("dialect"))
        .and_then(|name| name.as_str().map(|t| t.to_string()))
}

/// Get account address from "Move.toml"
pub fn get_account_address_from_toml(project_path: &Path) -> Option<String> {
    let move_toml_path = project_path.join("Move.toml");
    if !move_toml_path.exists() {
        return None;
    }
    let move_toml_content = std::fs::read_to_string(&move_toml_path).ok()?;
    let move_toml = toml::from_str::<toml::Value>(&move_toml_content).ok()?;
    move_toml
        .get("addresses")
        .and_then(|pack| pack.as_table())
        .and_then(|name| {
            name.get("Account")
                .and_then(|value| value.as_str().map(|value| value.to_string().to_lowercase()))
        })
}

/// check basic folders
pub fn assert_basic_project_dirs_exist(project_dir: &Path) -> Result<()> {
    for path in &[
        project_dir.join("sources"),
        project_dir.join("examples"),
        project_dir.join("scripts"),
        project_dir.join("doc_templates"),
        project_dir.join("tests"),
    ] {
        ensure!(
            path.exists(),
            r#"Directory "{}" does not exist"#,
            path.display()
        );
    }
    Ok(())
}
