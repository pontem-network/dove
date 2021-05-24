#![cfg(test)]

use std::fs::{read_to_string};
use toml::Value;

mod test_cmd_helper;
use crate::test_cmd_helper::{
    project_remove, project_start, project_build, set_dependencies_local_move_stdlib,
    execute_dove_at, execute_dove_at_wait_fail,
};
/// $ dove new demoproject_25
#[test]
fn test_cmd_dove_new_without_arguments() {
    // Project name and path
    let project_name = "demoproject_25";
    let (base_folder, project_folder) = project_start(project_name);
    project_remove(&project_folder);

    execute_dove_at(&base_folder, &["dove", "new", project_name]);

    // Check config
    let package = read_to_string(project_folder.join("Dove.toml"))
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
    project_build(&project_folder);
    project_remove(&project_folder);
}
/// $ dove new demoproject_32 -d ###
#[test]
fn test_cmd_dove_new_dialect() {
    // Path to dove folder, Project name and path
    let project_name = "demoproject_32";
    let (base_folder, project_folder) = project_start(project_name);
    project_remove(&project_folder);

    for dialect in &["pont", "diem", "dfinance"] {
        execute_dove_at(&base_folder, &["dove", "new", &project_name, "-d", dialect]);
        let package = read_to_string(project_folder.join("Dove.toml"))
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
                .contains(dialect),
            "Dove.toml: invalid dialect",
        );

        set_dependencies_local_move_stdlib(&project_folder);
        project_build(&project_folder);
        project_remove(&project_folder);
    }
}
/// $ dove new demoproject_33 -d dfinance -a ###
#[test]
fn test_cmd_dove_new_difinance_with_address() {
    // Path to dove folder, Project name and path
    let project_name = "demoproject_33";
    let (base_folder, project_folder) = project_start(project_name);
    project_remove(&project_folder);

    for address in &["0x1", "wallet1me0cdn52672y7feddy7tgcj6j4dkzq2su745vh"] {
        execute_dove_at(
            &base_folder,
            &[
                "dove",
                "new",
                &project_name,
                "-d",
                "dfinance",
                "-a",
                address,
            ],
        );
        // Check config
        let package = read_to_string(project_folder.join("Dove.toml"))
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
        project_build(&project_folder);
        project_remove(&project_folder);
    }
}
/// $ dove new demoproject_31 -d diem -a ###
#[test]
fn test_cmd_dove_new_diem_with_address() {
    // Path to dove folder, Project name and path
    let project_name = "demoproject_31";
    let (base_folder, project_folder) = project_start(project_name);
    project_remove(&project_folder);

    for address in &["0x1"] {
        execute_dove_at(
            &base_folder,
            &["dove", "new", &project_name, "-d", "diem", "-a", address],
        );
        // Check config
        let package = read_to_string(project_folder.join("Dove.toml"))
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
        project_build(&project_folder);
        project_remove(&project_folder);
    }
}
/// $ dove new demoproject_29 -d pont -a ###
#[test]
fn test_cmd_dove_new_pont_with_address() {
    // Project name and path
    let project_name = "demoproject_29";
    let (base_folder, project_folder) = project_start(project_name);
    project_remove(&project_folder);

    for address in &["5CdCiQzNRZXWx7wNVCVjPMzGBFpkYHe3WKrGzd6TG97vKbnv", "0x1"] {
        execute_dove_at(
            &base_folder,
            &["dove", "new", &project_name, "-d", "pont", "-a", address],
        );
        // Check config
        let package = read_to_string(project_folder.join("Dove.toml"))
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
                .get("account_address")
                .unwrap()
                .to_string()
                .contains(address),
            "Dove.toml: invalid account_address",
        );

        set_dependencies_local_move_stdlib(&project_folder);
        project_build(&project_folder);
        project_remove(&project_folder);
    }
}
/// $ dove new ### -r ###
#[test]
fn test_cmd_dove_new_pont_with_repo() {
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
        execute_dove_at(&base_folder, &["dove", "new", &project_name, "-r", api]);
        // Check config
        let package = read_to_string(project_folder.join("Dove.toml"))
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
        project_build(&project_folder);
        project_remove(&project_folder);
    }
}

/// $ dove new demoproject_38 -r ###
/// @todo Need to add validation on repo
#[test]
#[ignore]
fn test_cmd_dove_new_pont_with_incorrect_repo() {
    // Project name and path
    let project_name = "demoproject_38";
    let (base_folder, project_folder) = project_start(project_name);
    project_remove(&project_folder);

    for api in &[
        "demo",
        "/demo",
        "/demo/api",
        "//demo/api",
        "//demo:8080/api",
        "127.0.0.1/api",
        "ftp://demo.ru/api",
    ] {
        // $ dove new demoproject_35 -r ###
        execute_dove_at_wait_fail(&base_folder, &["dove", "new", &project_name, "-r", api]);
    }
}

/// $ dove new demoproject_37 -d incorrectdialect
#[test]
fn test_cmd_dove_new_incorrect_dialect() {
    // Project name and path
    let project_name = "demoproject_37";
    let (base_folder, project_folder) = project_start(project_name);
    project_remove(&project_folder);

    execute_dove_at_wait_fail(
        &base_folder,
        &["dove", "new", &project_name, "-d", "incorrectdialect"],
    );
}
