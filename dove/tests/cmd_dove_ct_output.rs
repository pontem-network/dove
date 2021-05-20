#![cfg(test)]

use std::path::{Path, PathBuf};
use std::process::{Command};
use std::fs::{remove_dir_all, read_to_string, remove_file};
use fs_extra::file::write_all;

/// $ dove ct -o ###
/// project name: demoproject_21
#[test]
fn output() {
    // Path to dove folder
    let dove_folder = {
        let mut folder = Path::new(".").canonicalize().unwrap();
        if folder.to_str().unwrap().find("dove").is_none() {
            folder.push("dove");
        }
        folder
    };
    // Project name and path
    let project_name = "demoproject_21";
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

    // $ cargo run -- new demoproject_21
    // $ dove new demoproject_21
    {
        let mut dove_new = Command::new("cargo");
        dove_new
            .args(&["run", "--", "new", project_name])
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
    }

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

    // $ cargo run -- ct -o z
    // $ dove ct -o z
    {
        // project_folder/scripts/sdemo.move
        {
            let mut path = project_folder.clone();
            path.push("scripts");
            path.push("sdemo.move");
            write_all(
                &path,
                "script {
                        fun main() {
                            assert((1+3)==4,1);
                        }
                    }",
            )
            .unwrap();
        }
        let mut dove_ct = Command::new("cargo");
        dove_ct
            .args(&["run", "--", "ct", "-o", "z"])
            .current_dir(&project_folder);
        let command_string = format!("{:?} ", dove_ct).replace("\"", "");
        let result = dove_ct
            .output()
            .expect(&format!("[RUN]: {}\r\n", command_string));
        let code = result.status.code().unwrap();
        assert_eq!(
            0,
            code,
            "[ERROR] Command: {}\r\nCode: {}\r\nError: {}\r\nOut: {}",
            command_string,
            code,
            String::from_utf8(result.stderr.clone()).unwrap(),
            String::from_utf8(result.stdout.clone()).unwrap(),
        );

        let tx_path = {
            let mut file = project_folder.clone();
            file.push("target");
            file.push("transactions");
            file.push("z.mvt");
            file
        };

        assert!(
            tx_path.exists(),
            "Transaction not found: {}\r\n[Command] {}",
            tx_path.to_str().unwrap(),
            &command_string,
        );
        remove_file(&tx_path).unwrap();
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
