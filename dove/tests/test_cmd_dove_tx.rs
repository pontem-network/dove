use fs_extra::file::write_all;
use dove::transaction::Transaction;
use dove::tests_helper::{execute_dove_at, project_start_new_and_build, project_remove};
use std::fs::remove_file;

/// $ dove tx
#[test]
fn test_cmd_dove_tx_without_arguments() {
    // Path to dove folder, project and project name
    let project_name = "project_tx_without_arguments";
    let project_folder = project_start_new_and_build(project_name);
    // project_folder/scripts/sdemo.move
    write_all(
        &project_folder.join("scripts").join("sdemo.move"),
        "script {
                    fun main() {
                        assert((1+3)==4,1);
                    }
                }",
    )
    .unwrap();
    let args = &["dove", "tx"];
    execute_dove_at(args, &project_folder).unwrap();
    let tx_path = project_folder
        .join("artifacts")
        .join("transactions")
        .join("main.mvt");
    assert!(
        tx_path.exists(),
        "Transaction not found: {}\n[Command] {}",
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

/// $ dove tx -n sdemo_4 -a 16 -t u8
/// $ dove tx 'sdemo_4()' -a 16 -t u8
/// $ dove tx 'sdemo_4(16)' -t u8
/// $ dove tx 'sdemo_4<u8>(16)'
#[test]
fn test_cmd_dove_tx_with_type() {
    // Path to dove folder, project and project name
    let project_name = "project_tx_with_type";
    let project_folder = project_start_new_and_build(project_name);
    // project_folder/modules/mdemo.move
    write_all(
        &project_folder.join("modules").join("mdemo.move"),
        "module ModuleDemo {
            struct T1 {}
            struct T2 {}
            struct Demo<T> has drop {
                value: u8
            }
            public fun new<T: drop>(value: u8): Demo<T> {
                Demo<T>{
                    value
                }
            }
        }",
    )
    .unwrap();

    // project_folder/scripts/sdemo.move
    write_all(
        &project_folder.join("scripts").join("sdemo.move"),
        "script {
            use 0x1::ModuleDemo;
            fun sdemo_4<T:drop>(value:u8) {
                let _tmp: ModuleDemo::Demo<T> = ModuleDemo::new<T>(value);
            }
        }",
    )
    .unwrap();

    let tx_path = project_folder
        .join("artifacts")
        .join("transactions")
        .join("sdemo_4.mvt");

    // u8
    for args in vec![
        vec!["dove", "tx", "-n", "sdemo_4", "-a", "16", "-t", "u8"],
        vec!["dove", "tx", "sdemo_4()", "-a", "16", "-t", "u8"],
        vec!["dove", "tx", "sdemo_4(16)", "-t", "u8"],
        vec!["dove", "tx", "sdemo_4<u8>(16)"],
    ] {
        execute_dove_at(args.as_ref(), &project_folder).unwrap();
        assert!(
            tx_path.exists(),
            "Transaction not found: {}\n[Command] {}",
            tx_path.display(),
            args.join(" "),
        );
        let tx_fmt = format!(
            "{:?}",
            bcs::from_bytes::<Transaction>(std::fs::read(&tx_path).unwrap().as_ref()).unwrap()
        );
        assert!(tx_fmt.contains(" args: [[16]]"));
        assert!(tx_fmt.contains(" type_args: [U8]"));
        assert!(tx_fmt.contains(" signers_count: 0"));
        remove_file(&tx_path).unwrap();
    }

    // 0x1::ModuleDemo::T1
    for args in vec![
        vec!["dove", "tx", "sdemo_4(16)", "-t", "0x1::ModuleDemo::T1"],
        vec!["dove", "tx", "sdemo_4<0x1::ModuleDemo::T1>(16)"],
    ] {
        execute_dove_at(args.as_ref(), &project_folder).unwrap();
        assert!(
            tx_path.exists(),
            "Transaction not found: {}\n[Command] {}",
            tx_path.display(),
            args.join(" "),
        );
        let tx_fmt = format!(
            "{:?}",
            bcs::from_bytes::<Transaction>(std::fs::read(&tx_path).unwrap().as_ref()).unwrap()
        );
        assert!(tx_fmt.contains(" args: [[16]]"));
        assert!(tx_fmt.contains(" module: Identifier(\"ModuleDemo\")"));
        assert!(tx_fmt.contains(" name: Identifier(\"T1\")"));
        assert!(tx_fmt.contains(" signers_count: 0"));
        remove_file(&tx_path).unwrap();
    }

    project_remove(&project_folder);
}

