use helper::{new_demo_project, delete_project, execute_dove_at, build};

mod helper;

/// Testing with a filter
///
/// $ dove test -f Test1 <<- success
/// $ dove test -f Test3 <<- error
#[test]
fn test_cmd_dove_test() {
    let project_name = "project_test";
    let project_path = new_demo_project(&project_name).unwrap();
    build(&project_path).unwrap();

    // Success
    for test_name in ["Test1", "Test2"] {
        execute_dove_at(&["test", "-f", &test_name], &project_path).unwrap();
    }
    // Error
    assert!(execute_dove_at(&["test", "-f", "Test3"], &project_path).is_err());

    delete_project(&project_path).unwrap();
}

/// Display a list of tests
/// $ dove test -l
#[test]
fn test_cmd_dove_test_list() {
    let project_name = "project_test_list";
    let project_path = new_demo_project(&project_name).unwrap();
    build(&project_path).unwrap();

    let output = execute_dove_at(&["test", "-l"], &project_path).unwrap();
    for name in [
        "0x2::Test1::success",
        "0x2::Test2::success",
        "0x2::Test3::error",
    ] {
        assert!(output.contains(name));
    }

    delete_project(&project_path).unwrap();
}
