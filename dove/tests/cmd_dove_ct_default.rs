#![cfg(test)]

use std::fs::{remove_file};
use fs_extra::file::write_all;
use dove::cmd::ct::Transaction;
use dove::cli::execute;

mod test_cmd_helper;
use crate::test_cmd_helper::{project_start_nb, project_remove};

/// $ dove ct
/// project name: demoproject_19
#[test]
fn default() {
    // Path to dove folder, project and project name
    let project_name = "demoproject_19";
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

    // $ dove ct
    {
        let args = &["dove", "ct"];
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
        assert!(tx_fmt.contains(" args: []"));
        assert!(tx_fmt.contains(" type_args: []"));
        assert!(tx_fmt.contains(" signers_count: 0"));

        remove_file(&tx_path).unwrap();
    }

    project_remove(&project_folder);
}
