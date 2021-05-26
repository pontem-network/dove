mod helper;
use crate::helper::{execute_dove_at, project_start_new_and_build, project_remove};

/// $ dove clean
#[test]
fn test_cmd_dove_clean() {
    // Path to dove folder, project and project name
    let project_name = "demoproject_16";
    let project_folder = project_start_new_and_build(project_name);
    let project_target = project_folder.join("target");
    assert!(
        project_target.exists(),
        "Target directory was not found: {}",
        project_target.display()
    );
    execute_dove_at(&["dove", "clean"], &project_folder).unwrap();
    assert!(
        !project_target.exists(),
        "Directory was not deleted: {}",
        project_target.display()
    );
    project_remove(&project_folder);
}
