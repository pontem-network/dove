#![cfg(test)]

use std::fs::{read_to_string, create_dir_all};
use toml::Value;
use dove::cli::execute;

mod test_cmd_helper;
use crate::test_cmd_helper::{
    project_start, project_remove, project_start_for_init, set_dependencies_local_move_stdlib,
    project_build,
};

/// $ dove init -d dfinance
/// project name: demoproject_43
#[test]
fn default() {
    // Project name and path
    let project_name = "demoproject_43";
    let project_folder = project_start_for_init(project_name);

    // $ dove init -d dfinance
    let args = &["dove", "init", "-d", "dfinance"];
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

/// $ dove init -d dfinance -a ###
/// project name: demoproject_44
#[test]
fn with_address() {
    // Project name and path
    let project_name = "demoproject_44";
    let (_, project_folder) = project_start(project_name);
    project_remove(&project_folder);

    for address in &["0x1", "wallet1me0cdn52672y7feddy7tgcj6j4dkzq2su745vh"] {
        // Create project directory
        create_dir_all(&project_folder).unwrap_or_else(|_| {
            panic!(
                "Failed to create directory: {}",
                project_folder.to_str().unwrap_or(" - "),
            )
        });
        // $ dove init -d dfinance -a ###
        let args = &["dove", "init", "-d", "dfinance", "-a", address];
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
        project_build(&project_folder);
        project_remove(&project_folder);
    }
}
