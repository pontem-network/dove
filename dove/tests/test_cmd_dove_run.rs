use fs_extra::file::write_all;
use dove::tests_helper::{
    execute_dove_at, project_start_new_and_build, project_remove,
    project_start_new_and_add_dependencies, execute_dove_bin_at,
};
use std::path::{PathBuf, Path};
use toml::value::Map;
use toml::Value;

/// $ dove run
#[test]
fn test_cmd_dove_run_without_arguments() {
    let project_folder =
        create_project_with_a_single_script_without_parameters("project_run_without_arguments");
    let args = &["dove", "run"];
    execute_dove_at(args, &project_folder).unwrap();
    project_remove(&project_folder);
}

/// dove run 'noparams()'
/// dove run 'noparams()' -s 0x1
/// dove run -f noparams
/// dove run -n noparams
/// dove run 'withnums(1,2)'
/// dove run 'withnums(1,2)' --signers 0x1 0x2
/// dove run --file withnums -a 1 2
/// dove run --name withnums --args 1 2
#[test]
fn test_cmd_dove_run_with_call_and_arguments() {
    let project_folder = create_project_with_any_scripts("project_run_with_call_and_arguments");
    for args in vec![
        vec!["dove", "run", "noparams()"],
        vec!["dove", "run", "noparams()", "-s", "0x1"],
        vec!["dove", "run", "-f", "noparams"],
        vec!["dove", "run", "-n", "noparams"],
        vec!["dove", "run", "withnums(1,2)"],
        vec!["dove", "run", "withnums(1,2)", "--signers", "0x1", "0x2"],
        vec!["dove", "run", "--file", "withnums", "-a", "1", "2"],
        vec!["dove", "run", "--name", "withnums", "--args", "1", "2"],
    ] {
        execute_dove_at(args.as_ref(), &project_folder).unwrap();
    }
    project_remove(&project_folder);
}

/// dove run 'script_1()'
/// dove run 'script_2(1)'
#[test]
fn test_cmd_dove_run_multiple_scripts() {
    let project_folder = create_project_with_any_scripts("project_run_multiple_scripts");
    for args in vec![
        vec!["dove", "run", "script_1()"],
        vec!["dove", "run", "script_2(1)"],
    ] {
        execute_dove_at(args.as_ref(), &project_folder).unwrap();
    }
    project_remove(&project_folder);
}

#[test]
fn test_cmd_dove_run_dependency_with_git_tag() {
    for v in 1..=2 {
        let project_folder = create_project_for_test_dependency(
            "project_run_dependency_with_git_tag",
            None,
            Some(&format!("v{}", v)),
            None,
        );

        // project_folder/scripts/version.move
        add_sctipt_getversion(&project_folder);

        let output = execute_dove_bin_at(
            env!("CARGO_BIN_EXE_dove"),
            &["dove", "run", "version()"],
            &project_folder,
        )
        .unwrap();

        assert!(output.contains(&format!("[debug] {}", v)));

        project_remove(&project_folder);
    }
}

#[test]
fn test_cmd_dove_run_dependency_with_rev() {
    for (value, rev) in &[
        (1, "049200421c880f9f4269d6406c2b1537891b23c7"),
        (2, "c276307c355d3c72e3daeb80f46e21272c6fab97"),
    ] {
        let project_folder = create_project_for_test_dependency(
            "project_run_dependency_with_rev",
            None,
            None,
            Some(rev),
        );

        // project_folder/scripts/version.move
        add_sctipt_getversion(&project_folder);

        let output = execute_dove_bin_at(
            env!("CARGO_BIN_EXE_dove"),
            &["dove", "run", "version()"],
            &project_folder,
        )
        .unwrap();

        assert!(output.contains(&format!("[debug] {}", value)));

        project_remove(&project_folder);
    }
}

