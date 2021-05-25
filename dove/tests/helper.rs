#![allow(dead_code)]

use std::path::{PathBuf, Path};
use std::fs::{remove_dir_all, read_to_string, create_dir_all};
use dove::cli::execute;
use fs_extra::file::write_all;
use toml::Value;

pub fn project_start(project_name: &str) -> (PathBuf, PathBuf) {
    let tmp_folder = std::env::temp_dir();
    let project_folder = tmp_folder.join(project_name);
    if project_folder.exists() {
        remove_dir_all(&project_folder).unwrap_or_else(|err| {
            panic!(
                "[ERROR] Couldn't delete project directory: {}\r\n[MESSAGE] {}\r\n",
                project_folder.display(),
                err.to_string()
            )
        });
    }
    (tmp_folder, project_folder)
}
pub fn project_start_for_init(project_name: &str) -> PathBuf {
    let (_, project_folder) = project_start(project_name);
    project_remove(&project_folder);

    // Create project directory
    create_dir_all(&project_folder).unwrap_or_else(|err| {
        panic!(
            "Failed to create directory: {}\r\n[ERROR] {}\r\n",
            project_folder.display(),
            err.to_string()
        )
    });
    project_folder
}
pub fn project_start_new_and_build(project_name: &str) -> PathBuf {
    let (base_folder, project_folder) = project_start(project_name);
    project_new_default(&base_folder, &project_folder, project_name);
    project_build(&project_folder).unwrap_or_else(|err| panic!("{}", err));
    project_folder
}
/// $ dove new ###
pub fn project_new_default(base_folder: &Path, project_folder: &Path, project_name: &str) {
    execute_dove_at(&["dove", "new", project_name], &base_folder).unwrap_or_else(|err| {
        panic!("{}", err);
    });
    set_dependencies_local_move_stdlib(&project_folder);
}
/// $ dove new ### -d ### -a ### -r ###
pub fn project_new_with_args(
    base_folder: &Path,
    project_folder: &Path,
    project_name: &str,
    project_dialect: &str,
    project_address: &str,
    project_blockchain_api: &str,
) {
    execute_dove_at(
        &[
            "dove",
            "new",
            project_name,
            "-d",
            project_dialect,
            "-r",
            project_blockchain_api,
            "-a",
            project_address,
        ],
        &base_folder,
    )
    .unwrap_or_else(|err| {
        panic!("{}", err);
    });
    set_dependencies_local_move_stdlib(&project_folder);
}
// @dove build
pub fn project_build(project_folder: &Path) -> Result<String, String> {
    execute_dove_at(&["dove", "build"], &project_folder)
}
pub fn project_remove(project_folder: &Path) {
    if project_folder.exists() {
        remove_dir_all(project_folder).unwrap_or_else(|err| {
            panic!(
                "[ERROR] Couldn't delete project directory: {}\r\n[MESSAGE] {}\r\n",
                project_folder.display(),
                err.to_string()
            )
        });
    }
}
pub fn set_dependencies_local_move_stdlib(project_path: &Path) {
    let move_stdlib = Path::new(".")
        .canonicalize()
        .unwrap()
        .join("resources/test_move_project");
    let mut dove_toml_path = project_path.to_path_buf();
    dove_toml_path.push("Dove.toml");
    let mut toml_value = read_to_string(&dove_toml_path)
        .unwrap()
        .parse::<Value>()
        .unwrap();

    let mut dd = toml::map::Map::new();
    dd.insert(
        "path".to_string(),
        Value::String(move_stdlib.to_str().unwrap().to_string()),
    );
    toml_value
        .get_mut("package")
        .unwrap()
        .as_table_mut()
        .unwrap()
        .insert(
            "dependencies".to_string(),
            Value::Array(vec![Value::Table(dd)]),
        );
    write_all(
        &dove_toml_path,
        toml::to_string(&toml_value).unwrap().as_str(),
    )
    .unwrap();
}
pub fn execute_dove_at(args: &[&str], project_folder: &Path) -> Result<String, String> {
    execute(args, project_folder.to_path_buf()).map_or_else(
        |err| {
            Err(format!(
                "[COMMAND] {}\r\n[FOLDER] {}\r\n[ERROR] {}\r\n",
                &args.join(" "),
                &project_folder.display(),
                err.to_string()
            ))
        },
        |_| {
            Ok(format!(
                "[COMMAND] {}\r\n[FOLDER] {}\r\n",
                &args.join(" "),
                &project_folder.display(),
            ))
        },
    )
}
pub fn check_dove_toml(
    project_folder: &Path,
    project_name: &str,
    dialect: Option<&str>,
    address: Option<&str>,
    api: Option<&str>,
) -> Result<(), String> {
    // Check config
    let dove_toml_string = read_to_string(project_folder.join("Dove.toml"))
        .unwrap()
        .replace(" ", "")
        .replace("\t", "");
    if !dove_toml_string.contains(&format!("name=\"{}\"", project_name)) {
        return Err("Dove.toml: invalid name\r\n".to_string());
    }
    if dialect.is_some()
        && !dove_toml_string.contains(&format!("dialect=\"{}\"", dialect.unwrap()))
    {
        return Err("Dove.toml: invalid dialect\r\n".to_string());
    }
    if address.is_some()
        && !dove_toml_string.contains(&format!("account_address=\"{}\"", address.unwrap()))
    {
        return Err("Dove.toml: invalid account_address\r\n".to_string());
    }
    if api.is_some()
        && !dove_toml_string.contains(&format!("blockchain_api=\"{}\"", api.unwrap()))
    {
        return Err("Dove.toml: invalid blockchain_api\r\n".to_string());
    }
    Ok(())
}
