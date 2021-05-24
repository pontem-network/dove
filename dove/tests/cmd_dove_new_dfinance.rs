#![cfg(test)]

mod test_cmd_helper;
use crate::test_cmd_helper::{
    project_start, project_remove, project_build, set_dependencies_local_move_stdlib,
};

use std::fs::{read_to_string};
use dove::cli::execute;
use toml::Value;

/// $ dove new demoproject_32 -d dfinance
/// project: demoproject_32
#[test]
fn default() {
    // Path to dove folder, Project name and path
    let project_name = "demoproject_32";
    let (base_folder, project_folder) = project_start(project_name);
    project_remove(&project_folder);

    // $ dove new demoproject_32 -d dfinance
    let args = &["dove", "new", &project_name, "-d", "dfinance"];
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
        package
            .get("dialect")
            .unwrap()
            .to_string()
            .contains("dfinance"),
        "Dove.toml: invalid dialect",
    );

    set_dependencies_local_move_stdlib(&project_folder);
    // $ dove build
    project_build(&project_folder);
    project_remove(&project_folder);
}

/// $ dove new demoproject_33 -d dfinance -a ###
/// $ dove build
/// project: demoproject_33
#[test]
fn with_address() {
    // Path to dove folder, Project name and path
    let project_name = "demoproject_33";
    let (base_folder, project_folder) = project_start(project_name);
    project_remove(&project_folder);

    for address in &["0x1", "wallet1me0cdn52672y7feddy7tgcj6j4dkzq2su745vh"] {
        // $ dove new demoproject_33 -d dfinance -a ###
        let args = &[
            "dove",
            "new",
            &project_name,
            "-d",
            "dfinance",
            "-a",
            address,
        ];
        let command_string: String = args.join(" ").to_string();
        execute(args, base_folder.clone()).unwrap_or_else(|err| {
            panic!(
                "[ADDRESS] {}\r\n[COMMAND] {}\r\n[FOLDER] {}\r\n[ERROR] {}\r\n",
                address,
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
            package
                .get("dialect")
                .unwrap()
                .to_string()
                .contains("dfinance"),
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
        test_cmd_helper::project_build(&project_folder);
        project_remove(&project_folder);
    }
}
