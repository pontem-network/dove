mod helper;

use crate::helper::{
    execute_dove_at, delete_project, get_project_name_from_toml, get_project_dialect_from_toml,
    create_folder_for_project,
};

/// Creating a project in an existing folder
/// $ dove new NAME_PROJECT --cwd
#[test]
fn test_cmd_dove_init() {
    // Project name and path
    let project_name = "project_new_cwd";
    let project_path = create_folder_for_project(project_name).unwrap();

    execute_dove_at(&["init", project_name], &project_path).unwrap();

    assert_eq!(
        get_project_name_from_toml(&project_path),
        Some(project_name.to_string())
    );
    assert_eq!(get_project_dialect_from_toml(&project_path), None);

    delete_project(&project_path).unwrap();
}
