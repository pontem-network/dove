mod helper;

use std::fs;
use std::io::Read;
use crate::helper::{delete_project, execute_dove_at, new_demo_project, build, create_new_project};
use std::collections::HashMap;

/// Build a project without additional parameters
/// $ dove build
#[test]
fn test_cmd_dove_build_without_arguments() {
    let project_name = "project_build_without_arguments";
    let project_path = new_demo_project(project_name, false).unwrap();

    execute_dove_at(&["build"], &project_path).unwrap();

    delete_project(&project_path).unwrap();
}

/// Build a project and generate documentation
/// $ dove build --doc
#[test]
fn test_cmd_dove_build_with_doc() {
    let project_name = "project_build_with_doc";
    let project_path = new_demo_project(project_name, false).unwrap();

    execute_dove_at(&["build", "--doc"], &project_path).unwrap();

    let docs_path = project_path.join("build").join(project_name).join("docs");

    assert!(["main.md", "demo1v.md", "demo2v.md", "demo3v.md"]
        .iter()
        .all(|name| docs_path.join(name).exists()));

    delete_project(&project_path).unwrap();
}

/// Build a project and package
/// $ dove build --bundle --modules_exclude NAME_1 NAME_2 ... NAME_N -o PACKAGE_NAME.mv
#[test]
fn test_cmd_dove_build_with_package() {
    let project_name = "project_build_with_package";
    let project_path = new_demo_project(project_name, false).unwrap();

    execute_dove_at(
        &[
            "build",
            "--bundle",
            "--modules_exclude",
            "Demo1v",
            "Demo2v",
            "-o",
            "demo",
        ],
        &project_path,
    )
    .unwrap();

    let mut content = Vec::new();
    fs::File::open(
        project_path
            .join("build")
            .join(project_name)
            .join("bundles")
            .join("demo.pac"),
    )
    .unwrap()
    .read_to_end(&mut content)
    .unwrap();

    assert!(find_u8(&content, b"Demo3v"));
    assert!(!find_u8(&content, b"Demo2v"));
    assert!(!find_u8(&content, b"Demo1v"));
    assert!(!find_u8(&content, b"main"));

    delete_project(&project_path).unwrap();
}

/// Build a project and package
/// $ dove build --error-map OUTPUT_NAME
#[test]
fn test_cmd_dove_build_error_map() {
    let project_name = "project_build_error_map";
    let project_path = new_demo_project(project_name, false).unwrap();

    execute_dove_at(&["build", "--error-map", "error_map"], &project_path).unwrap();
    assert!(project_path.join("error_map.errmap").exists());

    delete_project(&project_path).unwrap();
}

#[test]
fn test_cmd_dove_build_two_times() {
    let project_name = "project_build_two_times";
    let project_path = create_new_project(project_name, HashMap::new(), false).unwrap();

    build(&project_path).unwrap();
    build(&project_path).unwrap();

    delete_project(&project_path).unwrap();
}

fn find_u8(source: &[u8], need: &[u8]) -> bool {
    source.iter().enumerate().any(|(pos, _)| {
        need.iter()
            .enumerate()
            .all(|(index, byte)| Some(byte) == source.get(index + pos))
    })
}
