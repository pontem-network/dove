#![cfg(test)]

use std::path::{Path, PathBuf};
use std::fs::{remove_dir_all, read_to_string, remove_file};
use fs_extra::file::write_all;
use dove::cmd::ct::Transaction;
use dove::cli::execute;

/// $ dove ct
/// project name: demoproject_19
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
    let project_name = "demoproject_19";
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

    // $ cargo run -- new demoproject_19
    // $ dove new demoproject_19
    {
        let args = &["dove", "new", project_name];
        let command_string: String = args.join(" ").to_string();
        execute(args, dove_folder.clone()).expect(&format!("[COMMAND] {}", &command_string));
        set_dependencies_local_move_stdlib(&project_folder);
    }

    // $ cargo run -- build
    // $ dove build
    {
        let args = &["dove", "build"];
        let command_string: String = args.join(" ").to_string();
        execute(args, project_folder.clone()).expect(&format!(
            "[COMMAND] {}\r\n[FOLDER] {}",
            &command_string,
            project_folder.to_str().unwrap()
        ));
    }

    // project_folder/scripts/sdemo.move
    {
        let mut path = project_folder.clone();
        path.push("scripts");
        path.push("sdemo.move");
        write_all(
            &path,
            "script {
                        fun main() {
                            assert((1+3)==4,1);
                        }
                    }",
        )
        .unwrap();
    }
    // $ cargo run -- ct
    // $ dove ct
    {
        let args = &["dove", "ct"];
        let command_string: String = args.join(" ").to_string();
        execute(args, project_folder.clone()).expect(&format!(
            "[COMMAND] {}\r\n[FOLDER] {}",
            &command_string,
            project_folder.to_str().unwrap()
        ));

        let tx_path = {
            let mut file = project_folder.clone();
            file.push("target");
            file.push("transactions");
            file.push("main.mvt");
            file
        };

        assert!(
            tx_path.exists(),
            "Transaction not found: {}\r\n[Command] {}",
            tx_path.to_str().unwrap(),
            &command_string,
        );
        let tx_fmt = format!(
            "{:?}",
            bcs::from_bytes::<Transaction>(std::fs::read(&tx_path).unwrap().as_ref()).unwrap()
        );
        assert!(tx_fmt.contains(" args: []"));
        assert!(tx_fmt.contains(" type_args: []"));
        assert!(tx_fmt.contains(" signers_count: 0"));

        remove_file(&tx_path).unwrap();
    }

    remove_dir_all(&project_folder).expect(&format!(
        "[ERROR] Couldn't delete directory {}",
        project_folder.to_str().unwrap()
    ));
}

fn set_dependencies_local_move_stdlib(project_path: &PathBuf) {
    use toml::Value;

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
