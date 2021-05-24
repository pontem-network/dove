#![cfg(test)]

use std::fs::{remove_file};
use fs_extra::file::write_all;
use dove::cmd::ct::Transaction;
use dove::cli::execute;

mod test_cmd_helper;
use crate::test_cmd_helper::{project_start_nb, project_remove};

/// $ dove ct -a 1 2
/// $ dove ct 'main(1,2)'
/// project name: demoproject_22
#[test]
fn args() {
    // Path to dove folder, project and project name
    let project_name = "demoproject_22";
    let project_folder = project_start_nb(project_name);

    // project_folder/scripts/sdemo.move
    write_all(
        &project_folder.clone().join("scripts/sdemo.move"),
        "script {
                    fun main(_a1:u64,_a2:u64) { }
                }",
    )
    .unwrap();
    // $ dove ct -a 1 2
    {
        let args = &["dove", "ct", "-a", "1", "2"];
        let command_string: String = args.join(" ").to_string();
        execute(args, project_folder.clone()).unwrap_or_else(|err| {
            panic!(
                "[COMMAND] {}\r\n[FOLDER] {}\r\n[ERROR] {}\r\n",
                &command_string,
                project_folder.display(),
                err.to_string(),
            )
        });

        let tx_path = project_folder.clone().join("target/transactions/main.mvt");
        assert!(
            tx_path.exists(),
            "Transaction not found: {}\r\n[Command] {}",
            tx_path.to_str().unwrap(),
            &command_string,
        );
        let tx_fmt = format!(
            "{:?}",
            bcs::from_bytes::<Transaction>(std::fs::read(&tx_path).unwrap().as_ref()).unwrap()
        );

        assert!(tx_fmt.contains(" args: [U64(1), U64(2)]"));
        assert!(tx_fmt.contains(" type_args: []"));
        assert!(tx_fmt.contains(" signers_count: 0"));

        remove_file(&tx_path).unwrap();
    }
    // $ dove ct 'main(1,2)'
    {
        let args = &["dove", "ct", "main(1,2)"];
        let command_string: String = args.join(" ").to_string();
        execute(args, project_folder.clone()).unwrap_or_else(|err| {
            panic!(
                "[COMMAND] {}\r\n[FOLDER] {}\r\n[ERROR] {}\r\n",
                &command_string,
                project_folder.display(),
                err.to_string(),
            )
        });

        let tx_path = project_folder.clone().join("target/transactions/main.mvt");
        assert!(
            tx_path.exists(),
            "Transaction not found: {}\r\n[Command] {}",
            tx_path.to_str().unwrap(),
            &command_string,
        );
        let tx_fmt = format!(
            "{:?}",
            bcs::from_bytes::<Transaction>(std::fs::read(&tx_path).unwrap().as_ref()).unwrap()
        );

        assert!(tx_fmt.contains(" args: [U64(1), U64(2)]"));
        assert!(tx_fmt.contains(" type_args: []"));
        assert!(tx_fmt.contains(" signers_count: 0"));

        remove_file(&tx_path).unwrap();
    }
    project_remove(&project_folder);
}
