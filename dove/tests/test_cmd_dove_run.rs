mod helper;

use helper::{new_demo_project, execute_dove_at, delete_project};

/// $ dove run 'main()'
/// $ dove run 'one_param(true)'
/// $ dove run 'two_params(1,1)'
#[test]
fn test_cmd_dove_run_with_call() {
    let project_name = "project_run_with_call";
    let project_folder = new_demo_project(project_name).unwrap();

    for call in ["main()", "one_param(true)", "two_params(1,1)"] {
        execute_dove_at(&["run", call], &project_folder).unwrap();
    }

    delete_project(&project_folder).unwrap();
}

/// $ dove run 'one_param' -a true
/// $ dove run 'two_params' --args 1 1
#[test]
fn test_cmd_dove_run_with_params() {
    let project_name = "project_run_with_params";
    let project_folder = new_demo_project(project_name).unwrap();

    for call in [
        vec!["run", "one_param", "-a", "true"],
        vec!["run", "two_params", "--args", "1", "1"],
    ] {
        execute_dove_at(&call, &project_folder).unwrap();
    }

    delete_project(&project_folder).unwrap();
}

/// With type
/// $ dove run 'with_type<u8>(1)'
/// $ dove run 'with_type(1)' -t u8
/// $ dove run 'with_type' -a 1 -t u8
#[test]
fn test_cmd_dove_run_with_type() {
    let project_name = "project_run_with_type";
    let project_folder = new_demo_project(project_name).unwrap();

    for call in [
        vec!["run", "with_type<u8>(1)"],
        vec!["run", "with_type(1)", "-t", "u8"],
        vec!["run", "with_type", "-a", "1", "-t", "u8"],
    ] {
        execute_dove_at(&call, &project_folder).unwrap();
    }

    delete_project(&project_folder).unwrap();
}

/// multiple scripts
/// $ dove run 'script_1(true)'
/// $ dove run 'script_2(1,1)'
#[test]
#[ignore]
fn test_cmd_dove_run_multiple() {
    let project_name = "project_run_multiple";
    let project_folder = new_demo_project(project_name).unwrap();

    for call in ["script_1(true)", "script_2(1,1)"] {
        execute_dove_at(&["run", call], &project_folder).unwrap();
    }

    delete_project(&project_folder).unwrap();
}
