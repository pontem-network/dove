mod helpers;

use helpers::{new_demo_project, dove, delete_project};

/// $ dove call 'main()'
/// $ dove call 'one_param(true)'
/// $ dove call 'two_params(1,1)'
#[test]
fn test_cmd_dove_call() {
    let project_name = "project_call";
    let project_folder = new_demo_project(project_name).unwrap();

    for (name, call) in [
        ("main", "main()"),
        ("one_param", "one_param(true)"),
        ("two_params", "two_params(1,1)"),
    ] {
        dove(&["call", call], &project_folder).unwrap();
        let tx_path = project_folder
            .join("build")
            .join("for_tests")
            .join("transaction")
            .join(format!("{}.mvt", name));
        assert!(tx_path.exists());
    }
    delete_project(&project_folder).unwrap();
}

/// $ dove call 'one_param' -a true
/// $ dove call 'two_params' --args 1 1
#[test]
fn test_cmd_dove_call_with_params() {
    let project_name = "project_call_with_params";
    let project_folder = new_demo_project(project_name).unwrap();

    for call in [
        vec!["call", "one_param", "-a", "true"],
        vec!["call", "two_params", "--args", "1", "1"],
    ] {
        dove(&call, &project_folder).unwrap();
    }

    delete_project(&project_folder).unwrap();
}

/// With type
/// $ dove call 'with_type<u8>(1)'
/// $ dove call 'with_type(1)' -t u8
/// $ dove call 'with_type' -a 1 -t u8
#[test]
fn test_cmd_dove_call_with_type() {
    let project_name = "project_call_with_type";
    let project_folder = new_demo_project(project_name).unwrap();

    for call in [
        vec!["call", "with_type<u8>(1)"],
        vec!["call", "with_type(1)", "-t", "u8"],
        vec!["call", "with_type", "-a", "1", "-t", "u8"],
    ] {
        dove(&call, &project_folder).unwrap();
    }

    delete_project(&project_folder).unwrap();
}
