use dove::tests_helper::*;
/// $ dove new demoproject_25
#[test]
fn test_cmd_dove_new_without_arguments() {
    // Project name and path
    let project_name = "project_new_without_arguments";
    let (base_folder, project_folder) = project_start(project_name);
    project_remove(&project_folder);
    execute_dove_at(&["dove", "new", project_name], &base_folder).unwrap();
    assert_valid_dove_toml(&project_folder, project_name, Some("pont"), None, None);
    assert_basic_project_dirs_exist(&project_folder);
    set_dependencies_local_move_stdlib(&project_folder);
    project_build(&project_folder);
    project_remove(&project_folder);
}

/// $ dove new demoproject_32 -d ###
#[test]
fn test_cmd_dove_new_dialect() {
    // Path to dove folder, Project name and path
    let project_name = "project_new_dialect";
    let (base_folder, project_folder) = project_start(project_name);
    project_remove(&project_folder);
    for dialect in &["pont", "diem", "dfinance"] {
        execute_dove_at(&["dove", "new", &project_name, "-d", dialect], &base_folder).unwrap();
        assert_valid_dove_toml(&project_folder, project_name, Some(dialect), None, None);
        assert_basic_project_dirs_exist(&project_folder);
        set_dependencies_local_move_stdlib(&project_folder);
        project_build(&project_folder);
        project_remove(&project_folder);
    }
}

/// $ dove new demoproject_33 -d dfinance -a ###
#[test]
fn test_cmd_dove_new_difinance_with_address() {
    // Path to dove folder, Project name and path
    let project_name = "project_new_difinance_with_address";
    let (base_folder, project_folder) = project_start(project_name);
    project_remove(&project_folder);
    for address in &["0x1", "wallet1me0cdn52672y7feddy7tgcj6j4dkzq2su745vh"] {
        execute_dove_at(
            &[
                "dove",
                "new",
                &project_name,
                "-d",
                "dfinance",
                "-a",
                address,
            ],
            &base_folder,
        )
        .unwrap();
        assert_valid_dove_toml(
            &project_folder,
            project_name,
            Some("dfinance"),
            Some(address),
            None,
        );
        assert_basic_project_dirs_exist(&project_folder);
        set_dependencies_local_move_stdlib(&project_folder);
        project_build(&project_folder);
        project_remove(&project_folder);
    }
}

/// $ dove new demoproject_31 -d diem -a ###
#[test]
fn test_cmd_dove_new_diem_with_address() {
    // Path to dove folder, Project name and path
    let project_name = "project_new_diem_with_address";
    let (base_folder, project_folder) = project_start(project_name);
    project_remove(&project_folder);
    for address in &["0x1"] {
        execute_dove_at(
            &["dove", "new", &project_name, "-d", "diem", "-a", address],
            &base_folder,
        )
        .unwrap();
        assert_valid_dove_toml(
            &project_folder,
            project_name,
            Some("diem"),
            Some(address),
            None,
        );
        assert_basic_project_dirs_exist(&project_folder);
        set_dependencies_local_move_stdlib(&project_folder);
        project_build(&project_folder);
        project_remove(&project_folder);
    }
}

/// $ dove new demoproject_29 -d pont -a ###
#[test]
fn test_cmd_dove_new_pont_with_address() {
    // Project name and path
    let project_name = "project_new_pont_with_address";
    let (base_folder, project_folder) = project_start(project_name);
    project_remove(&project_folder);
    for address in &["5CdCiQzNRZXWx7wNVCVjPMzGBFpkYHe3WKrGzd6TG97vKbnv", "0x1"] {
        execute_dove_at(
            &["dove", "new", &project_name, "-d", "pont", "-a", address],
            &base_folder,
        )
        .unwrap();
        assert_valid_dove_toml(
            &project_folder,
            project_name,
            Some("pont"),
            Some(address),
            None,
        );
        assert_basic_project_dirs_exist(&project_folder);
        set_dependencies_local_move_stdlib(&project_folder);
        project_build(&project_folder);
        project_remove(&project_folder);
    }
}

/// $ dove new ### -r ###
#[test]
fn test_cmd_dove_new_pont_with_repo() {
    // Project name and path
    let project_name = "project_new_pont_with_repo";
    let (base_folder, project_folder) = project_start(project_name);
    project_remove(&project_folder);
    for api in &[
        "http://demo.ru/api",
        "https://demo.ru/api",
        "http://127.0.0.1/api",
        "http://localhost/api",
        "http://localhost:8080/api",
    ] {
        execute_dove_at(&["dove", "new", &project_name, "-r", api], &base_folder).unwrap();
        assert_valid_dove_toml(&project_folder, project_name, None, None, Some(api));
        assert_basic_project_dirs_exist(&project_folder);
        set_dependencies_local_move_stdlib(&project_folder);
        project_build(&project_folder);
        project_remove(&project_folder);
    }
}

/// $ dove new demoproject_38 -r ###
#[test]
fn test_cmd_dove_new_pont_with_incorrect_repo() {
    // Project name and path
    let project_name = "project_new_pont_with_incorrect_repo";
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
        assert!(execute_dove_bin_at(
            env!("CARGO_BIN_EXE_dove"),
            &["dove", "new", &project_name, "-r", api],
            &base_folder
        )
        .is_err());
    }
}

/// $ dove new demoproject_37 -d incorrectdialect
#[test]
fn test_cmd_dove_new_incorrect_dialect() {
    // Project name and path
    let project_name = "project_new_incorrect_dialect";
    let (base_folder, project_folder) = project_start(project_name);
    project_remove(&project_folder);
    assert!(execute_dove_at(
        &["dove", "new", &project_name, "-d", "incorrectdialect"],
        &base_folder,
    )
    .is_err());
}