#[test]
fn test_cmd_dove_run_dependencies_in_dependencies() {
    let project_folder = create_project_for_test_dependency(
        "project_run_dependencies_in_dependencies",
        Some("path"),
        None,
        None,
    );

    // project_folder/scripts/sum.move
    write_all(
        &project_folder.join("scripts").join("sum.move"),
        "script {\
            use 0x1::Sum;\
            use 0x1::Debug;\
            fun sum(){\
                Debug::print<u8>(&Sum::sum(1,2));\
            }\
        }",
    )
    .unwrap();

    let output = execute_dove_bin_at(
        env!("CARGO_BIN_EXE_dove"),
        &["dove", "run", "sum()"],
        &project_folder,
    )
    .unwrap();

    assert!(output.contains("[debug] 3"));

    project_remove(&project_folder);
}

#[test]
fn test_cmd_dove_run_cyclic_dependency_in_the_repository() {
    let project_folder = create_project_for_test_dependency(
        "project_run_cyclic_dependency_in_the_repository",
        Some("test_rec"),
        None,
        None,
    );

    execute_dove_bin_at(
        env!("CARGO_BIN_EXE_dove"),
        &["dove", "run", "version()"],
        &project_folder,
    )
    .unwrap();

    project_remove(&project_folder);
}

#[test]
fn test_cmd_dove_run_running_a_script_in_dependencies() {
    let project_folder = create_project_for_test_dependency(
        "project_run_running_a_script_in_dependencies",
        Some("script_main"),
        None,
        None,
    );

    let output = execute_dove_bin_at(
        env!("CARGO_BIN_EXE_dove"),
        &["dove", "run", "main()"],
        &project_folder,
    )
    .unwrap();

    assert_eq!(output.matches("[debug]").count(), 1);

    project_remove(&project_folder);
}

fn create_project_with_a_single_script_without_parameters(project_name: &str) -> PathBuf {
    let project_folder = project_start_new_and_build(project_name);
    // project_folder/scripts/sdemo.move
    write_all(
        &project_folder.join("scripts").join("sdemo.move"),
        "script { fun main() { assert((1+3)==4,1); } }",
    )
    .unwrap();
    project_folder
}

fn create_project_with_any_scripts(project_name: &str) -> PathBuf {
    let project_folder = project_start_new_and_build(project_name);
    // project_folder/scripts/noparams.move
    write_all(
        &project_folder.join("scripts").join("noparams.move"),
        "script { fun noparams() { } }",
    )
    .unwrap();
    // project_folder/scripts/withnums.move
    write_all(
        &project_folder.join("scripts").join("withnums.move"),
        "script { fun withnums(x:u64,y:u64) { let _result = x + y; }  }",
    )
    .unwrap();
    // project_folder/scripts/multiple.move
    write_all(
        &project_folder.join("scripts").join("multiple.move"),
        "script { fun script_1() {  } }\n\
                script { fun script_2(_a:u64) {  } }",
    )
    .unwrap();
    project_folder
}

fn create_project_for_test_dependency(
    project_name: &str,
    branch: Option<&str>,
    tag: Option<&str>,
    rev: Option<&str>,
) -> PathBuf {
    let mut rep = Map::new();
    rep.insert(
        "git".to_string(),
        Value::String("https://github.com/pontem-network/test-dove-dependency".to_string()),
    );
    if let Some(branch) = branch {
        rep.insert("branch".to_string(), Value::String(branch.to_string()));
    }
    if let Some(tag) = tag {
        rep.insert("tag".to_string(), Value::String(tag.to_string()));
    }
    if let Some(rev) = rev {
        rep.insert("rev".to_string(), Value::String(rev.to_string()));
    }
    let project_folder = project_start_new_and_add_dependencies(project_name, vec![rep]);

    // project_folder/scripts/main.move
    write_all(
        &project_folder.join("scripts").join("main.move"),
        "script {\
                use 0x1::Debug;\
                fun main(){\
                    Debug::print<u8>(&1);\
                }\
            }",
    )
    .unwrap();

    project_folder
}

fn add_sctipt_getversion(project_folder: &Path) {
    // project_folder/scripts/version.move
    write_all(
        &project_folder.join("scripts").join("version.move"),
        "script {\
            use 0x1::Version;\
            use 0x1::Debug;\
            fun version(){\
                Debug::print<u8>(&Version::get());\
            }\
        }",
    )
    .unwrap();
}