/// $ dove tx -o z
#[test]
fn test_cmd_dove_tx_with_output_file_name() {
    // Path to dove folder, project and project name
    let project_name = "project_tx_with_output_file_name";
    let project_folder = project_start_new_and_build(project_name);
    // project_folder/scripts/sdemo.move
    write_all(
        &project_folder.join("scripts").join("sdemo.move"),
        "script {
                    fun main() {
                        assert((1+3)==4,1);
                    }
                }",
    )
    .unwrap();
    let args = &["dove", "tx", "-o", "z"];
    execute_dove_at(args, &project_folder).unwrap();
    let tx_path = project_folder
        .join("artifacts")
        .join("transactions")
        .join("z.mvt");
    assert!(
        tx_path.exists(),
        "Transaction not found: {}\n[Command] {}",
        tx_path.display(),
        &args.join(" "),
    );
    project_remove(&project_folder);
}

/// $ dove tx -n test_fun -f sdemo
#[test]
fn test_cmd_dove_tx_with_script_name_arg() {
    // Path to dove folder, project and project name
    let project_name = "project_tx_with_script_name_arg";
    let project_folder = project_start_new_and_build(project_name);
    // project_folder/scripts/sdemo.move
    write_all(
        &project_folder.join("scripts").join("sdemo.move"),
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
    let tx_path = project_folder
        .join("artifacts")
        .join("transactions")
        .join("test_fun.mvt");
    assert!(
        tx_path.exists(),
        "Transaction not found: {}\n[Command] {}",
        tx_path.display(),
        &args.join(" "),
    );
    project_remove(&project_folder);
}

/// $ dove tx 'test_fun()' -f sdemo
#[test]
fn test_cmd_dove_tx_with_script_name_option() {
    // Path to dove folder, project and project name
    let project_name = "project_tx_with_script_name_option";
    let project_folder = project_start_new_and_build(project_name);
    // project_folder/scripts/sdemo.move
    write_all(
        &project_folder.join("scripts").join("sdemo.move"),
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
    let tx_path = project_folder
        .join("artifacts")
        .join("transactions")
        .join("test_fun.mvt");
    assert!(
        tx_path.exists(),
        "Transaction not found: {}\n[Command] {}",
        tx_path.display(),
        args.join(" "),
    );
    project_remove(&project_folder);
}

/// $ dove tx -f sdemo_2
#[test]
fn test_cmd_dove_tx_with_script_file_name() {
    // Path to dove folder, project and project name
    let project_name = "project_tx_with_script_file_name";
    let project_folder = project_start_new_and_build(project_name);
    // project_folder/scripts/sdemo_1.move
    write_all(
        &project_folder.join("scripts").join("sdemo_1.move"),
        "script {
                    fun sdemo_1() {
                        assert((1+3)==4,1);
                    }
                }",
    )
    .unwrap();
    // project_folder/scripts/sdemo_2.move
    write_all(
        &project_folder.join("scripts").join("sdemo_2.move"),
        "script {
                    fun sdemo_2() {
                        assert((2+2)==4,1);
                    }
                }",
    )
    .unwrap();
    let args = &["dove", "tx", "-f", "sdemo_2"];
    execute_dove_at(args, &project_folder).unwrap();
    let tx_path = project_folder
        .join("artifacts")
        .join("transactions")
        .join("sdemo_2.mvt");
    assert!(
        tx_path.exists(),
        "Transaction not found: {}\n[Command] {}",
        tx_path.display(),
        args.join(" "),
    );
    project_remove(&project_folder);
}

/// $ dove tx -a 1 2
#[test]
fn test_cmd_dove_tx_with_script_method_args() {
    // Path to dove folder, project and project name
    let project_name = "project_tx_with_script_method_args";
    let project_folder = project_start_new_and_build(project_name);
    // project_folder/scripts/sdemo.move
    write_all(
        &project_folder.join("scripts").join("sdemo.move"),
        "script {
                    fun main(a1:u64,a2:u64) { assert((a1!=a2),1); }
                }",
    )
    .unwrap();
    // $ dove tx -a 1 2
    let args = &["dove", "tx", "-a", "1", "2"];
    execute_dove_at(args, &project_folder).unwrap();
    let tx_path = project_folder
        .join("artifacts")
        .join("transactions")
        .join("main.mvt");
    assert!(
        tx_path.exists(),
        "Transaction not found: {}\n[Command] {}",
        tx_path.display(),
        &args.join(" "),
    );
    let tx_fmt = format!(
        "{:?}",
        bcs::from_bytes::<Transaction>(std::fs::read(&tx_path).unwrap().as_ref()).unwrap()
    );
    assert!(tx_fmt.contains(" args: [[1, 0, 0, 0, 0, 0, 0, 0], [2, 0, 0, 0, 0, 0, 0, 0]]"));
    assert!(tx_fmt.contains(" type_args: []"));
    assert!(tx_fmt.contains(" signers_count: 0"));
    project_remove(&project_folder);
}

