#![cfg(test)]

use std::path::{Path, PathBuf};
use std::process::{Command};
use std::fs::{remove_dir_all, read_to_string};
use fs_extra::file::write_all;

/// $ dove run rdemo.move
/// project name: demoproject_6
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
    let project_name = "demoproject_6";
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

    // $ cargo run -- new demoproject_6 -d pont
    // $ dove new demoproject_6 -d pont
    {
        let mut dove_new = Command::new("cargo");
        dove_new
            .args(&["run", "--", "new", project_name])
            .args(&["-d", "pont"])
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

    // project_folder/modules/mdemo.move
    {
        let module_path = {
            let mut path = project_folder.clone();
            path.push("modules");
            path.push("mdemo.move");
            path
        };
        write_all(
            &module_path,
            "address 0x1 {
                    module DemoModule {
                        public fun value(): u8 {
                            12
                        }
                    }
                }",
        )
        .unwrap();
    }
    // project_folder/scripts/demo.move
    {
        let script_path = {
            let mut path = project_folder.clone();
            path.push("scripts");
            path.push("rdemo.move");
            path
        };
        write_all(
            &script_path,
            "script {
                    use 0x1::DemoModule;
                    fun main() {
                        let _value = DemoModule::value();
                    }
                }",
        )
        .unwrap();
    }
    // $ cargo run -- run rdemo.move
    // $ dove run rdemo.move
    {
        let mut dove_run = Command::new("cargo");
        dove_run
            .args(&["run", "--", "run", "rdemo.move"])
            .current_dir(&project_folder);
        let command_string = format!("{:?} ", dove_run).replace("\"", "");
        let result = dove_run
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
            stdout.contains("main ...... ok"),
            "[ERROR] Command: {}. \r\nMessage: {}",
            command_string,
            stdout
        );
    }
    remove_dir_all(&project_folder).expect(&format!(
        "[ERROR] Couldn't delete project directory: {}",
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
