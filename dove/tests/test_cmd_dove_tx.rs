mod helper;

use helper::{new_demo_project, execute_dove_at, delete_project};

/// $ dove tx 'main()'
/// $ dove tx 'one_param(true)'
/// $ dove tx 'two_params(1,1)'
#[test]
fn test_cmd_dove_tx_with_call() {
    let project_name = "project_tx_with_call";
    let project_folder = new_demo_project(&project_name).unwrap();

    for call in ["main()", "one_param(true)", "two_params(1,1)"] {
        execute_dove_at(&["tx", call], &project_folder).unwrap();
    }
    delete_project(&project_folder).unwrap();
}

/// $ dove tx 'main()'
/// $ dove tx 'one_param' -p true
/// $ dove tx 'two_params' -p 1 1
#[test]
fn test_cmd_dove_tx_with_params() {
    let project_name = "project_tx_with_params";
    let project_folder = new_demo_project(&project_name).unwrap();

    for call in [
        vec!["tx", "one_param", "-p", "true"],
        vec!["tx", "two_params", "-p", "1", "1"],
    ] {
        execute_dove_at(&call, &project_folder).unwrap();
    }

    delete_project(&project_folder).unwrap();
}

/// With type
/// $ dove tx 'with_type<u8>(1)'
/// $ dove tx 'with_type(1)' -t u8
/// $ dove tx 'with_type' -p 1 -t u8
#[test]
fn test_cmd_dove_tx_with_type() {
    let project_name = "project_tx_with_type";
    let project_folder = new_demo_project(&project_name).unwrap();

    for call in [
        vec!["tx", "with_type<u8>(1)"],
        vec!["tx", "with_type(1)", "-t", "u8"],
        vec!["tx", "with_type", "-p", "1", "-t", "u8"],
    ] {
        execute_dove_at(&call, &project_folder).unwrap();
    }

    delete_project(&project_folder).unwrap();
}

/// Output path
/// $ dove tx 'main()' -o tmpname
#[test]
fn test_cmd_dove_tx_output() {
    let project_name = "project_tx_output";
    let project_folder = new_demo_project(&project_name).unwrap();

    for (name, args) in [
        ("main", vec!["tx", "main()"]),
        ("tmpname", vec!["tx", "main()", "-o", "tmpname"]),
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
