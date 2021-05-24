#![cfg(test)]

use std::fs::{remove_file};
use fs_extra::file::write_all;
use dove::cli::execute;

mod test_cmd_helper;
use crate::test_cmd_helper::{project_start_nb, project_remove};

/// $ dove ct -f sdemo_2
/// project name: demoproject_20
#[test]
fn file() {
    // Path to dove folder, project and project name
    let project_name = "demoproject_20";
    let project_folder = project_start_nb(project_name);
    // project_folder/scripts/sdemo_1.move
    write_all(
        &project_folder.clone().join("scripts/sdemo_1.move"),
        "script {
                    fun sdemo_1() {
                        assert((1+3)==4,1);
                    }
                }",
    )
    .unwrap();
    // project_folder/scripts/sdemo_2.move
    write_all(
        &project_folder.clone().join("scripts/sdemo_2.move"),
        "script {
                    fun sdemo_2() {
                        assert((2+2)==4,1);
                    }
                }",
    )
    .unwrap();
    // $ dove ct -f sdemo_2
    {
        let args = &["dove", "ct", "-f", "sdemo_2"];
        let command_string: String = args.join(" ").to_string();
        execute(args, project_folder.clone()).unwrap_or_else(|err| {
            panic!(
                "[COMMAND] {}\r\n[FOLDER] {}\r\n[ERROR] {}\r\n",
                &command_string,
                project_folder.display(),
                err.to_string(),
            )
        });

        let tx_path = project_folder
            .clone()
            .join("target/transactions/sdemo_2.mvt");
        assert!(
            tx_path.exists(),
            "Transaction not found: {}\r\n[Command] {}",
            tx_path.to_str().unwrap(),
            &command_string,
        );
        remove_file(&tx_path).unwrap();
    }

    project_remove(&project_folder);
}