/// $ dove tx 'main(1,2)'
#[test]
fn test_cmd_dove_tx_with_script_method_args_option() {
    // Path to dove folder, project and project name
    let project_name = "project_tx_with_script_method_args_option";
    let project_folder = project_start_new_and_build(project_name);
    // project_folder/scripts/sdemo.move
    write_all(
        &project_folder.join("scripts").join("sdemo.move"),
        "script {
                    fun main(_a1:u64,_a2:u64) { }
                }",
    )
    .unwrap();
    // $ dove tx 'main(1,2)'
    let args = &["dove", "tx", "main(1,2)"];
    execute_dove_at(args, &project_folder).unwrap();
    let tx_path = project_folder
        .join("artifacts")
        .join("transactions")
        .join("main.mvt");
    assert!(
        tx_path.exists(),
        "Transaction not found: {}\n[Command] {}",
        tx_path.display(),
        args.join(" "),
    );
    let tx_fmt = format!(
        "{:?}",
        bcs::from_bytes::<Transaction>(std::fs::read(&tx_path).unwrap().as_ref()).unwrap()
    );
    assert!(tx_fmt.contains(" args: [[1, 0, 0, 0, 0, 0, 0, 0], [2, 0, 0, 0, 0, 0, 0, 0]]"));
    assert!(tx_fmt.contains(" type_args: []"));
    assert!(tx_fmt.contains(" signers_count: 0"));
    project_remove(&project_folder);
}

/// $ dove tx 'script_1(1,2)'
#[test]
fn test_cmd_dove_tx_multiple_scripts() {
    // Path to dove folder, project and project name
    let project_name = "project_tx_multiple_scripts";
    let project_folder = project_start_new_and_build(project_name);
    // project_folder/scripts/multiple.move
    write_all(
        &project_folder.join("scripts").join("multiple.move"),
        "script { fun script_1() {  } }\n\
                script { fun script_2(_a:u64) {  } }",
    )
    .unwrap();
    let args = &["dove", "tx", "script_1()"];
    execute_dove_at(args, &project_folder).unwrap();

    let tx_path = project_folder
        .join("artifacts")
        .join("transactions")
        .join("script_1.mvt");
    assert!(
        tx_path.exists(),
        "Transaction not found: {}\n[Command] {}",
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

#[test]
fn test_cmd_dove_tx_signer() {
    // Path to dove folder, project and project name
    let project_name = "project_tx_signer";
    let project_folder = project_start_new_and_build(project_name);
    // project_folder/scripts/signer.move
    write_all(
        &project_folder.join("scripts").join("signer.move"),
        "script {\
            fun main(_a:signer, _b:signer, _c:u8){}
        }",
    )
    .unwrap();

    let tx_path = project_folder
        .join("artifacts")
        .join("transactions")
        .join("main.mvt");

    for args in &[
        vec!["dove", "tx", "main()", "-a", "8"],
        vec!["dove", "tx", "main()", "-a", "0x1", "0x2", "8"],
    ] {
        if tx_path.exists() {
            remove_file(&tx_path).unwrap();
        }

        execute_dove_at(args, &project_folder).unwrap();

        assert!(
            tx_path.exists(),
            "Transaction not found: {}\n[Command] {}",
            tx_path.display(),
            args.join(" "),
        );
        let tx_fmt = format!(
            "{:?}",
            bcs::from_bytes::<Transaction>(std::fs::read(&tx_path).unwrap().as_ref()).unwrap()
        );

        assert!(tx_fmt.contains(" args: [[8]]"));
        assert!(tx_fmt.contains(" type_args: []"));
        assert!(tx_fmt.contains(" signers_count: 2"));
    }

    project_remove(&project_folder);
}
