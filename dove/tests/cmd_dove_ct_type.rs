#![cfg(test)]

use std::fs::{remove_file};
use fs_extra::file::write_all;
use dove::cmd::ct::Transaction;
use dove::cli::execute;

mod test_cmd_helper;
use crate::test_cmd_helper::{project_start_nb, project_remove};

/// $ dove ct 'sdemo_4<u8>(16)'
/// project name: demoproject_24
#[test]
fn a_type() {
    // Path to dove folder, project and project name
    let project_name = "demoproject_24";
    let project_folder = project_start_nb(project_name);

    // project_folder/modules/mdemo.move
    write_all(
        &project_folder.clone().join("modules/mdemo.move"),
        "module ModuleDemo{
            struct T1 {}
            struct T2 {}

            struct Demo<T> has drop{
                value:u8
            }
            public fun new<T:drop>(value:u8): Demo<T>{
                Demo<T>{
                    value
                }
            }
        }",
    )
    .unwrap();
    // project_folder/scripts/sdemo.move
    write_all(
        &project_folder.clone().join("scripts/sdemo.move"),
        "script {
            use 0x1::ModuleDemo;

            fun sdemo_4<T:drop>(value:u8) {
                let _tmp:ModuleDemo::Demo<T> = ModuleDemo::new<T>(value);
            }
        }",
    )
    .unwrap();
    // $ dove ct 'main<u8>(16)'
    let args = &["dove", "ct", "sdemo_4<u8>(16)"];
    let command_string: String = args.join(" ").to_string();
    execute(args, project_folder.clone()).unwrap_or_else(|err| {
        panic!(
            "[COMMAND] {}\r\n[FOLDER] {}\r\n[ERROR] {}\r\n",
            &command_string,
            project_folder.to_str().unwrap(),
            err.to_string()
        )
    });

    let tx_path = project_folder
        .clone()
        .join("target/transactions/sdemo_4.mvt");
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
    assert!(tx_fmt.contains(" args: [U8(16)]"));
    assert!(tx_fmt.contains(" type_args: [U8]"));
    assert!(tx_fmt.contains(" signers_count: 0"));

    remove_file(&tx_path).unwrap();

    // @todo Add tests for $ dove ct -t ###, after bug fix

    project_remove(&project_folder);
}
