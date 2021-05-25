use fs_extra::file::write_all;
mod helper;
use crate::helper::{execute_dove_at, project_start_new_and_build, project_remove};

/// $ dove run rdemo.move
#[test]
fn test_cmd_dove_run_without_arguments() {
    // Path to dove folder, project and project name
    let project_name = "demoproject_6";
    let project_folder = project_start_new_and_build(project_name);

    // project_folder/modules/mdemo.move
    write_all(
        &project_folder.join("modules/mdemo.move"),
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
        &project_folder.join("scripts/rdemo.move"),
        "script {
                use 0x1::DemoModule;
                fun main() {
                    let _value = DemoModule::value();
                }
            }",
    )
    .unwrap();
    execute_dove_at(&project_folder, &["dove", "run", "rdemo.move"]).unwrap_or_else(|err| {
        panic!("{}", err);
    });

    project_remove(&project_folder);
}

/// $ dove run rdemo.move -a 3 5
#[test]
fn test_cmd_dove_run_with_arguments() {
    // Path to dove folder, project and project name
    let project_name = "demoproject_8";
    let project_folder = project_start_new_and_build(project_name);

    // project_folder/scripts/demo.move
    write_all(
        &project_folder.join("scripts/rdemo.move"),
        "script {
                fun main(x:u64,y:u64) {
                    let _result = x + y;
                }
            }",
    )
    .unwrap();
    execute_dove_at(
        &project_folder,
        &["dove", "run", "rdemo.move", "-a", "3", "5"],
    )
    .unwrap_or_else(|err| {
        panic!("{}", err);
    });

    project_remove(&project_folder);
}
