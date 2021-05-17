/// $ cargo run -- run rdemo.move [-a ### ### ..]
/// $ dove run
#[cfg(test)]
mod dove_run_cmd_success {
    use std::path::{Path, PathBuf};
    use std::process::{Command};
    use std::fs::{remove_dir_all, read_to_string};
    use fs_extra::file::write_all;

    /// project name: demoproject_6
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
        let project_name = "demoproject_6";
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
        // $ cargo run -- new demoproject_6 -d pont
        // $ dove new demoproject_6 -d pont
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
                "[ERROR] Command: {}; Code: {}; Message: {};",
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
                "[ERROR] Command: {}; Code: {}; Message: {};",
                command_string,
                code,
                String::from_utf8(result.stderr).unwrap()
            );
        }

        // $ cargo run -- run rdemo.move
        // $ dove run rdemo.move
        {
            // project_folder/modules/demo_module.move
            let module_path = {
                let mut path = project_folder.clone();
                path.push("modules");
                path.push("mdemo.move");
                path
            };
            write_all(
                &module_path,
                "address 0x1 {
                    module DemoModule {
                        public fun value(): u8 {
                            12
                        }
                    }
                }",
            )
            .unwrap();
            // project_folder/scripts/demo.move
            let script_path = {
                let mut path = project_folder.clone();
                path.push("scripts");
                path.push("rdemo.move");
                path
            };
            write_all(
                &script_path,
                "script {
                    use 0x1::DemoModule;
                    use 0x1::Debug;

                    fun main() {
                        let value = DemoModule::value();

                        Debug::print<u8>(&value);
                    }
                }",
            )
            .unwrap();

            let mut dove_run = Command::new("cargo");
            dove_run
                .args(&["run", "--", "run", "rdemo.move"])
                .current_dir(&project_folder);
            let command_string = format!("{:?} ", dove_run).replace("\"", "");
            let result = dove_run.output();
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
                stdout.contains("main ...... ok"),
                "[ERROR] Command: {}. \r\nMessage: {}",
                command_string,
                stdout
            );
        }
        assert!(
            remove_dir_all(&project_folder).is_ok(),
            "[ERROR] Couldn't delete directory {}",
            project_folder.to_str().unwrap()
        );
    }
    /// project name: demoproject_8
    #[test]
    fn success_with_argv() {
        // Path to dove folder
        let dove_folder = {
            let mut folder = Path::new(".").canonicalize().unwrap();
            if folder.to_str().unwrap().find("dove").is_none() {
                folder.push("dove");
            }
            folder
        };
        // Project name and path
        let project_name = "demoproject_8";
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
        // $ cargo run -- new demoproject_6 -d pont
        // $ dove new demoproject_6 -d pont
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
                "[ERROR] Command: {}; Code: {}; Message: {};",
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
                "[ERROR] Command: {}; Code: {}; Message: {};",
                command_string,
                code,
                String::from_utf8(result.stderr).unwrap()
            );
        }

        // $ cargo run -- run rdemo.move -a 3 5
        // $ dove run rdemo.move
        {
            // project_folder/scripts/demo.move
            let script_path = {
                let mut path = project_folder.clone();
                path.push("scripts");
                path.push("rdemo.move");
                path
            };
            write_all(
                &script_path,
                "script {
                    use 0x1::Debug;

                    fun main(x:u64,y:u64) {
                        let result = x + y;
                        Debug::print<u64>(&result);
                    }
                }",
            )
            .unwrap();

            let mut dove_run = Command::new("cargo");
            dove_run
                .args(&["run", "--"])
                .args(&["run", "rdemo.move"])
                .args(&["-a", "3", "5"])
                .current_dir(&project_folder);
            let command_string = format!("{:?} ", dove_run).replace("\"", "");
            let result = dove_run.output();
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
                String::from_utf8(result.stderr.clone()).unwrap()
            );
            let stdout = String::from_utf8(result.stdout).unwrap();
            assert!(
                stdout.contains("main ...... ok"),
                "[ERROR] Command: {}. \r\nMessage: {}",
                command_string,
                stdout
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
