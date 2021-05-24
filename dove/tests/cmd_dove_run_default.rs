#![cfg(test)]

use fs_extra::file::write_all;
use dove::cli::execute;

mod test_cmd_helper;
use crate::test_cmd_helper::{project_start_nb, project_remove};

/// $ dove run rdemo.move
/// project name: demoproject_6
#[test]
fn default() {
    // Path to dove folder, project and project name
    let project_name = "demoproject_6";
    let project_folder = project_start_nb(project_name);

    // project_folder/modules/mdemo.move
    write_all(
        &project_folder.clone().join("modules/mdemo.move"),
        "address 0x1 {
                module DemoModule {
                    public fun value(): u8 {
                        12
                    }
                }
            }",
    )
    .unwrap();
    // project_folder/scripts/demo.move
    write_all(
        &project_folder.clone().join("scripts/rdemo.move"),
        "script {
                use 0x1::DemoModule;
                fun main() {
                    let _value = DemoModule::value();
                }
            }",
    )
    .unwrap();
    // $ dove run rdemo.move
    let args = &["dove", "run", "rdemo.move"];
    let command_string: String = args.join(" ").to_string();
    execute(args, project_folder.clone()).unwrap_or_else(|err| {
        panic!(
            "[COMMAND] {}\r\n[FOLDER] {}\r\n[ERROR] {}\r\n",
            &command_string,
            project_folder.display(),
            err.to_string()
        )
    });

    project_remove(&project_folder);
}
