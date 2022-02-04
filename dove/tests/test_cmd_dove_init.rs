mod helper;

use crate::helper::{
    pre_start_dove_init, execute_dove_at, delete_project, get_project_name_from_toml,
    get_project_dialect_from_toml, assert_basic_project_dirs_exist,
};

/// Creating a project in an existing folder
/// $ dove init NAME_PROJECT
#[test]
fn test_cmd_dove_init() {
    // Project name and path
    let project_name = "project_init_without_arguments";
    let project_path = pre_start_dove_init(project_name).unwrap();

    execute_dove_at(&["init", project_name], &project_path).unwrap();

    assert_eq!(
        get_project_name_from_toml(&project_path),
        Some(project_name.to_string())
    );
    assert_eq!(get_project_dialect_from_toml(&project_path), None);
    assert!(assert_basic_project_dirs_exist(&project_path).is_ok());

    delete_project(&project_path).unwrap();
}
