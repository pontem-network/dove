#![cfg(test)]

use std::fs::{read_to_string};
use dove::cli::execute;
use toml::Value;

mod test_cmd_helper;
use crate::test_cmd_helper::{
    project_remove, project_start, set_dependencies_local_move_stdlib, project_build,
};

/// Create a new move project
/// Correct url: http://demo.ru/api, https://demo.ru/api, http://127.0.0.1/api, http://localhost/api, http://localhost:8080/api
///
/// $ dove new ### -r ###
/// project name: demoproject_35
#[test]
fn valid_api_url() {
    // Project name and path
    let project_name = "demoproject_35";
    let (base_folder, project_folder) = project_start(project_name);
    project_remove(&project_folder);

    for api in &[
        "http://demo.ru/api",
        "https://demo.ru/api",
        "http://127.0.0.1/api",
        "http://localhost/api",
        "http://localhost:8080/api",
    ] {
        // $ dove new demoproject_35 -r ###
        let args = &["dove", "new", &project_name, "-r", api];
        let command_string: String = args.join(" ").to_string();
        execute(args, base_folder.clone()).unwrap_or_else(|err| {
            panic!(
                "[COMMAND] {}\r\n[API] {}\r\n[FOLDER] {}\r\n[ERROR] {}\r\n",
                &command_string,
                api,
                &base_folder.to_str().unwrap(),
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
