use move_executor::compile_and_run_scripts;
use utils::tests::anonymous_script_file;

#[test]
fn test_run_two_scripts() {
    let text = r"
script {
    fun test_1() {
        assert(true, 0);
    }
}

script {
    fun test_2() {
        assert(false, 0);
    }
}
    ";

    let mut results =
        compile_and_run_scripts(anonymous_script_file(text), &[], "libra", "0x1", vec![])
            .unwrap();
    assert!(results
        .remove("test_1")
        .unwrap()
        .effects()
        .resources()
        .is_empty());
    assert_eq!(
        results.remove("test_2").unwrap().error(),
        "Execution aborted with code 0 in transaction script"
    );
}

#[test]
fn test_run_script_by_name() {
    let text = r"
script {
    fun test_1() {
        assert(true, 0);
    }
}

script {
    fun test_2() {
        assert(false, 0);
    }
}
    ";

    let mut results = compile_and_run_scripts(
        anonymous_script_file(text),
        &[],
        "libra",
        "0x1",
        vec!["test_2".to_string()],
    )
    .unwrap();
    assert_eq!(
        results.remove("test_2").unwrap().error(),
        "Execution aborted with code 0 in transaction script"
    );
    assert!(results.is_empty());
}
