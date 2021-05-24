#![cfg(test)]

mod test_cmd_helper;
use crate::test_cmd_helper::{project_remove, project_new_with_args, project_start, project_build};
use dove::cli::execute;

/// $ dove metadata
/// project name: demoproject_15
#[test]
fn default() {
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
    project_build(&project_folder);

    // $ dove metadata
    let args = &["dove", "metadata"];
    let command_string: String = args.join(" ").to_string();
    execute(args, project_folder.to_path_buf()).unwrap_or_else(|err| {
        panic!(
            "[COMMAND] {}\r\n[FOLDER] {}\r\n[ERROR] {}\r\n",
            &command_string,
            project_folder.to_str().unwrap(),
            err.to_string()
        )
    });

    project_remove(&project_folder);
}
