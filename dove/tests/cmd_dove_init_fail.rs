#![cfg(test)]

use dove::cli::execute;
mod test_cmd_helper;
use crate::test_cmd_helper::{project_start_for_init, project_remove};

/// Fail
/// $ dove init -d incorrectdialect
/// $ dove build
/// project: demoproject_46
#[test]
fn incorrect_dialect() {
    // Project name and path
    let project_name = "demoproject_46";
    let project_folder = project_start_for_init(project_name);

    // $ dove new demoproject_46 -d incorrectdialect
    let args = &["dove", "init", "-d", "incorrectdialect"];
    let command_string: String = args.join(" ").to_string();
    assert!(
        execute(args, project_folder.clone()).is_err(),
        "[ERROR] There must be a mistake here. Invalid dialect\r\n[COMMAND] {}\r\n[DIALECT] incorrectdialect",
        &command_string,
    );
    project_remove(&project_folder);
}

/// Fail
/// Incorrect url: demo, /demo, /demo/api, //demo/api, //demo:8080/api, 127.0.0.1/api, ftp://demo.ru/api
/// $ dove init -r ###
/// $ dove build
/// project: demoproject_47
/// @todo Need to add validation on repo
#[test]
#[ignore]
fn incorrect_repo() {
    // Project name and path
    let project_name = "demoproject_47";
    let project_folder = project_start_for_init(project_name);

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
        let args = &["dove", "init", "-r", api];
        let command_string: String = args.join(" ").to_string();
        assert!(
            execute(args, project_folder.clone()).is_err(),
            "[ERROR] There must be a mistake here. Invalid repo\r\n[COMMAND] {}\r\n[DIALECT] {}",
            &command_string,
            api
        );
    }
    project_remove(&project_folder);
}
