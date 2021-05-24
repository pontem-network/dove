#![cfg(test)]

use dove::cli::execute;

mod test_cmd_helper;
use crate::test_cmd_helper::{project_remove, project_start};

/// Fail
/// $ dove new demoproject_37 -d incorrectdialect
/// $ dove build
/// project: demoproject_37
#[test]
fn incorrect_dialect() {
    // Project name and path
    let project_name = "demoproject_37";
    let (base_folder, project_folder) = project_start(project_name);
    project_remove(&project_folder);

    // $ dove new demoproject_37 -d incorrectdialect
    let args = &["dove", "new", &project_name, "-d", "incorrectdialect"];
    let command_string: String = args.join(" ").to_string();
    assert!(execute(args, base_folder.clone()).is_err(),
            "[ERROR] There must be a mistake here. Invalid dialect\r\n[COMMAND] {}\r\n[DIALECT] incorrectdialect\r\n[FOLDER] {}\r\n", &command_string, &base_folder.display())
}

/// Fail
/// Incorrect url: demo, /demo, /demo/api, //demo/api, //demo:8080/api, 127.0.0.1/api, ftp://demo.ru/api
/// $ dove new demoproject_38 -r ###
/// $ dove build
/// project: demoproject_38
/// @todo Need to add validation on repo
#[test]
#[ignore]
fn incorrect_repo() {
    // Project name and path
    let project_name = "demoproject_38";
    let (base_folder, project_folder) = project_start(project_name);
    project_remove(&project_folder);

    for api in &[
        "demo",
        "/demo",
        "/demo/api",
        "//demo/api",
        "//demo:8080/api",
        "127.0.0.1/api",
        "ftp://demo.ru/api",
    ] {
        // $ dove new demoproject_35 -r ###
        let args = &["dove", "new", &project_name, "-r", api];
        let command_string: String = args.join(" ").to_string();
        assert!(
            execute(args, base_folder.clone()).is_err(),
            "[ERROR] There must be a mistake here. Invalid repo\r\n[COMMAND] {}\r\n[DIALECT] {}\r\n[FOLDER] {}\r\n",
            &command_string,
            api,
            base_folder.display()
        );
    }
}
