#![cfg(test)]

use std::path::{Path, PathBuf};
use std::fs::{remove_dir_all, read_to_string};
use fs_extra::file::write_all;
use dove::cli::execute;
use toml::Value;

/// $ dove new demoproject_32 -d dfinance
/// $ dove build
/// project: demoproject_32
#[test]
fn default() {
    // Path to dove folder
    let dove_folder = {
        let mut folder = Path::new(".").canonicalize().unwrap();
        if folder.to_str().unwrap().find("dove").is_none() {
            folder.push("dove");
        }
        folder
    };
    // Project name and path
    let project_name = "demoproject_32";
    let project_folder = {
        let mut folder = dove_folder.clone();
        folder.push(project_name);
        if folder.exists() {
            remove_dir_all(&folder).expect(&format!(
                "[ERROR] Couldn't delete project directory: {}",
                folder.to_str().unwrap()
            ));
        }
        folder
    };

    // $ dove new demoproject_32 -d dfinance
    {
        let args = &["dove", "new", &project_name, "-d", "dfinance"];
        let command_string: String = args.join(" ").to_string();
        execute(args, dove_folder.clone()).expect(&format!("[COMMAND] {}", &command_string));
        set_dependencies_local_move_stdlib(&project_folder);
    }

    // Check config
    {
        let mut path_toml = project_folder.clone();
        path_toml.push("Dove.toml");

        let package = read_to_string(path_toml)
            .unwrap()
            .parse::<Value>()
            .unwrap()
            .get("package")
            .unwrap()
            .clone();

        assert!(
            package
                .get("name")
                .expect(&format!("[ERROR] Dove.toml - name not found "))
                .to_string()
                .contains(project_name),
            "Dove.toml: invalid name",
        );

        assert!(
            package
                .get("dialect")
                .expect(&format!("[ERROR] Dove.toml - dialect not found "))
                .to_string()
                .contains("dfinance"),
            "Dove.toml: invalid dialect",
        );
    }
    // $ dove build
    {
        let args = &["dove", "build"];
        let command_string: String = args.join(" ").to_string();
        execute(args, project_folder.clone()).expect(&format!("[COMMAND] {}", &command_string));
    }

    remove_dir_all(&project_folder).expect(&format!(
        "[ERROR] Couldn't delete project directory: {}",
        project_folder.to_str().unwrap()
    ));
}

/// $ dove new demoproject_33 -d dfinance -a ###
/// $ dove build
/// project: demoproject_33
#[test]
fn with_address() {
    // Path to dove folder
    let dove_folder = {
        let mut folder = Path::new(".").canonicalize().unwrap();
        if folder.to_str().unwrap().find("dove").is_none() {
            folder.push("dove");
        }
        folder
    };
    // Project name and path
    let project_name = "demoproject_33";
    let project_folder = {
        let mut folder = dove_folder.clone();
        folder.push(project_name);
        if folder.exists() {
            remove_dir_all(&folder).expect(&format!(
                "[ERROR] Couldn't delete project directory: {}",
                folder.to_str().unwrap()
            ));
        }
        folder
    };

    for address in &["0x1", "wallet1me0cdn52672y7feddy7tgcj6j4dkzq2su745vh"] {
        // $ dove new demoproject_33 -d dfinance -a ###
        {
            let args = &[
                "dove",
                "new",
                &project_name,
                "-d",
                "dfinance",
                "-a",
                address,
            ];
            let command_string: String = args.join(" ").to_string();
            execute(args, dove_folder.clone()).expect(&format!(
                "[COMMAND] {}\r\n[ADDRESS] {}",
                &command_string, address
            ));
            set_dependencies_local_move_stdlib(&project_folder);
        }
        // Check config
        {
            let mut path_toml = project_folder.clone();
            path_toml.push("Dove.toml");

            let package = read_to_string(path_toml)
                .unwrap()
                .parse::<Value>()
                .unwrap()
                .get("package")
                .unwrap()
                .clone();
            assert!(
                package
                    .get("name")
                    .expect(&format!("[ERROR] Dove.toml - name not found "))
                    .to_string()
                    .contains(project_name),
                "Dove.toml: invalid name",
            );
            assert!(
                package
                    .get("dialect")
                    .expect(&format!("[ERROR] Dove.toml - dialect not found "))
                    .to_string()
                    .contains("dfinance"),
                "Dove.toml: invalid dialect",
            );
            assert!(
                package
                    .get("account_address")
                    .expect(&format!("[ERROR] Dove.toml - account_address not found "))
                    .to_string()
                    .contains(address),
                "Dove.toml: invalid account_address",
            );
        }
        // $ dove build
        {
            let args = &["dove", "build"];
            let command_string: String = args.join(" ").to_string();
            execute(args, project_folder.clone())
                .expect(&format!("[COMMAND] {}", &command_string));
        }

        remove_dir_all(&project_folder).expect(&format!(
            "[ERROR] Couldn't delete project directory: {}",
            project_folder.to_str().unwrap()
        ));
    }
}

fn set_dependencies_local_move_stdlib(project_path: &PathBuf) {
    let mut dove_toml_path = project_path.clone();
    dove_toml_path.push("Dove.toml");
    let mut toml_value = read_to_string(&dove_toml_path)
        .unwrap()
        .parse::<Value>()
        .unwrap();
    {
        let v = toml_value
            .get_mut("package")
            .unwrap()
            .get_mut("dependencies")
            .unwrap()
            .as_array_mut()
            .unwrap();
        v.clear();
        let mut dd = toml::map::Map::new();
        dd.insert(
            "path".to_string(),
            Value::String("../tests/move-stdlib".to_string()),
        );
        v.push(Value::Table(dd));
    }
    write_all(
        &dove_toml_path,
        toml::to_string(&toml_value).unwrap().as_str(),
    )
    .unwrap();
}
