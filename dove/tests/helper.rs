#![allow(dead_code)]

use std::path::{PathBuf, Path};
use std::fs;
use std::fs::{remove_dir_all, create_dir};
use std::collections::HashMap;
use anyhow::{Result, ensure};
use std::io::Write;

/// get tmp_folder, project_folder and remove project folder if exist
pub fn pre_start_dove_new(project_name: &str) -> Result<(PathBuf, PathBuf)> {
    let tmp_folder = std::env::temp_dir();
    let project_folder = tmp_folder.join(project_name);
    delete_project(&project_folder)?;
    Ok((tmp_folder, project_folder))
}

/// get tmp_folder, project_folder and remove project folder if exist
pub fn pre_start_dove_init(project_name: &str) -> Result<PathBuf> {
    let tmp_folder = std::env::temp_dir();
    let project_folder = tmp_folder.join(project_name);
    delete_project(&project_folder)?;
    create_dir(&project_folder)?;
    Ok(project_folder)
}

/// remove project
pub fn delete_project(project_path: &Path) -> Result<()> {
    if project_path.exists() {
        remove_dir_all(project_path)?;
    }
    Ok(())
}

/// run bin dove
pub fn execute_dove_at(args: &[&str], project_path: &Path) -> Result<String> {
    ensure!(
        project_path.exists(),
        "Project folder {:?} does not exist",
        project_path.display()
    );

    let output = std::process::Command::new(env!("CARGO_BIN_EXE_dove"))
        .current_dir(project_path)
        .args(args)
        .output()?;

    ensure!(
        output.status.success(),
        "Command {:?} failed with code {}. \n Output: \n{}",
        args,
        output.status,
        String::from_utf8(output.stderr).unwrap_or_default()
    );
    Ok(String::from_utf8(output.stdout)?)
}

/// Get the project name from "Move.toml"
pub fn get_project_name_from_toml(project_path: &Path) -> Option<String> {
    let move_toml_path = project_path.join("Move.toml");
    if !move_toml_path.exists() {
        return None;
    }
    let move_toml_content = std::fs::read_to_string(&move_toml_path).ok()?;
    let move_toml = toml::from_str::<toml::Value>(&move_toml_content).ok()?;
    move_toml
        .get("package")
        .and_then(|pack| pack.get("name"))
        .and_then(|name| name.as_str().map(|t| t.to_string()))
}

/// Get dialect name from "Move.toml"
pub fn get_project_dialect_from_toml(project_path: &Path) -> Option<String> {
    let move_toml_path = project_path.join("Move.toml");
    if !move_toml_path.exists() {
        return None;
    }
    let move_toml_content = std::fs::read_to_string(&move_toml_path).ok()?;
    let move_toml = toml::from_str::<toml::Value>(&move_toml_content).ok()?;
    move_toml
        .get("package")
        .and_then(|pack| pack.get("dialect"))
        .and_then(|name| name.as_str().map(|t| t.to_string()))
}

/// Get account address from "Move.toml"
pub fn get_account_address_from_toml(project_path: &Path) -> Option<String> {
    let move_toml_path = project_path.join("Move.toml");
    if !move_toml_path.exists() {
        return None;
    }
    let move_toml_content = std::fs::read_to_string(&move_toml_path).ok()?;
    let move_toml = toml::from_str::<toml::Value>(&move_toml_content).ok()?;
    move_toml
        .get("addresses")
        .and_then(|pack| pack.as_table())
        .and_then(|name| {
            name.get("Account")
                .and_then(|value| value.as_str().map(|value| value.to_string().to_lowercase()))
        })
}

/// check basic folders
pub fn assert_basic_project_dirs_exist(project_dir: &Path) -> Result<()> {
    for path in &[
        project_dir.join("sources"),
        project_dir.join("examples"),
        project_dir.join("scripts"),
        project_dir.join("doc_templates"),
        project_dir.join("tests"),
    ] {
        ensure!(
            path.exists(),
            r#"Directory "{}" does not exist"#,
            path.display()
        );
    }
    Ok(())
}

