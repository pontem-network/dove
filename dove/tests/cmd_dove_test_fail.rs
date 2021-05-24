#![cfg(test)]

use fs_extra::file::write_all;
use dove::cli::execute;

mod test_cmd_helper;
use crate::test_cmd_helper::{project_start_nb, project_remove};

/// $ dove test
/// project name: demoproject_12
#[test]
fn fail() {
    // Path to dove folder, project and project name
    let project_name = "demoproject_12";
    let project_folder = project_start_nb(project_name);

    // project_folder/tests/test_1.move
    write_all(
        &project_folder.clone().join("tests/test_1.move"),
        "script {
                fun main() {
                    assert((3+2)==4,1);
                }
            }",
    )
    .unwrap();
    // $ dove test
    let args = &["dove", "test"];
    let command_string: String = args.join(" ").to_string();
    assert!(
        execute(args, project_folder.clone()).is_err(),
        "Expected error\r\n[COMMAND] {}\r\n[FOLDER] {}\r\n",
        &command_string,
        project_folder.to_str().unwrap()
    );

    project_remove(&project_folder);
}
