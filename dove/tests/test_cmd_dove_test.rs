use fs_extra::file::write_all;
use dove::tests_helper::{execute_dove_at, project_start_new_and_build, project_remove};

/// $ dove test
#[test]
fn test_cmd_dove_test_run_all_test_in_project() {
    // Path to dove folder, project and project name
    let project_name = "project_test_run_all_test_in_project";
    let project_folder = project_start_new_and_build(project_name, None);
    // project_folder/modules/mdemo.move
    write_all(
        &project_folder.join("modules").join("mdemo.move"),
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
        &project_folder.join("tests").join("test_1.move"),
        "module 0x1::Tests {
                    #[test]
                    fun main() {
                        assert((3+1)==4,1);
                    }
            }",
    )
    .unwrap();
    execute_dove_at(&["dove", "test"], &project_folder).unwrap();
    project_remove(&project_folder);
}

/// $ dove test -f test_2
#[test]
fn test_cmd_dove_test_run_one_test_in_project() {
    // Path to dove folder, project and project name
    let project_name = "project_test_run_one_test_in_project";
    let project_folder = project_start_new_and_build(project_name, None);
    // project_folder/modules/mdemo.move
    write_all(
        &project_folder.join("modules").join("mdemo.move"),
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
        &project_folder.join("tests").join("test_1.move"),
        "module 0x1::Tests {
                 #[test]
                fun test_1() {
                    assert((1+3)==4,1);
                }
            }",
    )
    .unwrap();
    // project_folder/tests/test_2.move
    write_all(
        &project_folder.join("tests").join("test_2.move"),
        "module 0x1::Tests_2 {
                #[test]
                fun test_2() {
                    assert((2+2)==4,2);
                }
            }",
    )
    .unwrap();
    execute_dove_at(&["dove", "test", "-f", "test_2"], &project_folder).unwrap();
    project_remove(&project_folder);
}

/// $ dove test
#[test]
fn test_cmd_dove_test_fail_test_in_project() {
    // Path to dove folder, project and project name
    let project_name = "project_test_fail_test_in_project";
    let project_folder = project_start_new_and_build(project_name, None);
    // project_folder/tests/test_1.move
    write_all(
        &project_folder.join("tests").join("test_1.move"),
        "module 0x1::Tests {
                #[test]
                fun main() {
                    assert((3+2)==4,1);
                }
            }",
    )
    .unwrap();
    assert!(execute_dove_at(&["dove", "test"], &project_folder).is_err());
    project_remove(&project_folder);
}
