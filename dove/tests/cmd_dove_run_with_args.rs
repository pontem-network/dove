#![cfg(test)]

use fs_extra::file::write_all;
use dove::cli::execute;

mod test_cmd_helper;
use crate::test_cmd_helper::{project_start_nb, project_remove};

/// $ dove run rdemo.move -a 3 5
/// project name: demoproject_8
#[test]
fn with_args() {
    // Path to dove folder, project and project name
    let project_name = "demoproject_8";
    let project_folder = project_start_nb(project_name);

    // project_folder/scripts/demo.move
    write_all(
        &project_folder.clone().join("scripts/rdemo.move"),
        "script {
                fun main(x:u64,y:u64) {
                    let _result = x + y;
                }
            }",
    )
    .unwrap();
    // $ dove run rdemo.move -a 3 5
    let args = &["dove", "run", "rdemo.move", "-a", "3", "5"];
    let command_string: String = args.join(" ").to_string();
    execute(args, project_folder.clone()).unwrap_or_else(|err| {
        panic!(
            "[COMMAND] {}\r\n[FOLDER] {}\r\n[ERROR] {}\r\n",
            &command_string,
            &project_folder.display(),
            err.to_string()
        )
    });

    project_remove(&project_folder);
}
