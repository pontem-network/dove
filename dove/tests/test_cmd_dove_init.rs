use std::fs::{create_dir_all};
mod helper;
use crate::helper::{
    project_start, project_start_for_init, project_remove, set_dependencies_local_move_stdlib,
    project_build, execute_dove_at, check_dove_toml,
};

/// $ dove init
#[test]
fn test_cmd_dove_init_without_arguments() {
    // Project name and path
    let project_name = "demoproject_36";
    let project_folder = project_start_for_init(project_name);

    execute_dove_at(&["dove", "init"], &project_folder).unwrap_or_else(|err| {
        panic!("{}", err);
    });
    check_dove_toml(&project_folder, project_name, None, None, None)
        .unwrap_or_else(|err| panic!("{}", err));

    set_dependencies_local_move_stdlib(&project_folder);
    project_build(&project_folder).unwrap_or_else(|err| panic!("{}", err));
    project_remove(&project_folder);
}
/// $ dove init -d ###
#[test]
fn test_cmd_dove_init_with_dialect() {
    // Project name and path
    let project_name = "demoproject_43";
    let project_folder = project_start_for_init(project_name);

    for dialect in &["pont", "diem", "dfinance"] {
        execute_dove_at(&["dove", "init", "-d", dialect], &project_folder).unwrap_or_else(
            |err| {
                panic!("{}", err);
            },
        );
        check_dove_toml(&project_folder, project_name, Some(dialect), None, None)
            .unwrap_or_else(|err| panic!("{}", err));

        set_dependencies_local_move_stdlib(&project_folder);
        project_build(&project_folder).unwrap_or_else(|err| panic!("{}", err));
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
            panic!("Failed to create directory: {}", project_folder.display(),)
        });
        execute_dove_at(
            &["dove", "init", "-d", "dfinance", "-a", address],
            &project_folder,
        )
        .unwrap_or_else(|err| {
            panic!("{}", err);
        });
        check_dove_toml(
            &project_folder,
            &project_name,
            Some("dfinance"),
            Some(address),
            None,
        )
        .unwrap_or_else(|err| panic!("{}", err));

        set_dependencies_local_move_stdlib(&project_folder);
        project_build(&project_folder).unwrap_or_else(|err| panic!("{}", err));
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
            panic!("Failed to create directory: {}", project_folder.display(),)
        });
        execute_dove_at(
            &["dove", "init", "-d", "diem", "-a", address],
            &project_folder,
        )
        .unwrap_or_else(|err| {
            panic!("{}", err);
        });
        check_dove_toml(
            &project_folder,
            &project_name,
            Some("diem"),
            Some(address),
            None,
        )
        .unwrap_or_else(|err| panic!("{}", err));

        set_dependencies_local_move_stdlib(&project_folder);
        project_build(&project_folder).unwrap_or_else(|err| panic!("{}", err));
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
            panic!("Failed to create directory: {}", project_folder.display(),)
        });
        // $ dove init -d pont -a ###
        execute_dove_at(
            &["dove", "init", "-d", "pont", "-a", address],
            &project_folder,
        )
        .unwrap_or_else(|err| {
            panic!("{}", err);
        });
        check_dove_toml(
            &project_folder,
            &project_name,
            Some("pont"),
            Some(address),
            None,
        )
        .unwrap_or_else(|err| panic!("{}", err));

        set_dependencies_local_move_stdlib(&project_folder);
        project_build(&project_folder).unwrap_or_else(|err| panic!("{}", err));
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
            panic!("Failed to create directory: {}", project_folder.display(),)
        });
        execute_dove_at(&["dove", "init", "-r", api], &project_folder).unwrap_or_else(|err| {
            panic!("{}", err);
        });
        check_dove_toml(&project_folder, &project_name, None, None, Some(api))
            .unwrap_or_else(|err| panic!("{}", err));

        set_dependencies_local_move_stdlib(&project_folder);
        project_build(&project_folder).unwrap_or_else(|err| panic!("{}", err));
        project_remove(&project_folder);
    }
}

/// $ dove init -d incorrectdialect
#[test]
fn itest_cmd_dove_init_incorrect_dialect() {
    // Project name and path
    let project_name = "demoproject_46";
    let project_folder = project_start_for_init(project_name);

    assert!(
        execute_dove_at(&["dove", "init", "-d", "incorrectdialect"], &project_folder,).is_err()
    );
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
        assert!(execute_dove_at(&["dove", "init", "-r", api], &project_folder,).is_err());
    }
    project_remove(&project_folder);
}
