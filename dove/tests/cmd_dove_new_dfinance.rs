#![cfg(test)]
mod test_cmd_helper;

use std::fs::{read_to_string};
use dove::cli::execute;
use toml::Value;
use crate::test_cmd_helper::{project_remove, project_build, set_dependencies_local_move_stdlib};

/// $ dove new demoproject_32 -d dfinance
/// project: demoproject_32
#[test]
fn default() {
    // Path to dove folder, Project name and path
    let project_name = "demoproject_32";
    let (dove_folder, project_folder) = test_cmd_helper::startup(project_name);

    // $ dove new demoproject_32 -d dfinance
    let args = &["dove", "new", &project_name, "-d", "dfinance"];
    let command_string: String = args.join(" ").to_string();
    execute(args, dove_folder.clone())
        .unwrap_or_else(|_| panic!("[COMMAND] {}", &command_string));
    set_dependencies_local_move_stdlib(&project_folder);

    // Check config
    let mut path_toml = project_folder.clone();
    path_toml.push("Dove.toml");

    let package = read_to_string(path_toml)
        .unwrap()
        .parse::<Value>()
        .unwrap()
        .get("package")
        .unwrap()
        .clone();

    assert!(
        package
            .get("name")
            .expect("[ERROR] Dove.toml - name not found")
            .to_string()
            .contains(project_name),
        "Dove.toml: invalid name",
    );
    assert!(
        package
            .get("dialect")
            .expect("[ERROR] Dove.toml - dialect not found")
            .to_string()
            .contains("dfinance"),
        "Dove.toml: invalid dialect",
    );

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
    let (dove_folder, project_folder) = test_cmd_helper::startup(project_name);

    for address in &["0x1", "wallet1me0cdn52672y7feddy7tgcj6j4dkzq2su745vh"] {
        // $ dove new demoproject_33 -d dfinance -a ###
        {
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
            execute(args, dove_folder.clone()).unwrap_or_else(|_| {
                panic!("[COMMAND] {}\r\n[ADDRESS] {}", &command_string, address)
            });
            set_dependencies_local_move_stdlib(&project_folder);
        }
        // Check config
        let mut path_toml = project_folder.clone();
        path_toml.push("Dove.toml");

        let package = read_to_string(path_toml)
            .unwrap()
            .parse::<Value>()
            .unwrap()
            .get("package")
            .unwrap()
            .clone();
        assert!(
            package
                .get("name")
                .expect("[ERROR] Dove.toml - name not found")
                .to_string()
                .contains(project_name),
            "Dove.toml: invalid name",
        );
        assert!(
            package
                .get("dialect")
                .expect("[ERROR] Dove.toml - dialect not found")
                .to_string()
                .contains("dfinance"),
            "Dove.toml: invalid dialect",
        );
        assert!(
            package
                .get("account_address")
                .expect("[ERROR] Dove.toml - account_address not found ")
                .to_string()
                .contains(address),
            "Dove.toml: invalid account_address",
        );
        // $ dove build
        test_cmd_helper::project_build(&project_folder);
        project_remove(&project_folder);
    }
}
