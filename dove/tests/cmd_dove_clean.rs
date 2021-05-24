#![cfg(test)]

use dove::cli::execute;
mod test_cmd_helper;
use crate::test_cmd_helper::{project_start_nb, project_remove};

/// $ dove clean
/// project name: demoproject_16
#[test]
fn clean() {
    // Path to dove folder, project and project name
    let project_name = "demoproject_16";
    let project_folder = project_start_nb(project_name);

    // $ dove clean
    let args = &["dove", "clean"];
    let command_string: String = args.join(" ").to_string();
    execute(args, project_folder.clone()).unwrap_or_else(|err| {
        panic!(
            "[COMMAND] {}\r\n[FOLDER] {}\r\n[ERROR] {}\r\n",
            &command_string,
            project_folder.display(),
            err.to_string(),
        )
    });

    let project_target = project_folder.clone().join("target");
    assert!(
        !project_folder.clone().join("target").exists(),
        "Directory was not deleted: {}",
        project_target.to_str().unwrap_or(" - ")
    );

    project_remove(&project_folder);
}
