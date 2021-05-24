#![cfg(test)]

use std::fs::{read_to_string};
use dove::cli::execute;
use toml::Value;

mod test_cmd_helper;
use crate::test_cmd_helper::{
    project_remove, project_start, set_dependencies_local_move_stdlib, project_build,
};

/// $ dove new demoproject_30 -d diem
/// $ dove build
/// project: demoproject_30
#[test]
fn default() {
    // Path to dove folder, Project name and path
    let project_name = "demoproject_30";
    let (base_folder, project_folder) = project_start(project_name);
    project_remove(&project_folder);

    // $ dove new demoproject_30 -d dfinance
    let args = &["dove", "new", &project_name, "-d", "diem"];
    let command_string: String = args.join(" ").to_string();
    execute(args, base_folder.clone()).unwrap_or_else(|err| {
        panic!(
            "[COMMAND] {}\r\n[FOLDER] {}\r\n[ERROR] {}\r\n",
            &command_string,
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
        package.get("dialect").unwrap().to_string().contains("diem"),
        "Dove.toml: invalid dialect",
    );

    set_dependencies_local_move_stdlib(&project_folder);
    // $ dove build
    project_build(&project_folder);
    project_remove(&project_folder);
}
/// $ dove new demoproject_31 -d diem -a ###
/// $ dove build
/// project: demoproject_31
#[test]
fn with_address() {
    // Path to dove folder, Project name and path
    let project_name = "demoproject_31";
    let (base_folder, project_folder) = project_start(project_name);
    project_remove(&project_folder);

    for address in &["0x1"] {
        // $ dove new demoproject_31 -d diem -a ###
        let args = &["dove", "new", &project_name, "-d", "diem", "-a", address];
        let command_string: String = args.join(" ").to_string();
        execute(args, base_folder.clone()).unwrap_or_else(|err| {
            panic!(
                "[COMMAND] {}\r\n[FOLDER] {}\r\n[ADDRESS] {}\r\n[ERROR] {}\r\n",
                &command_string,
                &base_folder.to_str().unwrap(),
                address,
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
            package.get("dialect").unwrap().to_string().contains("diem"),
            "Dove.toml: invalid dialect",
        );
        assert!(
            package
                .get("account_address")
                .unwrap()
                .to_string()
                .contains(address),
            "Dove.toml: invalid account_address",
        );

        set_dependencies_local_move_stdlib(&project_folder);
        // $ dove build
        project_build(&project_folder);
        project_remove(&project_folder);
    }
}
