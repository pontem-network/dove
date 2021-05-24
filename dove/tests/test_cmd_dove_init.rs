#![cfg(test)]

use std::fs::{read_to_string, create_dir_all};
use toml::Value;

mod test_cmd_helper;
use crate::test_cmd_helper::{
    project_start, project_start_for_init, project_remove, set_dependencies_local_move_stdlib,
    project_build, execute_dove_at, execute_dove_at_wait_fail,
};

/// $ dove init
#[test]
fn test_cmd_dove_init_without_arguments() {
    // Project name and path
    let project_name = "demoproject_36";
    let project_folder = project_start_for_init(project_name);

    execute_dove_at(&project_folder, &["dove", "init"]);
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
/// $ dove init -d ###
#[test]
fn test_cmd_dove_init_with_dialect() {
    // Project name and path
    let project_name = "demoproject_43";
    let project_folder = project_start_for_init(project_name);

    for dialect in &["pont", "diem", "dfinance"] {
        execute_dove_at(&project_folder, &["dove", "init", "-d", dialect]);
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
                .contains(dialect),
            "Dove.toml: invalid dialect",
        );

        set_dependencies_local_move_stdlib(&project_folder);
        project_build(&project_folder);
        project_remove(&project_folder);
    }
}
/// $ dove init -d dfinance -a ###
#[test]
fn test_cmd_dove_init_dfinance_with_address() {
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
        execute_dove_at(
            &project_folder,
            &["dove", "init", "-d", "dfinance", "-a", address],
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
/// $ dove init -d diem -a ###
#[test]
fn test_cmd_dove_init_diem_with_address() {
    // Project name and path
    let project_name = "demoproject_42";
    let (_, project_folder) = project_start(project_name);
    project_remove(&project_folder);

    for address in &["0x1"] {
        // Create project directory
        create_dir_all(&project_folder).unwrap_or_else(|_| {
            panic!(
                "Failed to create directory: {}",
                project_folder.to_str().unwrap_or(" - "),
            )
        });
        execute_dove_at(
            &project_folder,
            &["dove", "init", "-d", "diem", "-a", address],
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
/// $ dove init -d pont -a ###
#[test]
fn test_cmd_dove_init_pont_with_address() {
    // Project name and path
    let project_name = "demoproject_40";
    let (_, project_folder) = project_start(project_name);
    project_remove(&project_folder);

    for address in &["5CdCiQzNRZXWx7wNVCVjPMzGBFpkYHe3WKrGzd6TG97vKbnv", "0x1"] {
        // Create project directory
        create_dir_all(&project_folder).unwrap_or_else(|_| {
            panic!(
                "Failed to create directory: {}",
                project_folder.to_str().unwrap_or(" - "),
            )
        });
        // $ dove init -d pont -a ###
        execute_dove_at(
            &project_folder,
            &["dove", "init", "-d", "pont", "-a", address],
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

/// $ dove init -d pont
#[test]
fn test_cmd_dove_init_pont_with_repo() {
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
        execute_dove_at(&project_folder, &["dove", "init", "-r", api]);
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

/// $ dove init -d incorrectdialect
#[test]
fn itest_cmd_dove_init_ncorrect_dialect() {
    // Project name and path
    let project_name = "demoproject_46";
    let project_folder = project_start_for_init(project_name);

    execute_dove_at_wait_fail(&project_folder, &["dove", "init", "-d", "incorrectdialect"]);
    project_remove(&project_folder);
}

/// $ dove init -r ###
/// @todo Need to add validation on repo
#[test]
#[ignore]
fn test_cmd_dove_init_incorrect_repo() {
    // Project name and path
    let project_name = "demoproject_47";
    let project_folder = project_start_for_init(project_name);

    for api in &[
        "demo",
        "/demo",
        "/demo/api",
        "//demo/api",
        "//demo:8080/api",
        "127.0.0.1/api",
        "ftp://demo.ru/api",
    ] {
        execute_dove_at_wait_fail(&project_folder, &["dove", "init", "-r", api]);
    }
    project_remove(&project_folder);
}
