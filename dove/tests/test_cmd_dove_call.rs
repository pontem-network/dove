mod helper;

use helper::{new_demo_project, execute_dove_at, delete_project};

/// $ dove call 'main()'
/// $ dove call 'one_param(true)'
/// $ dove call 'two_params(1,1)'
#[test]
fn test_cmd_dove_call() {
    let project_name = "project_call";
    let project_folder = new_demo_project(project_name, false).unwrap();

    for call in ["main()", "one_param(true)", "two_params(1,1)"] {
        execute_dove_at(&["call", call], &project_folder).unwrap();
    }
    delete_project(&project_folder).unwrap();
}

/// $ dove call 'main()'
/// $ dove call 'one_param' -a true
/// $ dove call 'two_params' --args 1 1
#[test]
fn test_cmd_dove_call_with_params() {
    let project_name = "project_call_with_params";
    let project_folder = new_demo_project(project_name, false).unwrap();

    for call in [
        vec!["call", "one_param", "-a", "true"],
        vec!["call", "two_params", "--args", "1", "1"],
    ] {
        execute_dove_at(&call, &project_folder).unwrap();
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
    let project_folder = new_demo_project(project_name, false).unwrap();

    for call in [
        vec!["call", "with_type<u8>(1)"],
        vec!["call", "with_type(1)", "-t", "u8"],
        vec!["call", "with_type", "-a", "1", "-t", "u8"],
    ] {
        execute_dove_at(&call, &project_folder).unwrap();
    }

    delete_project(&project_folder).unwrap();
}

/// Output path
/// $ dove call 'main()' -o tmpname
#[test]
fn test_cmd_dove_call_output() {
    let project_name = "project_call_output";
    let project_folder = new_demo_project(project_name, false).unwrap();

    for (name, args) in [
        ("main", vec!["call", "main()"]),
        ("tmpname", vec!["call", "main()", "-o", "tmpname"]),
    ] {
        execute_dove_at(&args, &project_folder).unwrap();
        let tx_path = project_folder
            .join("build")
            .join(project_name)
            .join("transaction")
            .join(format!("{}.mvt", name));
        assert!(tx_path.exists());
    }

    delete_project(&project_folder).unwrap();
}
