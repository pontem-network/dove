use fs_extra::file::write_all;
mod helper;
use crate::helper::{execute_dove_at, project_start_new_and_build, project_remove};
/// $ dove run sdemo.move
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
    // project_folder/scripts/sdemo.move
    write_all(
        &project_folder.join("scripts/sdemo.move"),
        "script {
                use 0x1::DemoModule;
                fun main() {
                    let _value = DemoModule::value();
                }
            }",
    )
    .unwrap();
    execute_dove_at(&["dove", "run", "sdemo.move"], &project_folder).unwrap();
    project_remove(&project_folder);
}
/// $ dove run sdemo.move -a 3 5
#[test]
fn test_cmd_dove_run_with_arguments() {
    // Path to dove folder, project and project name
    let project_name = "demoproject_8";
    let project_folder = project_start_new_and_build(project_name);
    // project_folder/scripts/sdemo.move
    write_all(
        &project_folder.join("scripts/sdemo.move"),
        "script {
                fun main(x:u64,y:u64) {
                    let _result = x + y;
                }
            }",
    )
    .unwrap();
    execute_dove_at(
        &["dove", "run", "sdemo.move", "-a", "3", "5"],
        &project_folder,
    )
    .unwrap();
    project_remove(&project_folder);
}
/// $ dove run sdemo.move --signers 0x1
#[test]
fn test_cmd_dove_run_with_signers() {
    // Path to dove folder, project and project name
    let project_name = "demoproject_5";
    let project_folder = project_start_new_and_build(project_name);
    // project_folder/scripts/sdemo.move
    write_all(
        &project_folder.join("scripts/sdemo.move"),
        "script {
            fun main(_account: signer) { }
        }",
    )
    .unwrap();
    execute_dove_at(
        &["dove", "run", "sdemo.move", "--signers", "0x1"],
        &project_folder,
    )
    .unwrap();
    project_remove(&project_folder);
}
