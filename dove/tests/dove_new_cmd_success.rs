/// Create a new move project
/// Correct url: http://demo.ru/api, https://demo.ru/api, http://127.0.0.1/api, http://localhost/api, http://localhost:8080/api
///
/// $ cargo run -- new ### [-d ###] [-a ###] [-r ###]
/// $ dove new ### [-d ###] [-a ###] [-r ###]
#[cfg(test)]
mod dove_new_cmd_success {
    use std::path::{Path, PathBuf};
    use std::process::{Command};
    use std::fs::{remove_dir_all, read_to_string};
    use fs_extra::file::write_all;

    /// project name: demoproject_13
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
        let project_name = "demoproject_13";
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
        // $ dove new demoproject_13 [-d ###] [-a ###] [-r ###]
        // $ dove build
        for (dialect, address, blockchain_api) in vec![
            (None, None, None),
            (Some("pont"), None, None),
            (
                Some("pont"),
                Some("5CdCiQzNRZXWx7wNVCVjPMzGBFpkYHe3WKrGzd6TG97vKbnv"),
                None,
            ),
            (Some("pont"), Some("0x1"), None),
            (Some("pont"), Some("0x1"), Some("http://demo.ru/api")),
            (Some("pont"), Some("0x1"), Some("https://demo.ru/api")),
            (Some("pont"), Some("0x1"), Some("http://127.0.0.1/api")),
            (Some("pont"), Some("0x1"), Some("http://localhost/api")),
            (Some("pont"), Some("0x1"), Some("http://localhost:8080/api")),
            (Some("dfinance"), None, None),
            (
                Some("dfinance"),
                Some("wallet1me0cdn52672y7feddy7tgcj6j4dkzq2su745vh"),
                None,
            ),
            (Some("dfinance"), Some("0x1"), None),
            (Some("dfinance"), Some("0x1"), Some("http://demo.ru/api")),
            (Some("dfinance"), Some("0x1"), Some("https://demo.ru/api")),
            (Some("dfinance"), Some("0x1"), Some("http://127.0.0.1/api")),
            (Some("dfinance"), Some("0x1"), Some("http://localhost/api")),
            (
                Some("dfinance"),
                Some("0x1"),
                Some("http://localhost:8080/api"),
            ),
            (Some("diem"), None, None),
            (Some("diem"), Some("0x1"), None),
            (Some("diem"), Some("0x1"), Some("http://demo.ru/api")),
            (Some("diem"), Some("0x1"), Some("https://demo.ru/api")),
            (Some("diem"), Some("0x1"), Some("http://127.0.0.1/api")),
            (Some("diem"), Some("0x1"), Some("http://localhost/api")),
            (Some("diem"), Some("0x1"), Some("http://localhost:8080/api")),
        ] {
            // $ cargo run -- new demoproject_13 [-d ###] [-a ###] [-r ###]
            // $ dove new demoproject_13 [-d ###] [-a ###] [-r ###]
            {
                let mut dove_new = Command::new("cargo");
                dove_new
                    .args(&["run", "--", "new", project_name])
                    .current_dir(&dove_folder);
                if let Some(dialect) = dialect.as_ref() {
                    dove_new.args(&["-d", dialect]);
                }
                if let Some(address) = address.as_ref() {
                    dove_new.args(&["-a", address]);
                }
                if let Some(api) = blockchain_api.as_ref() {
                    dove_new.args(&["-r", api]);
                }

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
                    "[Command]: {}\r\n[Settings]: {:?} {:?} {:?}\r\n[Code]: {}\r\n[Message]: {}",
                    command_string,
                    dialect,
                    address,
                    blockchain_api,
                    code,
                    String::from_utf8(result.stderr).unwrap(),
                );
            }

            assert!(
                remove_dir_all(&project_folder).is_ok(),
                "[ERROR] Couldn't delete directory {}",
                project_folder.to_str().unwrap()
            );
        }
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
