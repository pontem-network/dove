use dove::tests_helper::{execute_dove_at, project_start_new_and_build, project_remove};

/// $ dove clean
#[test]
fn test_cmd_dove_clean() {
    // Path to dove folder, project and project name
    let project_name = "project_clean";
    let project_folder = project_start_new_and_build(project_name);
    let project_artifacts = project_folder.join("artifacts");
    assert!(
        project_artifacts.exists(),
        "Artifacts directory was not found: {}",
        project_artifacts.display()
    );
    execute_dove_at(&["dove", "clean"], &project_folder).unwrap();
    assert!(
        !project_artifacts.exists(),
        "Directory was not deleted: {}",
        project_artifacts.display()
    );
    project_remove(&project_folder);
}
