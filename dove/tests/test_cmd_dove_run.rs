// @todo

use dove::tests_helper::{
    execute_dove_at, project_start_new_and_build, project_remove, execute_dove_bin_at,
};
use std::path::PathBuf;

/// $ dove run
#[test]
fn test_cmd_dove_run_without_arguments() {
    let project_folder =
        create_project_with_a_single_script_without_parameters("project_run_without_arguments");
    let args = &["dove", "run", "main"];
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
        vec!["dove", "run", "noparams", "-f", "noparams.move"],
        vec!["dove", "run", "withnums(1, 2)"],
        vec!["dove", "run", "withnums(1, 2)"],
        vec![
            "dove",
            "run",
            "withnums",
            "--file",
            "withnums.move",
            "-a",
            "1",
            "2",
        ],
        vec!["dove", "run", "withnums", "--args", "1", "2"],
    ] {
        execute_dove_at(args.as_ref(), &project_folder)
            .map_err(|err| {
                format!(
                    "Failed to execute '{}'. Error:{}",
                    args.join(" "),
                    err.to_string()
                )
            })
            .unwrap();
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

/// dove run 'main()'
#[test]
fn test_cmd_dove_run_script_with_attr_dialect() {
    let path_dove = env!("CARGO_BIN_EXE_dove");
    for (dialect, result) in [("diem", 1), ("pont", 2), ("dfinance", 3)] {
        let project_folder = create_project_with_attr_dialect(
            "project_run_script_with_attr_dialect",
            Some(dialect),
        );
        let args = &["dove", "run", "main()"];
        let stdout = execute_dove_bin_at(path_dove, args, &project_folder)
            .map_err(|err| {
                format!(
                    "Failed to execute '{}'. Error:{}",
                    args.join(" "),
                    err.to_string()
                )
            })
            .unwrap();
        assert!(stdout.contains(format!("[debug] {}", result).as_str()));
        project_remove(&project_folder);
    }
}

fn create_project_with_a_single_script_without_parameters(project_name: &str) -> PathBuf {
    let project_folder = project_start_new_and_build(project_name, None);
    // project_folder/scripts/sdemo.move
    write_all(
        &project_folder.join("scripts").join("sdemo.move"),
        "script { fun main() { assert((1+3)==4,1); } }",
    )
    .unwrap();
    project_folder
}

fn create_project_with_any_scripts(project_name: &str) -> PathBuf {
    let project_folder = project_start_new_and_build(project_name, None);
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

fn create_project_with_attr_dialect(project_name: &str, dialect: Option<&str>) -> PathBuf {
    let project_folder = project_start_new_and_build(project_name, dialect);
    // project_folder/scripts/main.move
    write_all(
        &project_folder.join("scripts").join("main.move"),
        "script {
            use 0x1::Debug;
            use 0x1::Version;
            fun main(){
                Debug::print<u8>(&Version::get());
            }
        }",
    )
    .unwrap();
    // project_folder/modules/mdiem.move
    write_all(
        &project_folder.join("modules").join("mdiem.move"),
        "
        #![dialect(diem)]
        /*
         #![dialect(dfinance)]
        */
        module 0x1::Version{
            public fun get():u8{
                1
            }
        }",
    )
    .unwrap();
    // project_folder/modules/mpont.move
    write_all(
        &project_folder.join("modules").join("mpont.move"),
        "
        #![dialect(pont)]
        /*
         // #![dialect(diem)]
        */
        module 0x1::Version{
            public fun get():u8{
                2
            }
        }",
    )
    .unwrap();
    // project_folder/modules/mdfinance.move
    write_all(
        &project_folder.join("modules").join("mdfinance.move"),
        "
        #![dialect(dfinance)]
        // #![dialect(pont)]
        module 0x1::Version{
            public fun get():u8{
                3
            }
        }",
    )
    .unwrap();

    project_folder
}
