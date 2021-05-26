mod helper;
use crate::helper::{
    project_remove, project_new_with_args, project_start, project_build, execute_dove_at,
    TErrPanicFormat,
};

/// $ dove metadata
#[test]
fn test_cmd_dove_metadata() {
    // Project name and path
    let project_name = "demoproject_15";
    let (base_folder, project_folder) = project_start(project_name);
    project_new_with_args(
        &base_folder,
        &project_folder,
        project_name,
        "pont",
        "5Csxuy81dNEVYbRA9K7tyHypu7PivHmwCZSKxcbU78Cy2v7v",
        "https://localhost/api",
    );
    project_build(&project_folder).err_panic_with_formatted();
    execute_dove_at(&["dove", "metadata"], &project_folder).err_panic_with_formatted();
    project_remove(&project_folder);
}
