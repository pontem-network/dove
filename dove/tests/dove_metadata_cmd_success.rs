/// $ cargo run -- metadata
/// $ dove metadata
#[cfg(test)]
mod dove_metadata_cmd_success {
    use std::path::{Path, PathBuf};
    use std::process::{Command};
    use std::fs::{remove_dir_all, read_to_string};
    use fs_extra::file::write_all;

    /// project name: demoproject_15
    #[test]
    fn success() {
        // Path to dove folder
        let dove_folder = {
            let mut folder = Path::new(".").canonicalize().unwrap();
            if folder.to_str().unwrap().find("dove").is_none() {
                folder.push("dove");
            }
            folder
        };
        // Project name and path
        let project_name = "demoproject_15";
        let project_address = "5Csxuy81dNEVYbRA9K7tyHypu7PivHmwCZSKxcbU78Cy2v7v";
        let blockchain_api = "https://localhost/api";
        let project_dialect = "pont";

        let project_folder = {
            let mut folder = dove_folder.clone();
            folder.push(project_name);
            folder
        };
        if project_folder.exists() {
            assert!(
                remove_dir_all(&project_folder).is_ok(),
                "[ERROR] Couldn't delete project directory. Folder: {}",
                project_folder.to_str().unwrap()
            );
        }
        // $ cargo run -- new demoproject_15 -d pont -a 5Csxuy81dNEVYbRA9K7tyHypu7PivHmwCZSKxcbU78Cy2v7v -r https://localhost/api
        // $ dove new demoproject_15 -d pont
        {
            let mut dove_new = Command::new("cargo");
            dove_new
                .args(&["run", "--", "new", project_name])
                .args(&["-d", project_dialect])
                .args(&["-r", blockchain_api])
                .args(&["-a", project_address])
                .current_dir(&dove_folder);
            let command_string = format!("{:?} ", dove_new).replace("\"", "");
            let result = dove_new.output();
            assert!(
                result.is_ok(),
                "[ERROR]: {}\r\n[RUN]: {}",
                result.err().unwrap(),
                command_string
            );
            let result = result.unwrap();
            let code = result.status.code().unwrap();
            assert_eq!(
                0,
                code,
                "[ERROR] Command: {}; Code: {}; Message: {};",
                command_string,
                code,
                String::from_utf8(result.stderr).unwrap()
            );

            // @todo remove later
            add_in_dove_toml_branch(&project_folder);
        }

        // $ cargo run -- metadata
        // $ dove metadata
        {
            let mut dove_metadata = Command::new("cargo");
            dove_metadata
                .args(&["run", "--", "metadata"])
                .current_dir(&project_folder);
            let command_string = format!("{:?} ", dove_metadata).replace("\"", "");
            let result = dove_metadata.output();
            assert!(
                result.is_ok(),
                "[ERROR]: {}\r\n[RUN]: {}",
                result.err().unwrap(),
                command_string
            );

            let result = result.unwrap();
            let code = result.status.code().unwrap();
            assert_eq!(
                0,
                code,
                "[ERROR] Command: {}; Code: {}; Message: {};",
                command_string,
                code,
                String::from_utf8(result.stderr).unwrap()
            );

            let stdout = String::from_utf8(result.stdout).unwrap();
            assert!(
                stdout.contains(&project_name),
                "Not found in metadata name: {}",
                &project_name
            );
            assert!(
                stdout.contains(project_address),
                "Not found in metadata account_address: {}",
                project_address
            );
            assert!(
                stdout.contains(project_dialect),
                "Not found in metadata dialect: {}",
                project_dialect
            );
            assert!(
                stdout.contains(blockchain_api),
                "Not found in metadata blockchain_api: {}",
                blockchain_api
            );
        }

        assert!(
            remove_dir_all(&project_folder).is_ok(),
            "[ERROR] Couldn't delete directory {}",
            project_folder.to_str().unwrap()
        );
    }

    // @todo remove later
    fn add_in_dove_toml_branch(project_path: &PathBuf) {
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
                .get_mut(0)
                .unwrap()
                .as_table_mut()
                .unwrap();
            v.insert("branch".to_string(), Value::String("move-1.2".to_string()));
        }
        write_all(
            &dove_toml_path,
            toml::to_string(&toml_value).unwrap().as_str(),
        )
        .unwrap();
    }
}
