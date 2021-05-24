#![cfg(test)]

mod test_cmd_helper;
use crate::test_cmd_helper::{project_start_nb, project_remove, execute_dove_at};

/// $ dove clean
#[test]
fn test_cmd_dove_clean() {
    // Path to dove folder, project and project name
    let project_name = "demoproject_16";
    let project_folder = project_start_nb(project_name);

    // $ dove clean
    execute_dove_at(&project_folder, &["dove", "clean"]);
    let project_target = project_folder.join("target");
    assert!(
        !project_target.exists(),
        "Directory was not deleted: {}",
        project_target.display()
    );

    project_remove(&project_folder);
}
