use crate::helpers::{delete_project, new_demo_project};

mod helpers;

#[test]
fn test_allows_different_addresses() {
    let project_name = "project_build_with_addresses";
    let project_folder = new_demo_project(project_name).unwrap();

    helpers::dove(&["build"], project_folder.as_path()).unwrap();

    delete_project(&project_folder).unwrap();
}
