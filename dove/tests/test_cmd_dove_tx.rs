// @todo

use std::fs;
use std::fs::remove_file;
use dove::tests_helper::{execute_dove_at, project_remove, project_start_new_and_build};
use move_core_types::language_storage::{TypeTag, StructTag, CORE_CODE_ADDRESS};
use move_core_types::identifier::Identifier;
use dove::tx::model::{Transaction, Signer, V1};

/// $ dove tx
#[test]
fn test_cmd_dove_tx_without_arguments() {
    // Path to dove folder, project and project name
    let project_name = "project_tx_without_arguments";
    let project_folder = project_start_new_and_build(project_name, None);
    // project_folder/scripts/sdemo.move
    fs::write(
        &project_folder.join("scripts").join("sdemo.move"),
        "script {
                    fun main() {
                        assert((1+3)==4,1);
                    }
                }",
    )
    .unwrap();
    let args = &["dove", "tx", "main"];
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
    let tx = bcs::from_bytes::<Transaction>(&fs::read(&tx_path).unwrap())
        .unwrap()
        .inner();
    assert!(tx.args.is_empty());
    assert!(tx.type_args.is_empty());
    assert!(tx.signers.is_empty());
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
    let project_folder = project_start_new_and_build(project_name, None);
    // project_folder/modules/mdemo.move
    write_all(
        &project_folder.join("modules").join("mdemo.move"),
        "module 0x1::ModuleDemo {
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
        vec!["dove", "tx", "sdemo_4", "-a", "16", "-t", "u8"],
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
        let tx = bcs::from_bytes::<Transaction>(&fs::read(&tx_path).unwrap())
            .unwrap()
            .inner();
        assert_eq!(tx.args, vec![vec![16]]);
        assert_eq!(tx.type_args, vec![TypeTag::U8]);
        assert_eq!(tx.signers, vec![]);
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
        let tx = bcs::from_bytes::<Transaction>(std::fs::read(&tx_path).unwrap().as_ref())
            .unwrap()
            .inner();
        assert_eq!(tx.args, vec![vec![16]]);
        assert_eq!(
            tx.type_args,
            vec![TypeTag::Struct(StructTag {
                address: CORE_CODE_ADDRESS,
                module: Identifier::new("ModuleDemo").unwrap(),
                name: Identifier::new("T1").unwrap(),
                type_params: vec![]
            })]
        );
        assert_eq!(tx.signers, vec![]);
        remove_file(&tx_path).unwrap();
    }

    project_remove(&project_folder);
}

/// $ dove tx -o z
#[test]
fn test_cmd_dove_tx_with_output_file_name() {
    // Path to dove folder, project and project name
    let project_name = "project_tx_with_output_file_name";
    let project_folder = project_start_new_and_build(project_name, None);
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
    let args = &["dove", "tx", "main", "-o", "z"];
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
    let project_folder = project_start_new_and_build(project_name, None);
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
    let args = &["dove", "tx", "test_fun", "-f", "sdemo.move"];
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
    let project_folder = project_start_new_and_build(project_name, None);
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
    let args = &["dove", "tx", "test_fun()", "-f", "sdemo.move"];
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
    let project_folder = project_start_new_and_build(project_name, None);
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
    let args = &["dove", "tx", "sdemo_2", "-f", "sdemo_2.move"];
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
    let project_folder = project_start_new_and_build(project_name, None);
    // project_folder/scripts/sdemo.move
    write_all(
        &project_folder.join("scripts").join("sdemo.move"),
        "script {
                    fun main(a1: u64, a2: u64) { assert((a1!=a2),1); }
                }",
    )
    .unwrap();
    // $ dove tx -a 1 2
    let args = &["dove", "tx", "main", "-a", "1", "2"];
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
    let tx = bcs::from_bytes::<Transaction>(&fs::read(&tx_path).unwrap())
        .unwrap()
        .inner();
    assert_eq!(
        tx.args,
        vec![vec![1, 0, 0, 0, 0, 0, 0, 0], vec![2, 0, 0, 0, 0, 0, 0, 0]]
    );
    assert!(tx.type_args.is_empty());
    assert_eq!(tx.signers, vec![]);
    project_remove(&project_folder);
}

