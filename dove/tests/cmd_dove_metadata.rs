#![cfg(test)]

use std::path::{Path, PathBuf};
use std::process::{Command};
use std::fs::{remove_dir_all, read_to_string};
use fs_extra::file::write_all;

/// $ cargo run -- metadata
/// $ dove metadata
/// project name: demoproject_15
#[test]
fn default() {
    // Path to dove folder
    let dove_folder = {
        let mut folder = Path::new(".").canonicalize().unwrap();
        if folder.to_str().unwrap().find("dove").is_none() {
            folder.push("dove");
        }
        folder
    };
    // Project name and path
    let project_name = "demoproject_15";
    let project_address = "5Csxuy81dNEVYbRA9K7tyHypu7PivHmwCZSKxcbU78Cy2v7v";
    let blockchain_api = "https://localhost/api";
    let project_dialect = "pont";

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

    // $ cargo run -- new demoproject_15 -d pont -a 5Csxuy81dNEVYbRA9K7tyHypu7PivHmwCZSKxcbU78Cy2v7v -r https://localhost/api
    // $ dove new demoproject_15 -d pont
    {
        let mut dove_new = Command::new("cargo");
        dove_new
            .args(&["run", "--", "new", project_name])
            .args(&["-d", project_dialect])
            .args(&["-r", blockchain_api])
            .args(&["-a", project_address])
            .current_dir(&dove_folder);
        let command_string = format!("{:?} ", dove_new).replace("\"", "");
        let result = dove_new
            .output()
            .expect(&format!("[RUN]: {}", &command_string));
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
    }

    // $ cargo run -- metadata
    // $ dove metadata
    {
        let mut dove_metadata = Command::new("cargo");
        dove_metadata
            .args(&["run", "--", "metadata"])
            .current_dir(&project_folder);
        let command_string = format!("{:?} ", dove_metadata).replace("\"", "");
        let result = dove_metadata
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

        let stdout = String::from_utf8(result.stdout).unwrap();
        assert!(
            stdout.contains(&project_name),
            "Not found in metadata name: {}",
            &project_name
        );
        assert!(
            stdout.contains(project_address),
            "Not found in metadata account_address: {}",
            project_address
        );
        assert!(
            stdout.contains(project_dialect),
            "Not found in metadata dialect: {}",
            project_dialect
        );
        assert!(
            stdout.contains(blockchain_api),
            "Not found in metadata blockchain_api: {}",
            blockchain_api
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
