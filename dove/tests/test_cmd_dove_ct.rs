use fs_extra::file::write_all;
use dove::cmd::tx::Transaction;
mod helper;
use crate::helper::{execute_dove_at, project_start_new_and_build, project_remove};
// @todo Add tests for $ dove ct -t ###, after bug fix
/// $ dove ct
#[test]
fn test_cmd_dove_ct_without_arguments() {
    // Path to dove folder, project and project name
    let project_name = "demoproject_19";
    let project_folder = project_start_new_and_build(project_name);
    // project_folder/scripts/sdemo.move
    write_all(
        &project_folder.join("scripts/sdemo.move"),
        "script {
                    fun main() {
                        assert((1+3)==4,1);
                    }
                }",
    )
    .unwrap();
    let args = &["dove", "tx"];
    execute_dove_at(args, &project_folder).unwrap();
    let tx_path = project_folder.join("target/transactions/main.mvt");
    assert!(
        tx_path.exists(),
        "Transaction not found: {}\r\n[Command] {}",
        tx_path.display(),
        args.join(" "),
    );
    let tx_fmt = format!(
        "{:?}",
        bcs::from_bytes::<Transaction>(std::fs::read(&tx_path).unwrap().as_ref()).unwrap()
    );
    assert!(tx_fmt.contains(" args: []"));
    assert!(tx_fmt.contains(" type_args: []"));
    assert!(tx_fmt.contains(" signers_count: 0"));
    project_remove(&project_folder);
}
/// $ dove ct 'sdemo_4<u8>(16)'
#[test]
fn test_cmd_dove_ct_with_type() {
    // Path to dove folder, project and project name
    let project_name = "demoproject_24";
    let project_folder = project_start_new_and_build(project_name);
    // project_folder/modules/mdemo.move
    write_all(
        &project_folder.join("modules/mdemo.move"),
        "module ModuleDemo{
            struct T1 {}
            struct T2 {}
            struct Demo<T> has drop{
                value:u8
            }
            public fun new<T:drop>(value:u8): Demo<T>{
                Demo<T>{
                    value
                }
            }
        }",
    )
    .unwrap();
    // project_folder/scripts/sdemo.move
    write_all(
        &project_folder.join("scripts/sdemo.move"),
        "script {
            use 0x1::ModuleDemo;
            fun sdemo_4<T:drop>(value:u8) {
                let _tmp:ModuleDemo::Demo<T> = ModuleDemo::new<T>(value);
            }
        }",
    )
    .unwrap();
    let args = &["dove", "tx", "sdemo_4<u8>(16)"];
    execute_dove_at(args, &project_folder).unwrap();
    let tx_path = project_folder.join("target/transactions/sdemo_4.mvt");
    assert!(
        tx_path.exists(),
        "Transaction not found: {}\r\n[Command] {}",
        tx_path.display(),
        args.join(" "),
    );
    let tx_fmt = format!(
        "{:?}",
        bcs::from_bytes::<Transaction>(std::fs::read(&tx_path).unwrap().as_ref()).unwrap()
    );
    assert!(tx_fmt.contains(" args: [U8(16)]"));
    assert!(tx_fmt.contains(" type_args: [U8]"));
    assert!(tx_fmt.contains(" signers_count: 0"));
    project_remove(&project_folder);
}
/// $ dove ct -o z
#[test]
fn test_cmd_dove_ct_with_output_file_name() {
    // Path to dove folder, project and project name
    let project_name = "demoproject_21";
    let project_folder = project_start_new_and_build(project_name);
    // project_folder/scripts/sdemo.move
    write_all(
        &project_folder.join("scripts/sdemo.move"),
        "script {
                    fun main() {
                        assert((1+3)==4,1);
                    }
                }",
    )
    .unwrap();
    let args = &["dove", "tx", "-o", "z"];
    execute_dove_at(args, &project_folder).unwrap();
    let tx_path = project_folder.join("target/transactions/z.mvt");
    assert!(
        tx_path.exists(),
        "Transaction not found: {}\r\n[Command] {}",
        tx_path.display(),
        &args.join(" "),
    );
    project_remove(&project_folder);
}
/// $ dove ct -n test_fun -f sdemo
#[test]
fn test_cmd_dove_ct_with_script_name_arg() {
    // Path to dove folder, project and project name
    let project_name = "demoproject_23";
    let project_folder = project_start_new_and_build(project_name);
    // project_folder/scripts/sdemo.move
    write_all(
        &project_folder.join("scripts/sdemo.move"),
        "script {
                    fun main(_a:u64,_b:u64) { }
                }
                script {
                    fun test_fun() { }
                }",
    )
    .unwrap();
    let args = &["dove", "tx", "-f", "sdemo", "-n", "test_fun"];
    execute_dove_at(args, &project_folder).unwrap();
    let tx_path = project_folder.join("target/transactions/test_fun.mvt");
    assert!(
        tx_path.exists(),
        "Transaction not found: {}\r\n[Command] {}",
        tx_path.display(),
        &args.join(" "),
    );
    project_remove(&project_folder);
}
/// $ dove ct 'test_fun()' -f sdemo
#[test]
fn test_cmd_dove_ct_with_script_name_option() {
    // Path to dove folder, project and project name
    let project_name = "demoproject_3";
    let project_folder = project_start_new_and_build(project_name);
    // project_folder/scripts/sdemo.move
    write_all(
        &project_folder.join("scripts/sdemo.move"),
        "script {
                    fun main(_a:u64,_b:u64) { }
                }
                script {
                    fun test_fun() { }
                }",
    )
    .unwrap();
    let args = &["dove", "tx", "test_fun()", "-f", "sdemo"];
    execute_dove_at(args, &project_folder).unwrap();
    let tx_path = project_folder.join("target/transactions/test_fun.mvt");
    assert!(
        tx_path.exists(),
        "Transaction not found: {}\r\n[Command] {}",
        tx_path.display(),
        args.join(" "),
    );
    project_remove(&project_folder);
}
/// $ dove ct -f sdemo_2
#[test]
fn test_cmd_dove_ct_with_script_file_name() {
    // Path to dove folder, project and project name
    let project_name = "demoproject_20";
    let project_folder = project_start_new_and_build(project_name);
    // project_folder/scripts/sdemo_1.move
    write_all(
        &project_folder.join("scripts/sdemo_1.move"),
        "script {
                    fun sdemo_1() {
                        assert((1+3)==4,1);
                    }
                }",
    )
    .unwrap();
    // project_folder/scripts/sdemo_2.move
    write_all(
        &project_folder.join("scripts/sdemo_2.move"),
        "script {
                    fun sdemo_2() {
                        assert((2+2)==4,1);
                    }
                }",
    )
    .unwrap();
    let args = &["dove", "tx", "-f", "sdemo_2"];
    execute_dove_at(args, &project_folder).unwrap();
    let tx_path = project_folder.join("target/transactions/sdemo_2.mvt");
    assert!(
        tx_path.exists(),
        "Transaction not found: {}\r\n[Command] {}",
        tx_path.display(),
        args.join(" "),
    );
    project_remove(&project_folder);
}
/// $ dove ct -a 1 2
#[test]
fn test_cmd_dove_ct_with_script_method_args() {
    // Path to dove folder, project and project name
    let project_name = "demoproject_1";
    let project_folder = project_start_new_and_build(project_name);
    // project_folder/scripts/sdemo.move
    write_all(
        &project_folder.join("scripts/sdemo.move"),
        "script {
                    fun main(_a1:u64,_a2:u64) { }
                }",
    )
    .unwrap();
    // $ dove ct -a 1 2
    let args = &["dove", "tx", "-a", "1", "2"];
    execute_dove_at(args, &project_folder).unwrap();
    let tx_path = project_folder.join("target/transactions/main.mvt");
    assert!(
        tx_path.exists(),
        "Transaction not found: {}\r\n[Command] {}",
        tx_path.display(),
        &args.join(" "),
    );
    let tx_fmt = format!(
        "{:?}",
        bcs::from_bytes::<Transaction>(std::fs::read(&tx_path).unwrap().as_ref()).unwrap()
    );
    assert!(tx_fmt.contains(" args: [U64(1), U64(2)]"));
    assert!(tx_fmt.contains(" type_args: []"));
    assert!(tx_fmt.contains(" signers_count: 0"));
    project_remove(&project_folder);
}
/// $ dove ct 'main(1,2)'
#[test]
fn test_cmd_dove_ct_with_script_method_args_option() {
    // Path to dove folder, project and project name
    let project_name = "demoproject_2";
    let project_folder = project_start_new_and_build(project_name);
    // project_folder/scripts/sdemo.move
    write_all(
        &project_folder.join("scripts/sdemo.move"),
        "script {
                    fun main(_a1:u64,_a2:u64) { }
                }",
    )
    .unwrap();
    // $ dove ct 'main(1,2)'
    let args = &["dove", "tx", "main(1,2)"];
    execute_dove_at(args, &project_folder).unwrap();
    let tx_path = project_folder.join("target/transactions/main.mvt");
    assert!(
        tx_path.exists(),
        "Transaction not found: {}\r\n[Command] {}",
        tx_path.display(),
        args.join(" "),
    );
    let tx_fmt = format!(
        "{:?}",
        bcs::from_bytes::<Transaction>(std::fs::read(&tx_path).unwrap().as_ref()).unwrap()
    );
    assert!(tx_fmt.contains(" args: [U64(1), U64(2)]"));
    assert!(tx_fmt.contains(" type_args: []"));
    assert!(tx_fmt.contains(" signers_count: 0"));
    project_remove(&project_folder);
}
