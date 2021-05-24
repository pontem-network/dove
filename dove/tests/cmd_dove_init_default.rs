#![cfg(test)]

use std::fs::{read_to_string};
use toml::Value;
use dove::cli::execute;

mod test_cmd_helper;
use crate::test_cmd_helper::{
    project_start_for_init, project_remove, set_dependencies_local_move_stdlib, project_build,
};

/// $ dove init
/// project name: demoproject_36
#[test]
fn default() {
    // Project name and path
    let project_name = "demoproject_36";
    let project_folder = project_start_for_init(project_name);

    // $ dove init
    let args = &["dove", "init"];
    let command_string: String = args.join(" ").to_string();
    execute(args, project_folder.clone()).unwrap_or_else(|err| {
        panic!(
            "[COMMAND] {}\r\n[FOLDER] {}\r\n[ERROR] {}\r\n",
            &command_string,
            project_folder.to_str().unwrap(),
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

    set_dependencies_local_move_stdlib(&project_folder);
    // $ dove build
    project_build(&project_folder);
    project_remove(&project_folder);
}