/// Create a new project in a temporary directory
/// Returns the path to the project
pub fn create_new_project(project_name: &str, addresses: HashMap<&str, &str>) -> Result<PathBuf> {
    let (base_dir, project_dir) = pre_start_dove_new(project_name)?;
    let mut args = vec!["new", project_name];

    let addresses: Vec<String> = addresses
        .iter()
        .map(|(name, address)| format!(r#"{}={}"#, name, address))
        .collect();

    if !addresses.is_empty() {
        args.push("--addresses");
        args.extend(addresses.iter().map(|v| v.as_str()));
    }

    execute_dove_at(&args, &base_dir)?;
    Ok(project_dir)
}

/// Create a test project
pub fn new_demo_project(project_name: &str) -> Result<PathBuf> {
    let addresses = [("Demo", "0x2")].into_iter().collect();
    let project_path = create_new_project(project_name, addresses)?;

    // scripts/main.move
    let mut main_script = fs::File::create(project_path.join("scripts").join("main.move"))?;
    main_script.write_all(b"script { fun main(){} }")?;

    // scripts/one_param.move
    let mut one_param_scripts =
        fs::File::create(project_path.join("scripts").join("one_param.move"))?;
    one_param_scripts.write_all(b"script { fun one_param(a:bool){ assert(a,2); } }")?;

    // scripts/two_params.move
    let mut two_params_scripts =
        fs::File::create(project_path.join("scripts").join("two_params.move"))?;
    two_params_scripts.write_all(b"script { fun two_params(a:u8, b:u8){ assert(a==b,2); } }")?;

    // scripts/with_type.move
    let mut with_type_scripts =
        fs::File::create(project_path.join("scripts").join("with_type.move"))?;
    with_type_scripts.write_all(b"script { fun with_type<T>(_a:u8){ assert(true, 3); } }")?;

    // scripts/multiple_scripts.move
    let mut multiple_scripts =
        fs::File::create(project_path.join("scripts").join("multiple_scripts.move"))?;
    multiple_scripts.write_all(
        b"script { fun script_1(a:bool){ assert(a, 1); } }\n\
            script { fun script_2(a:u8, b:u8){ assert(a==b,2); } }",
    )?;

    // sources/demo1v.move
    let mut demo1v_script = fs::File::create(project_path.join("sources").join("demo1v.move"))?;
    demo1v_script.write_all(b"module Demo::Demo1v{ fun run(){ } }")?;

    // sources/demo2v.move
    let mut demo2v_script = fs::File::create(project_path.join("sources").join("demo2v.move"))?;
    demo2v_script.write_all(b"module Demo::Demo2v{ fun run(){ } }")?;

    // sources/demo3v.move
    let mut demo3v_script = fs::File::create(project_path.join("sources").join("demo3v.move"))?;
    demo3v_script.write_all(b"module Demo::Demo3v{ fun run(){ } }")?;

    // tests/test1.move
    let mut test_1 = fs::File::create(project_path.join("tests").join("test1.move"))?;
    test_1.write_all(
        b"#[test_only]\n\
        module Demo::Test1{\n\
            #[test]\n\
            fun success(){ assert(true,1); }\n\
        }",
    )?;

    // tests/test2.move
    let mut test_2 = fs::File::create(project_path.join("tests").join("test2.move"))?;
    test_2.write_all(
        b"#[test_only]\n\
        module Demo::Test2{\n\
            #[test]\n\
            fun success(){ assert(true,2); }\n\
        }",
    )?;

    // tests/test3.move
    let mut test_3 = fs::File::create(project_path.join("tests").join("test3.move"))?;
    test_3.write_all(
        b"#[test_only]\n\
        module Demo::Test3{\n\
            #[test]\n\
            fun error(){ assert(false,3); }\n\
        }",
    )?;

    Ok(project_path)
}

/// Build a project
pub fn build(project_dir: &Path) -> Result<String> {
    execute_dove_at(&["build"], project_dir)
}
