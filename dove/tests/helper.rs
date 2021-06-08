#![allow(dead_code)]

use std::path::{PathBuf, Path};
use std::fs::{remove_dir_all, read_to_string, create_dir_all};
use fs_extra::file::write_all;
use toml::Value;
use dove::cli::execute;

pub fn project_start(project_name: &str) -> (PathBuf, PathBuf) {
    let tmp_folder = std::env::temp_dir();
    let project_folder = tmp_folder.join(project_name);
    if project_folder.exists() {
        remove_dir_all(&project_folder).unwrap();
    }
    (tmp_folder, project_folder)
}

pub fn project_start_for_init(project_name: &str) -> PathBuf {
    let (_, project_folder) = project_start(project_name);
    // Create project directory
    create_dir_all(&project_folder).unwrap();
    project_folder
}

pub fn project_start_new_and_build(project_name: &str) -> PathBuf {
    let (base_folder, project_folder) = project_start(project_name);
    project_new_default(&base_folder, &project_folder, project_name);
    project_build(&project_folder);
    project_folder
}

/// $ dove new ###
pub fn project_new_default(base_folder: &Path, project_folder: &Path, project_name: &str) {
    execute_dove_at(&["dove", "new", project_name], &base_folder).unwrap();
    set_dependencies_local_move_stdlib(&project_folder);
}

/// $ dove new ### -d ### -a ### -r ###
pub fn project_new_with_args(
    base_folder: &Path,
    project_folder: &Path,
    project_name: &str,
    project_dialect: &str,
    project_address: &str,
    project_blockchain_api: &str,
) {
    execute_dove_at(
        &[
            "dove",
            "new",
            project_name,
            "-d",
            project_dialect,
            "-r",
            project_blockchain_api,
            "-a",
            project_address,
        ],
        &base_folder,
    )
    .unwrap();
    set_dependencies_local_move_stdlib(&project_folder);
}

// $ dove build
pub fn project_build(project_folder: &Path) {
    execute_dove_at(&["dove", "build"], &project_folder).unwrap()
}

pub fn project_remove(project_folder: &Path) {
    if project_folder.exists() {
        remove_dir_all(project_folder).unwrap();
    }
}

pub fn set_dependencies_local_move_stdlib(project_path: &Path) {
    let move_stdlib = Path::new(".")
        .canonicalize()
        .unwrap()
        .join("resources")
        .join("test_move_project");
    let mut dove_toml_path = project_path.to_path_buf();
    dove_toml_path.push("Dove.toml");
    let mut toml_value = read_to_string(&dove_toml_path)
        .unwrap()
        .parse::<Value>()
        .unwrap();
    let mut dd = toml::map::Map::new();
    dd.insert(
        "path".to_string(),
        Value::String(move_stdlib.to_str().unwrap().to_string()),
    );
    toml_value
        .get_mut("package")
        .unwrap()
        .as_table_mut()
        .unwrap()
        .insert(
            "dependencies".to_string(),
            Value::Array(vec![Value::Table(dd)]),
        );
    write_all(
        &dove_toml_path,
        toml::to_string(&toml_value).unwrap().as_str(),
    )
    .unwrap();
}

pub fn execute_dove_at(args: &[&str], project_folder: &Path) -> anyhow::Result<()> {
    execute(args, project_folder.to_path_buf())
}
pub fn execute_dove_bin_at(args: &[&str], project_folder: &Path) -> anyhow::Result<String> {
    assert!(project_folder.exists(), "project_folder does not exist");
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_dove"))
        .current_dir(project_folder)
        .args(args.iter().skip(1)) // TODO: remove first argument `dove` on callee side
        .output()?;
    anyhow::ensure!(
        output.status.success(),
        "Command {:?} failed with code {}",
        args,
        output.status
    );
    Ok(String::from_utf8(output.stdout)?)
}

pub fn assert_valid_dove_toml(
    project_folder: &Path,
    project_name: &str,
    dialect: Option<&str>,
    address: Option<&str>,
    api: Option<&str>,
) {
    // Check config
    let dove_toml_string = read_to_string(project_folder.join("Dove.toml"))
        .unwrap()
        .replace(" ", "")
        .replace("\t", "");

    assert!(
        dove_toml_string.contains(&format!("name=\"{}\"", project_name)),
        "Missing name = {}",
        project_name
    );
    if let Some(dialect) = dialect {
        assert!(
            dove_toml_string.contains(&format!("dialect=\"{}\"", dialect)),
            "Missing dialect = {}",
            dialect
        );
    }
    if let Some(address) = address {
        assert!(
            dove_toml_string.contains(&format!("account_address=\"{}\"", address)),
            "Missing account_address = {}",
            address
        );
    }
    if let Some(api) = api {
        assert!(
            dove_toml_string.contains(&format!("blockchain_api=\"{}\"", api)),
            "Missing blockchain_api = {}",
            api
        );
    }
}

pub fn assert_basic_project_dirs_exist(project_folder: &Path) {
    assert!(
        project_folder.join("modules").exists(),
        "Folder modules not found"
    );
    assert!(
        project_folder.join("scripts").exists(),
        "Folder scripts not found"
    );
    assert!(
        project_folder.join("tests").exists(),
        "Folder tests not found"
    );
}
