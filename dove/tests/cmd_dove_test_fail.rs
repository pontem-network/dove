#![cfg(test)]

use std::path::{Path, PathBuf};
use std::fs::{remove_dir_all, read_to_string};
use fs_extra::file::write_all;
use dove::cli::execute;

/// $ dove test
/// project name: demoproject_12
#[test]
fn fail() {
    // Path to dove folder
    let dove_folder = {
        let mut folder = Path::new(".").canonicalize().unwrap();
        if folder.to_str().unwrap().find("dove").is_none() {
            folder.push("dove");
        }
        folder
    };
    // Project name and path
    let project_name = "demoproject_12";
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

    // $ cargo run -- new demoproject_12 -d pont
    // $ dove new demoproject_12 -d pont
    {
        let args = &["dove", "new", project_name];
        let command_string: String = args.join(" ").to_string();
        execute(args, dove_folder.clone()).expect(&format!("[COMMAND] {}", &command_string));
        set_dependencies_local_move_stdlib(&project_folder);
    }

    // $ cargo run -- build
    // $ dove build
    {
        let args = &["dove", "build"];
        let command_string: String = args.join(" ").to_string();
        execute(args, project_folder.clone()).expect(&format!(
            "[COMMAND] {}\r\n[FOLDER] {}",
            &command_string,
            project_folder.to_str().unwrap()
        ));
    }

    // project_folder/tests/test_1.move
    {
        let test_1_path = {
            let mut path = project_folder.clone();
            path.push("tests");
            path.push("test_1.move");
            path
        };
        write_all(
            &test_1_path,
            "script {
                    fun main() {
                        assert((3+2)==4,1);
                    }
                }",
        )
        .unwrap();
    }
    // $ cargo run -- test
    // $ dove test
    {
        let args = &["dove", "test"];
        let command_string: String = args.join(" ").to_string();
        assert!(
            execute(args, project_folder.clone()).is_err(),
            "Expected error\r\n[COMMAND] {}\r\n[FOLDER] {}",
            &command_string,
            project_folder.to_str().unwrap()
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
