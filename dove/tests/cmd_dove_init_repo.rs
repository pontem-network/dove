#![cfg(test)]

use std::fs::{read_to_string, create_dir_all};
use toml::Value;
use dove::cli::execute;

mod test_cmd_helper;
use crate::test_cmd_helper::{
    project_start, project_remove, project_build, set_dependencies_local_move_stdlib,
};

/// $ dove init -d pont
/// Correct url: http://demo.ru/api, https://demo.ru/api, http://127.0.0.1/api, http://localhost/api, http://localhost:8080/api
/// project name: demoproject_45
#[test]
fn valid_api_url() {
    // Project name and path
    let project_name = "demoproject_45";
    let (_, project_folder) = project_start(project_name);
    project_remove(&project_folder);

    for api in &[
        "http://demo.ru/api",
        "https://demo.ru/api",
        "http://127.0.0.1/api",
        "http://localhost/api",
        "http://localhost:8080/api",
    ] {
        // Create project directory
        create_dir_all(&project_folder).unwrap_or_else(|_| {
            panic!(
                "Failed to create directory: {}",
                project_folder.to_str().unwrap_or(" - "),
            )
        });
        // $ dove init -r ###
        let args = &["dove", "init", "-r", api];
        let command_string: String = args.join(" ").to_string();
        execute(args, project_folder.clone()).unwrap_or_else(|err| {
            panic!(
                "[COMMAND] {}\r\n[FOLDER] {}\r\n[ERROR] {}\r\n",
                &command_string,
                project_folder.display(),
                err.to_string()
            )
        });
        // Check config
        let package = read_to_string(project_folder.clone().join("Dove.toml"))
            .unwrap()
            .parse::<Value>()
            .unwrap()
            .get("package")
            .unwrap()
            .clone();
        assert!(
            package
                .get("name")
                .unwrap()
                .to_string()
                .contains(project_name),
            "Dove.toml: invalid name",
        );
        assert!(
            package.get("dialect").unwrap().to_string().contains("pont"),
            "Dove.toml: invalid dialect",
        );
        assert!(
            package
                .get("blockchain_api")
                .unwrap()
                .to_string()
                .contains(api),
            "Dove.toml: invalid blockchain_api",
        );

        set_dependencies_local_move_stdlib(&project_folder);
        // $ dove build
        project_build(&project_folder);
        project_remove(&project_folder);
    }
}
