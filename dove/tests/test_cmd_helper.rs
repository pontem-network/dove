#![cfg(test)]
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
        remove_dir_all(&project_folder).unwrap_or_else(|_| {
            panic!(
                "[ERROR] Couldn't delete project directory: {}",
                project_folder.display()
            )
        });
    }
    (tmp_folder, project_folder)
}
pub fn project_start_for_init(project_name: &str) -> PathBuf {
    let (_, project_folder) = project_start(project_name);
    project_remove(&project_folder);

    // Create project directory
    create_dir_all(&project_folder).unwrap_or_else(|_| {
        panic!(
            "Failed to create directory: {}",
            project_folder.to_str().unwrap_or(" - "),
        )
    });
    project_folder
}
pub fn project_start_nb(project_name: &str) -> PathBuf {
    let (base_folder, project_folder) = project_start(project_name);
    project_new_default(&base_folder, &project_folder, project_name);
    project_build(&project_folder);
    project_folder
}
/// $ dove new ###
pub fn project_new_default(base_folder: &Path, project_folder: &Path, project_name: &str) {
    let args = &["dove", "new", project_name];
    let command_string: String = args.join(" ");
    execute(args, base_folder.to_path_buf()).unwrap_or_else(|err| {
        panic!(
            "[COMMAND] {}\r\n[FOLDER] {}\r\n[ERROR] {}\r\n",
            &command_string,
            base_folder.display(),
            err.to_string()
        )
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
    let args = &[
        "dove",
        "new",
        project_name,
        "-d",
        project_dialect,
        "-r",
        project_blockchain_api,
        "-a",
        project_address,
    ];
    let command_string: String = args.join(" ");
    execute(args, base_folder.to_path_buf()).unwrap_or_else(|err| {
        panic!(
            "[COMMAND] {}\r\n[FOLDER] {}\r\n[ERROR] {}\r\n",
            &command_string,
            base_folder.display(),
            err.to_string()
        )
    });
    set_dependencies_local_move_stdlib(&project_folder);
}
// @dove build
pub fn project_build(project_folder: &Path) {
    let args = &["dove", "build"];
    let command_string: String = args.join(" ");
    execute(args, project_folder.to_path_buf()).unwrap_or_else(|err| {
        panic!(
            "[COMMAND] {}\r\n[FOLDER] {}\r\n[ERROR] {}\r\n",
            &command_string,
            project_folder.display(),
            err.to_string()
        )
    });
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
    {
        let v = toml_value
            .get_mut("package")
            .unwrap()
            .get_mut("dependencies")
            .unwrap()
            .as_array_mut()
            .unwrap();
        v.clear();
        let mut dd = toml::map::Map::new();
        dd.insert(
            "path".to_string(),
            Value::String(move_stdlib.to_str().unwrap().to_string()),
        );
        v.push(Value::Table(dd));
    }
    write_all(
        &dove_toml_path,
        toml::to_string(&toml_value).unwrap().as_str(),
    )
    .unwrap();
}

pub fn execute_dove_at(project_folder: &Path, args: &[&str]) {
    execute(args, project_folder.to_path_buf()).unwrap_or_else(|err| {
        panic!(
            "[COMMAND] {}\r\n[FOLDER] {}\r\n[ERROR] {}\r\n",
            &args.join(" "),
            &project_folder.display(),
            err.to_string()
        )
    });
}
pub fn execute_dove_at_wait_fail(project_folder: &Path, args: &[&str]) {
    assert!(
        execute(args, project_folder.to_path_buf()).is_err(),
        "Expected error\r\n[COMMAND] {}\r\n[FOLDER] {}\r\n",
        &args.join(" "),
        project_folder.display()
    );
}
