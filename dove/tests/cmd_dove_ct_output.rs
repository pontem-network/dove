#![cfg(test)]

use std::fs::{remove_file};
use fs_extra::file::write_all;
use dove::cli::execute;

mod test_cmd_helper;
use crate::test_cmd_helper::{project_start_nb, project_remove};

/// $ dove ct -o z
/// project name: demoproject_21
#[test]
fn output() {
    // Path to dove folder, project and project name
    let project_name = "demoproject_21";
    let project_folder = project_start_nb(project_name);
    // project_folder/scripts/sdemo.move
    write_all(
        &project_folder.clone().join("scripts/sdemo.move"),
        "script {
                    fun main() {
                        assert((1+3)==4,1);
                    }
                }",
    )
    .unwrap();
    // $ dove ct -o z
    let args = &["dove", "ct", "-o", "z"];
    let command_string: String = args.join(" ").to_string();
    execute(args, project_folder.clone()).unwrap_or_else(|err| {
        panic!(
            "[COMMAND] {}\r\n[FOLDER] {}\r\n[ERROR] {}\r\n",
            &command_string,
            project_folder.to_str().unwrap(),
            err.to_string()
        )
    });
    let tx_path = project_folder.clone().join("target/transactions/z.mvt");
    assert!(
        tx_path.exists(),
        "Transaction not found: {}\r\n[Command] {}",
        tx_path.to_str().unwrap(),
        &command_string,
    );
    remove_file(&tx_path).unwrap();

    project_remove(&project_folder);
}
