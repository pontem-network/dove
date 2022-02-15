mod helpers;

use std::fs;
use std::io::Read;
use helpers::{delete_project, dove, new_demo_project};

/// Build a project and package
/// $ dove deploy --modules_exclude Demo1v Demo2v
#[test]
fn test_cmd_dove_deploy() {
    let project_name = "project_deploy";
    let project_path = new_demo_project(project_name).unwrap();

    dove(
        &["deploy", "--modules_exclude", "Demo1v", "Demo2v"],
        &project_path,
    )
    .unwrap();

    let mut content = Vec::new();
    fs::File::open(
        project_path
            .join("build")
            .join("for_tests")
            .join("bundles")
            .join("for_tests.pac"),
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

fn find_u8(source: &[u8], need: &[u8]) -> bool {
    source.iter().enumerate().any(|(pos, _)| {
        need.iter()
            .enumerate()
            .all(|(index, byte)| Some(byte) == source.get(index + pos))
    })
}
