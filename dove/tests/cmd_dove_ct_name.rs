#![cfg(test)]

use std::fs::{remove_file};
use fs_extra::file::write_all;
use dove::cli::execute;

mod test_cmd_helper;
use crate::test_cmd_helper::{project_start_nb, project_remove};

/// $ dove ct -n test_fun -f sdemo
/// $ dove ct 'test_fun()' -f sdemo
/// project name: demoproject_23
#[test]
fn name() {
    // Path to dove folder, project and project name
    let project_name = "demoproject_23";
    let project_folder = project_start_nb(project_name);

    // project_folder/scripts/sdemo.move
    write_all(
        &project_folder.clone().join("scripts/sdemo.move"),
        "script {
                    fun main(_a:u64,_b:u64) { }
                }
                script {
                    fun test_fun() { }
                }",
    )
    .unwrap();
    // $ dove ct -n test_fun -f sdemo
    {
        let args = &["dove", "ct", "-f", "sdemo", "-n", "test_fun"];
        let command_string: String = args.join(" ").to_string();
        execute(args, project_folder.clone()).unwrap_or_else(|err| {
            panic!(
                "[COMMAND] {}\r\n[FOLDER] {}\r\n[ERROR] {}\r\n",
                &command_string,
                project_folder.display(),
                err.to_string()
            )
        });

        let tx_path = project_folder
            .clone()
            .join("target/transactions/test_fun.mvt");
        assert!(
            tx_path.exists(),
            "Transaction not found: {}\r\n[Command] {}",
            tx_path.to_str().unwrap(),
            &command_string,
        );

        remove_file(&tx_path).unwrap();
    }

    // $ dove ct 'test_fun()' -f sdemo
    {
        let args = &["dove", "ct", "test_fun()", "-f", "sdemo"];
        let command_string: String = args.join(" ").to_string();
        execute(args, project_folder.clone()).unwrap_or_else(|err| {
            panic!(
                "[COMMAND] {}\r\n[FOLDER] {}\r\n[ERROR] {}\r\n",
                &command_string,
                project_folder.display(),
                err.to_string()
            )
        });

        let tx_path = project_folder
            .clone()
            .join("target/transactions/test_fun.mvt");
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
