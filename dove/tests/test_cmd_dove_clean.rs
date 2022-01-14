mod helper;

use crate::helper::{new_demo_project, delete_project, build, execute_dove_at};

/// $ dove clean
#[test]
fn test_cmd_dove_clean() {
    let project_name = "project_clean";
    let project_dir = new_demo_project(project_name, false).unwrap();
    build(&project_dir).unwrap();

    assert!(project_dir.join("build").exists());
    execute_dove_at(&["clean"], &project_dir).unwrap();
    assert!(!project_dir.join("build").exists());

    delete_project(&project_dir).unwrap();
}
