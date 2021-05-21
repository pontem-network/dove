#![cfg(test)]

mod test_cmd_helper;
use crate::test_cmd_helper::{project_start_nb, project_remove};
use dove::cli::execute;

/// $ cargo run -- clean
/// $ dove clean
/// project name: demoproject_16
#[test]
fn clean() {
    // Path to dove folder, Project name and path
    let project_name = "demoproject_16";
    let (_, project_folder) = project_start_nb(project_name);

    // $ dove clean
    let args = &["dove", "clean"];
    let command_string: String = args.join(" ").to_string();
    execute(args, project_folder.clone()).unwrap_or_else(|_| {
        panic!(
            "[COMMAND] {}\r\n[FOLDER] {}",
            &command_string,
            project_folder.to_str().unwrap()
        )
    });

    let mut project_target = project_folder.clone();
    project_target.push("target");

    assert!(
        !project_target.exists(),
        "Directory was not deleted: {}",
        project_target.to_str().unwrap_or(" - ")
    );

    project_remove(&project_folder);
}
