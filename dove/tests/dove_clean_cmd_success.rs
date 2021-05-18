/// $ cargo run -- clean
/// $ dove clean
#[cfg(test)]
mod dove_clean_cmd_success {
    use std::path::{Path, PathBuf};
    use std::process::{Command};
    use std::fs::{remove_dir_all, read_to_string};
    use fs_extra::file::write_all;

    /// project name: demoproject_16
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
        let project_name = "demoproject_16";

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
        // $ cargo run -- new demoproject_16 -d pont
        // $ dove new demoproject_16 -d pont
        {
            let mut dove_new = Command::new("cargo");
            dove_new
                .args(&["run", "--", "new", project_name])
                .args(&["-d", "pont"])
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
                "[ERROR] Command: {}\r\nCode: {}\r\nMessage: {}\r\n",
                command_string,
                code,
                String::from_utf8(result.stderr).unwrap()
            );

            // @todo remove later
            add_in_dove_toml_branch(&project_folder);
        }

        // $ cargo run -- build
        // $ dove build
        {
            let mut dove_build = Command::new("cargo");
            dove_build
                .args(&["run", "--", "build"])
                .current_dir(&project_folder);
            let command_string = format!("{:?} ", dove_build).replace("\"", "");
            let result = dove_build.output();
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
                "[ERROR] Command: {}\r\nCode: {}\r\nMessage: {}\r\n",
                command_string,
                code,
                String::from_utf8(result.stderr).unwrap()
            );
        }

        // $ cargo run -- clean
        // $ dove clean
        {
            let mut dove_build = Command::new("cargo");
            dove_build
                .args(&["run", "--", "clean"])
                .current_dir(&project_folder);
            let command_string = format!("{:?} ", dove_build).replace("\"", "");
            let result = dove_build.output();
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
                "[ERROR] Command: {}\r\nCode: {}\r\nMessage: {}\r\n",
                command_string,
                code,
                String::from_utf8(result.stderr).unwrap()
            );

            let mut project_target = project_folder.clone();
            project_target.push("target");

            assert!(
                !project_target.exists(),
                "Directory was not deleted: {}",
                project_target.to_str().unwrap_or(" - ")
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
