#![cfg(test)]

use std::path::{Path, PathBuf};
use std::process::{Command};
use std::fs::{remove_dir_all, read_to_string};
use fs_extra::file::write_all;

/// $ cargo run -- clean
/// $ dove clean
/// project name: demoproject_16
#[test]
fn clean() {
    // Path to dove folder
    let dove_folder = {
        let mut folder = Path::new(".").canonicalize().unwrap();
        if folder.to_str().unwrap().find("dove").is_none() {
            folder.push("dove");
        }
        folder
    };
    // Project name and path
    let project_name = "demoproject_16";
    let project_folder = {
        let mut folder = dove_folder.clone();
        folder.push(project_name);
        if folder.exists() {
            remove_dir_all(&folder).expect(&format!(
                "[ERROR] Couldn't delete project directory: {}",
                folder.to_str().unwrap()
            ));
        }
        folder
    };
    // $ cargo run -- new demoproject_16 -d pont
    // $ dove new demoproject_16 -d pont
    {
        let mut dove_new = Command::new("cargo");
        dove_new
            .args(&["run", "--", "new", project_name, "-d", "pont"])
            .current_dir(&dove_folder);
        let command_string = format!("{:?} ", dove_new).replace("\"", "");
        let result = dove_new
            .output()
            .expect(&format!("[RUN]: {}", command_string));
        let code = result.status.code().unwrap();
        assert_eq!(
            0,
            code,
            "[ERROR] Command: {}\r\nCode: {}\r\nMessage: {}\r\n",
            command_string,
            code,
            String::from_utf8(result.stderr).unwrap()
        );
        set_dependencies_local_move_stdlib(&project_folder);
    };
    // $ cargo run -- build
    // $ dove build
    {
        let mut dove_build = Command::new("cargo");
        dove_build
            .args(&["run", "--", "build"])
            .current_dir(&project_folder);
        let command_string = format!("{:?} ", dove_build).replace("\"", "");
        let result = dove_build
            .output()
            .expect(&format!("[RUN]: {}", command_string));
        let code = result.status.code().unwrap();
        assert_eq!(
            0,
            code,
            "[ERROR] Command: {}\r\nCode: {}\r\nMessage: {}\r\n",
            command_string,
            code,
            String::from_utf8(result.stderr).unwrap()
        );
    }

    // $ cargo run -- clean
    // $ dove clean
    {
        let mut dove_build = Command::new("cargo");
        dove_build
            .args(&["run", "--", "clean"])
            .current_dir(&project_folder);
        let command_string = format!("{:?} ", dove_build).replace("\"", "");
        let result = dove_build
            .output()
            .expect(&format!("[RUN]: {}", command_string));
        let code = result.status.code().unwrap();
        assert_eq!(
            0,
            code,
            "[ERROR] Command: {}\r\nCode: {}\r\nMessage: {}\r\n",
            command_string,
            code,
            String::from_utf8(result.stderr).unwrap()
        );

        let mut project_target = project_folder.clone();
        project_target.push("target");

        assert!(
            !project_target.exists(),
            "Directory was not deleted: {}",
            project_target.to_str().unwrap_or(" - ")
        );
    }
    remove_dir_all(&project_folder).expect(&format!(
        "[ERROR] Couldn't delete directory {}",
        project_folder.to_str().unwrap()
    ));
}

fn set_dependencies_local_move_stdlib(project_path: &PathBuf) {
    use toml::Value;

    let mut dove_toml_path = project_path.clone();
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
            Value::String("../tests/move-stdlib".to_string()),
        );
        v.push(Value::Table(dd));
    }
    write_all(
        &dove_toml_path,
        toml::to_string(&toml_value).unwrap().as_str(),
    )
    .unwrap();
}
