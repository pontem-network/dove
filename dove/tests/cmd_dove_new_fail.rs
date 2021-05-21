#![cfg(test)]

use std::path::{Path};
use std::fs::{remove_dir_all};
use dove::cli::execute;

/// Fail
/// $ dove new demoproject_37 -d incorrectdialect
/// $ dove build
/// project: demoproject_37
#[test]
fn incorrect_dialect() {
    // Path to dove folder
    let dove_folder = {
        let mut folder = Path::new(".").canonicalize().unwrap();
        if folder.to_str().unwrap().find("dove").is_none() {
            folder.push("dove");
        }
        folder
    };
    // Project name and path
    let project_name = "demoproject_37";
    let mut folder = dove_folder.clone();
    folder.push(project_name);
    if folder.exists() {
        remove_dir_all(&folder).expect(&format!(
            "[ERROR] Couldn't delete project directory: {}",
            folder.to_str().unwrap()
        ));
    }

    // $ dove new demoproject_37 -d incorrectdialect
    {
        let args = &["dove", "new", &project_name, "-d", "incorrectdialect"];
        let command_string: String = args.join(" ").to_string();
        assert!(execute(args, dove_folder.clone()).is_err(), "[ERROR] There must be a mistake here. Invalid dialect\r\n[COMMAND] {}\r\n[DIALECT] incorrectdialect", &command_string)
    }
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
    // Path to dove folder
    let dove_folder = {
        let mut folder = Path::new(".").canonicalize().unwrap();
        if folder.to_str().unwrap().find("dove").is_none() {
            folder.push("dove");
        }
        folder
    };
    // Project name and path
    let project_name = "demoproject_38";
    {
        let mut folder = dove_folder.clone();
        folder.push(project_name);
        if folder.exists() {
            remove_dir_all(&folder).expect(&format!(
                "[ERROR] Couldn't delete project directory: {}",
                folder.to_str().unwrap()
            ));
        }
    };
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
        {
            let args = &["dove", "new", &project_name, "-r", api];
            let command_string: String = args.join(" ").to_string();
            assert!(execute(args, dove_folder.clone()).is_err(), "[ERROR] There must be a mistake here. Invalid repo\r\n[COMMAND] {}\r\n[DIALECT] {}", &command_string, api)
        }
    }
}
