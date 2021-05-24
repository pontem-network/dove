#![cfg(test)]

use fs_extra::file::write_all;
mod test_cmd_helper;
use crate::test_cmd_helper::{
    project_start_nb, project_remove, execute_dove_at, execute_dove_at_wait_fail,
};

/// $ dove test
#[test]
fn test_cmd_dove_test_run_all_test_in_project() {
    // Path to dove folder, project and project name
    let project_name = "demoproject_10";
    let project_folder = project_start_nb(project_name);

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
    // project_folder/tests/test_1.move
    write_all(
        &project_folder.join("tests/test_1.move"),
        "script {
                fun main() {
                    assert((3+1)==4,1);
                }
            }",
    )
    .unwrap();
    execute_dove_at(&project_folder, &["dove", "test"]);
    project_remove(&project_folder);
}

/// $ dove test -k test_2
#[test]
fn test_cmd_dove_test_run_one_test_in_project() {
    // Path to dove folder, project and project name
    let project_name = "demoproject_11";
    let project_folder = project_start_nb(project_name);

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
    // project_folder/tests/test_1.move
    write_all(
        &project_folder.join("tests/test_1.move"),
        "script {
                fun main() {
                    assert((1+3)==4,1);
                }
            }",
    )
    .unwrap();
    // project_folder/tests/test_2.move
    write_all(
        &project_folder.join("tests/test_2.move"),
        "script {
                fun main() {
                    assert((2+2)==4,2);
                }
            }",
    )
    .unwrap();
    execute_dove_at(&project_folder, &["dove", "test", "-k", "test_2"]);
    project_remove(&project_folder);
}

/// $ dove test
#[test]
fn test_cmd_dove_test_fail_test_in_project() {
    // Path to dove folder, project and project name
    let project_name = "demoproject_12";
    let project_folder = project_start_nb(project_name);

    // project_folder/tests/test_1.move
    write_all(
        &project_folder.join("tests/test_1.move"),
        "script {
                fun main() {
                    assert((3+2)==4,1);
                }
            }",
    )
    .unwrap();
    execute_dove_at_wait_fail(&project_folder, &["dove", "test"]);
    project_remove(&project_folder);
}
