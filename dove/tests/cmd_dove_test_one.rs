#![cfg(test)]

use fs_extra::file::write_all;
use dove::cli::execute;

mod test_cmd_helper;
use crate::test_cmd_helper::{project_start_nb, project_remove};

/// $ dove test -k test_2
/// project name: demoproject_11
#[test]
fn one_test() {
    // Path to dove folder, project and project name
    let project_name = "demoproject_11";
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
    // project_folder/tests/test_1.move
    write_all(
        &project_folder.clone().join("tests/test_1.move"),
        "script {
                fun main() {
                    assert((1+3)==4,1);
                }
            }",
    )
    .unwrap();
    // project_folder/tests/test_2.move
    write_all(
        &project_folder.clone().join("tests/test_2.move"),
        "script {
                fun main() {
                    assert((2+2)==4,2);
                }
            }",
    )
    .unwrap();

    // $ dove test -k test_2
    let args = &["dove", "test", "-k", "test_2"];
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
