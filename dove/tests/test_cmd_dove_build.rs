mod helper;

use std::path::PathBuf;
use std::fs;
use std::io::{Write, Read, BufRead, BufReader};
use std::collections::HashMap;
use std::fs::read_to_string;
use anyhow::Result;
use dialect::Dialect;
use crate::helper::{create_new_project, delete_project, execute_dove_at};

/// Build a project without additional parameters
/// $ dove build
#[test]
fn test_cmd_dove_build_without_arguments() {
    let project_name = "project_build_without_arguments";
    let project_path = create_project(&project_name).unwrap();

    execute_dove_at(&["build"], &project_path).unwrap();

    delete_project(&project_path).unwrap();
}

/// Build a project and generate documentation
/// $ dove build -d
#[test]
fn test_cmd_dove_build_with_doc() {
    let project_name = "project_build_build_with_doc";
    let project_path = create_project(&project_name).unwrap();

    execute_dove_at(&["build", "-d"], &project_path).unwrap();

    let docs_path = project_path.join("build").join(project_name).join("docs");

    assert!(["main.md", "demo1v.md", "demo2v.md", "demo3v.md"]
        .iter()
        .all(|name| docs_path.join(name).exists()));

    delete_project(&project_path).unwrap();
}

/// Build a project and package
/// $ dove build -p --modules_exclude NAME_1 NAME_2 ... NAME_N -o PACKAGE_NAME.mv
#[test]
fn test_cmd_dove_build_with_package() {
    let project_name = "project_build_build_with_package";
    let project_path = create_project(&project_name).unwrap();

    execute_dove_at(
        &[
            "build",
            "-p",
            "--modules_exclude",
            "demo1v",
            "demo2v.mv",
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
            .join("demo.mv"),
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
    let project_name = "project_build_build_error_map";
    let project_path = create_project(&project_name).unwrap();

    execute_dove_at(&["build", "--error-map", "error_map"], &project_path).unwrap();
    assert!(project_path.join("error_map.errmap").exists());

    delete_project(&project_path).unwrap();
}

fn create_project(project_name: &str) -> Result<PathBuf> {
    let t = [("Demo", "0x2")].into_iter().collect();
    let project_path = create_new_project(&project_name, t)?;
    // scripts/main.move
    let mut main_script = fs::File::create(project_path.join("scripts").join("main.move"))?;
    main_script.write(b"script { fun main(){} }");

    // sources/demo1v.move
    let mut demo1v_script = fs::File::create(project_path.join("sources").join("demo1v.move"))?;
    demo1v_script.write(b"module Demo::Demo1v{ fun run(){ } }");

    // sources/demo2v.move
    let mut demo2v_script = fs::File::create(project_path.join("sources").join("demo2v.move"))?;
    demo2v_script.write(b"module Demo::Demo2v{ fun run(){ } }");

    // sources/demo3v.move
    let mut demo3v_script = fs::File::create(project_path.join("sources").join("demo3v.move"))?;
    demo3v_script.write(b"module Demo::Demo3v{ fun run(){ } }");

    Ok(project_path)
}

fn find_u8(source: &[u8], need: &[u8]) -> bool {
    source
        .iter()
        .enumerate()
        .find(|(pos, _)| {
            need.iter()
                .enumerate()
                .all(|(index, byte)| Some(byte) == source.get(index + pos))
        })
        .is_some()
}
