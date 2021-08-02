use dove::tests_helper::*;
/// $ dove init
#[test]
fn test_cmd_dove_init_without_arguments() {
    // Project name and path
    let project_name = "project_init_without_arguments";
    let project_folder = project_start_for_init(project_name);
    execute_dove_at(&["dove", "init"], &project_folder).unwrap();

    assert_valid_dove_toml(&project_folder, project_name, None, None, None);
    assert_basic_project_dirs_exist(&project_folder);
    set_dependencies_local_move_stdlib(&project_folder);
    project_build(&project_folder);
    project_remove(&project_folder);
}

/// $ dove init -d ###
#[test]
fn test_cmd_dove_init_with_dialect() {
    // Project name and path
    let project_name = "project_init_with_dialect";
    for dialect in &["pont", "diem", "dfinance"] {
        let project_folder = project_start_for_init(project_name);
        execute_dove_at(&["dove", "init", "-d", dialect], &project_folder).unwrap();
        assert_valid_dove_toml(&project_folder, project_name, Some(dialect), None, None);
        assert_basic_project_dirs_exist(&project_folder);
        set_dependencies_local_move_stdlib(&project_folder);
        project_build(&project_folder);
        project_remove(&project_folder);
    }
}

/// $ dove init -d dfinance -a ###
#[test]
fn test_cmd_dove_init_dfinance_with_address() {
    // Project name and path
    let project_name = "project_init_dfinance_with_address";
    for address in &["0x1", "wallet1me0cdn52672y7feddy7tgcj6j4dkzq2su745vh"] {
        let project_folder = project_start_for_init(project_name);
        execute_dove_at(
            &["dove", "init", "-d", "dfinance", "-a", address],
            &project_folder,
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

/// $ dove init -d diem -a ###
#[test]
fn test_cmd_dove_init_diem_with_address() {
    // Project name and path
    let project_name = "project_init_diem_with_address";
    for address in &["0x1"] {
        let project_folder = project_start_for_init(project_name);
        execute_dove_at(
            &["dove", "init", "-d", "diem", "-a", address],
            &project_folder,
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

/// $ dove init -d pont -a ###
#[test]
fn test_cmd_dove_init_pont_with_address() {
    // Project name and path
    let project_name = "project_init_pont_with_address";
    for address in &["5CdCiQzNRZXWx7wNVCVjPMzGBFpkYHe3WKrGzd6TG97vKbnv", "0x1"] {
        let project_folder = project_start_for_init(project_name);
        // $ dove init -d pont -a ###
        execute_dove_at(
            &["dove", "init", "-d", "pont", "-a", address],
            &project_folder,
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

/// $ dove init -d pont
#[test]
fn test_cmd_dove_init_pont_with_repo() {
    // Project name and path
    let project_name = "project_init_pont_with_repo";
    for api in &[
        "http://demo.ru/api",
        "https://demo.ru/api",
        "http://127.0.0.1/api",
        "http://localhost/api",
        "http://localhost:8080/api",
    ] {
        let project_folder = project_start_for_init(project_name);
        execute_dove_at(&["dove", "init", "-r", api], &project_folder).unwrap();
        assert_valid_dove_toml(&project_folder, project_name, None, None, Some(api));
        assert_basic_project_dirs_exist(&project_folder);
        set_dependencies_local_move_stdlib(&project_folder);
        project_build(&project_folder);
        project_remove(&project_folder);
    }
}

/// $ dove init -d incorrectdialect
#[test]
fn itest_cmd_dove_init_incorrect_dialect() {
    // Project name and path
    let project_name = "project_itest_cmd_dove_init_incorrect_dialect";
    let project_folder = project_start_for_init(project_name);
    assert!(
        execute_dove_at(&["dove", "init", "-d", "incorrectdialect"], &project_folder).is_err()
    );
    project_remove(&project_folder);
}

/// $ dove init -r ###
#[test]
fn test_cmd_dove_init_incorrect_repo() {
    // Project name and path
    let project_name = "project_init_incorrect_repo";
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
        assert!(execute_dove_bin_at(
            env!("CARGO_BIN_EXE_dove"),
            &["dove", "init", "-r", api],
            &project_folder
        )
        .is_err());
    }
    project_remove(&project_folder);
}