/// $ dove tx 'main(1,2)'
#[test]
fn test_cmd_dove_tx_with_script_method_args_option() {
    // Path to dove folder, project and project name
    let project_name = "project_tx_with_script_method_args_option";
    let project_folder = project_start_new_and_build(project_name, None);
    // project_folder/scripts/sdemo.move
    write_all(
        &project_folder.join("scripts").join("sdemo.move"),
        "script {
                    fun main(_a1: u64, _a2: u64) { }
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
    let tx = bcs::from_bytes::<Transaction>(&fs::read(&tx_path).unwrap())
        .unwrap()
        .inner();

    assert_eq!(
        tx.args,
        vec![vec![1, 0, 0, 0, 0, 0, 0, 0], vec![2, 0, 0, 0, 0, 0, 0, 0]]
    );
    assert!(tx.type_args.is_empty());
    assert_eq!(tx.signers, vec![]);
    project_remove(&project_folder);
}

/// $ dove tx 'script_1(1,2)'
#[test]
fn test_cmd_dove_tx_multiple_scripts() {
    // Path to dove folder, project and project name
    let project_name = "project_tx_multiple_scripts";
    let project_folder = project_start_new_and_build(project_name, None);
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
    let tx = bcs::from_bytes::<Transaction>(&fs::read(&tx_path).unwrap())
        .unwrap()
        .inner();
    assert!(tx.args.is_empty());
    assert!(tx.type_args.is_empty());
    assert!(tx.signers.is_empty());
    project_remove(&project_folder);
}

#[test]
fn test_cmd_dove_tx_signer() {
    // Path to dove folder, project and project name
    let project_name = "project_tx_signer";
    let project_folder = project_start_new_and_build(project_name, None);
    // project_folder/scripts/signer.move
    write_all(
        &project_folder.join("scripts").join("signer.move"),
        "script {\
            fun main(_a:signer, _b:signer, _c:u8){}
        }

        script {
            fun signers_tr_and_rt_with_user(_rt: signer, _tr: signer, _usr: signer) {
            }
        }
        ",
    )
    .unwrap();

    let tx_path = project_folder
        .join("artifacts")
        .join("transactions")
        .join("main.mvt");

    let perform = |cmd: &[&str]| -> V1 {
        if tx_path.exists() {
            remove_file(&tx_path).unwrap();
        }
        execute_dove_at(cmd, &project_folder).unwrap();
        assert!(
            tx_path.exists(),
            "Transaction not found: {}\n[Command] {}",
            tx_path.display(),
            cmd.join(" "),
        );
        bcs::from_bytes::<Transaction>(&fs::read(&tx_path).unwrap())
            .unwrap()
            .inner()
    };

    let tx = perform(&["dove", "tx", "main()", "-a", "8"]);
    assert_eq!(tx.args, vec![vec![0x8]]);
    assert!(tx.type_args.is_empty());
    assert_eq!(tx.signers, vec![Signer::Placeholder, Signer::Placeholder]);

    let tx = perform(&["dove", "tx", "main(_, 8)"]);
    assert_eq!(tx.args, vec![vec![0x8]]);
    assert!(tx.type_args.is_empty());
    assert_eq!(tx.signers, vec![Signer::Placeholder, Signer::Placeholder]);

    let tx = perform(&["dove", "tx", "main(_, _, 8)"]);
    assert_eq!(tx.args, vec![vec![0x8]]);
    assert!(tx.type_args.is_empty());
    assert_eq!(tx.signers, vec![Signer::Placeholder, Signer::Placeholder]);

    let tx = perform(&["dove", "tx", "main(rt, _, 8)"]);
    assert_eq!(tx.args, vec![vec![0x8]]);
    assert!(tx.type_args.is_empty());
    assert_eq!(tx.signers, vec![Signer::Root, Signer::Placeholder]);

    let tx = perform(&["dove", "tx", "main(root, 8)"]);
    assert_eq!(tx.args, vec![vec![0x8]]);
    assert!(tx.type_args.is_empty());
    assert_eq!(tx.signers, vec![Signer::Root, Signer::Placeholder]);

    let tx = perform(&["dove", "tx", "main(_, tr, 8)"]);
    assert_eq!(tx.args, vec![vec![0x8]]);
    assert!(tx.type_args.is_empty());
    assert_eq!(tx.signers, vec![Signer::Placeholder, Signer::Treasury]);

    let tx = perform(&[
        "dove",
        "tx",
        "signers_tr_and_rt_with_user(rt, tr)",
        "-o",
        "main.mvt",
    ]);
    assert!(tx.args.is_empty());
    assert!(tx.type_args.is_empty());
    assert_eq!(
        tx.signers,
        vec![Signer::Root, Signer::Treasury, Signer::Placeholder]
    );

    project_remove(&project_folder);
}
