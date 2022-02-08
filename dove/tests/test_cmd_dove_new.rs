mod helper;

use crate::helper::{
    pre_start_dove_new, execute_dove_at, delete_project, get_project_name_from_toml,
    get_project_dialect_from_toml, assert_basic_project_dirs_exist, pre_start_dove_init,
};

/// Creating a project
/// $ dove new project_new_without_arguments
#[test]
fn test_cmd_dove_new() {
    // Project name and path
    let project_name = "project_new";
    let (base_path, project_path) = pre_start_dove_new(project_name).unwrap();

    execute_dove_at(&["new", project_name], &base_path).unwrap();

    assert_eq!(
        get_project_name_from_toml(&project_path),
        Some(project_name.to_string())
    );
    assert_eq!(get_project_dialect_from_toml(&project_path), None);
    assert!(assert_basic_project_dirs_exist(&project_path).is_ok());

    delete_project(&project_path).unwrap();
}

/// Creating a project in an existing folder
/// $ dove new NAME_PROJECT --cwd
#[test]
fn test_cmd_dove_new_cwd() {
    // Project name and path
    let project_name = "project_new_cwd";
    let project_path = pre_start_dove_init(project_name).unwrap();

    execute_dove_at(&["new", project_name, "--cwd"], &project_path).unwrap();

    assert_eq!(
        get_project_name_from_toml(&project_path),
        Some(project_name.to_string())
    );
    assert_eq!(get_project_dialect_from_toml(&project_path), None);
    assert!(assert_basic_project_dirs_exist(&project_path).is_ok());

    delete_project(&project_path).unwrap();
}
