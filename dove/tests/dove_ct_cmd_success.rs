/// $ cargo run -- ct [###] [-f ###] [-n ###] [-a ### ### ..] [-o ###]
/// $ dove ct [###] [-f ###] [-n ###] [-a ### ### ..] [-o ###]
/// @todo Add tests for $ dove ct -t ###, after bug fix
#[cfg(test)]
mod dove_ct_cmd_success {
    use std::path::{Path, PathBuf};
    use std::process::{Command};
    use std::fs::{remove_dir_all, read_to_string, remove_file};
    use fs_extra::file::write_all;
    use dove::cmd::ct::Transaction;

    /// project name: demoproject_18
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
        let project_name = "demoproject_18";
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
        // $ cargo run -- new demoproject_18
        // $ dove new demoproject_18
        {
            let mut dove_new = Command::new("cargo");
            dove_new
                .args(&["run", "--", "new", project_name])
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

        // $ cargo run -- ct
        // $ dove ct
        {
            // project_folder/scripts/sdemo_1.move
            {
                let mut path = project_folder.clone();
                path.push("scripts");
                path.push("sdemo_1.move");
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
            let mut dove_ct = Command::new("cargo");
            dove_ct
                .args(&["run", "--", "ct"])
                .current_dir(&project_folder);
            let command_string = format!("{:?} ", dove_ct).replace("\"", "");
            let result = dove_ct.output();
            assert!(
                result.is_ok(),
                "[ERROR]: {}\r\n[RUN]: {}\r\n",
                result.err().unwrap(),
                command_string,
            );

            let result = result.unwrap();
            let code = result.status.code().unwrap();
            assert_eq!(
                0,
                code,
                "[ERROR] Command: {}\r\nCode: {}\r\nError: {}\r\nOut: {}",
                command_string,
                code,
                String::from_utf8(result.stderr.clone()).unwrap(),
                String::from_utf8(result.stdout.clone()).unwrap(),
            );

            let mut tx_path = project_folder.clone();
            tx_path.push("target");
            tx_path.push("transactions");
            tx_path.push("main.mvt");

            assert!(
                tx_path.exists(),
                "Transaction not found: {}\r\n[Command] {}",
                tx_path.to_str().unwrap(),
                &command_string,
            );
            let tx = bcs::from_bytes::<Transaction>(std::fs::read(&tx_path).unwrap().as_ref())
                .unwrap();
            let tx_fmt = format!("{:?}", tx);

            assert!(tx_fmt.contains(" args: []"));
            assert!(tx_fmt.contains(" type_args: []"));
            assert!(tx_fmt.contains(" signers_count: 0"));

            remove_file(&tx_path).unwrap();
        }

        // $ cargo run -- ct -f sdemo_2
        // $ dove ct -f sdemo_2
        {
            // project_folder/scripts/sdemo_2.move
            {
                let mut path = project_folder.clone();
                path.push("scripts");
                path.push("sdemo_2.move");
                write_all(
                    &path,
                    "script {
                        fun main() {
                            assert((2+2)==4,1);
                        }
                    }",
                )
                .unwrap();
            }
            let mut dove_ct = Command::new("cargo");
            dove_ct
                .args(&["run", "--", "ct"])
                .args(&["-f", "sdemo_2"])
                .current_dir(&project_folder);
            let command_string = format!("{:?} ", dove_ct).replace("\"", "");
            let result = dove_ct.output();
            assert!(
                result.is_ok(),
                "[ERROR]: {}\r\n[RUN]: {}\r\n",
                result.err().unwrap(),
                command_string,
            );

            let result = result.unwrap();
            let code = result.status.code().unwrap();
            assert_eq!(
                0,
                code,
                "[ERROR] Command: {}\r\nCode: {}\r\nError: {}\r\nOut: {}",
                command_string,
                code,
                String::from_utf8(result.stderr.clone()).unwrap(),
                String::from_utf8(result.stdout.clone()).unwrap(),
            );

            let mut tx_path = project_folder.clone();
            tx_path.push("target");
            tx_path.push("transactions");
            tx_path.push("main.mvt");

            assert!(
                tx_path.exists(),
                "Transaction not found: {}\r\n[Command] {}",
                tx_path.to_str().unwrap(),
                &command_string,
            );

            let tx = bcs::from_bytes::<Transaction>(std::fs::read(&tx_path).unwrap().as_ref())
                .unwrap();
            let tx_fmt = format!("{:?}", tx);

            assert!(tx_fmt.contains(" args: []"));
            assert!(tx_fmt.contains(" type_args: []"));
            assert!(tx_fmt.contains(" signers_count: 0"));

            remove_file(&tx_path).unwrap();
        }

        // $ cargo run -- ct -f sdemo_2 -o demo_2
        // $ dove ct -f sdemo_2 -o demo_2
        {
            let mut dove_ct = Command::new("cargo");
            dove_ct
                .args(&["run", "--", "ct"])
                .args(&["-f", "sdemo_2"])
                .args(&["-o", "demo_2"])
                .current_dir(&project_folder);
            let command_string = format!("{:?} ", dove_ct).replace("\"", "");
            let result = dove_ct.output();
            assert!(
                result.is_ok(),
                "[ERROR]: {}\r\n[RUN]: {}\r\n",
                result.err().unwrap(),
                command_string,
            );

            let result = result.unwrap();
            let code = result.status.code().unwrap();
            assert_eq!(
                0,
                code,
                "[ERROR] Command: {}\r\nCode: {}\r\nError: {}\r\nOut: {}",
                command_string,
                code,
                String::from_utf8(result.stderr.clone()).unwrap(),
                String::from_utf8(result.stdout.clone()).unwrap(),
            );

            let mut tx_path = project_folder.clone();
            tx_path.push("target");
            tx_path.push("transactions");
            tx_path.push("demo_2.mvt");

            assert!(
                tx_path.exists(),
                "Transaction not found: {}\r\n[Command] {}",
                tx_path.to_str().unwrap(),
                &command_string,
            );

            let tx = bcs::from_bytes::<Transaction>(std::fs::read(&tx_path).unwrap().as_ref())
                .unwrap();
            let tx_fmt = format!("{:?}", tx);

            assert!(tx_fmt.contains(" args: []"));
            assert!(tx_fmt.contains(" type_args: []"));
            assert!(tx_fmt.contains(" signers_count: 0"));

            remove_file(&tx_path).unwrap();
        }

        // $ cargo run -- ct -f sdemo_3 -o demo_3 -a 1 2
        // $ dove ct -f sdemo_2 -o demo_3 -a 1 2
        {
            // project_folder/scripts/sdemo_3.move
            {
                let mut path = project_folder.clone();
                path.push("scripts");
                path.push("sdemo_3.move");
                write_all(
                    &path,
                    "script {
                        fun main(_a1:u64,_a2:u64) { }
                    }",
                )
                    .unwrap();
            }
            let mut dove_ct = Command::new("cargo");
            dove_ct
                .args(&["run", "--", "ct"])
                .args(&["-f", "sdemo_3"])
                .args(&["-o", "demo_3"])
                .args(&["-a", "1", "2"])
                .current_dir(&project_folder);
            let command_string = format!("{:?} ", dove_ct).replace("\"", "");
            let result = dove_ct.output();
            assert!(
                result.is_ok(),
                "[ERROR]: {}\r\n[RUN]: {}\r\n",
                result.err().unwrap(),
                command_string,
            );

            let result = result.unwrap();
            let code = result.status.code().unwrap();
            assert_eq!(
                0,
                code,
                "[ERROR] Command: {}\r\nCode: {}\r\nError: {}\r\nOut: {}",
                command_string,
                code,
                String::from_utf8(result.stderr.clone()).unwrap(),
                String::from_utf8(result.stdout.clone()).unwrap(),
            );

            let mut tx_path = project_folder.clone();
            tx_path.push("target");
            tx_path.push("transactions");
            tx_path.push("demo_3.mvt");

            assert!(
                tx_path.exists(),
                "Transaction not found: {}\r\n[Command] {}",
                tx_path.to_str().unwrap(),
                &command_string,
            );

            let tx = bcs::from_bytes::<Transaction>(std::fs::read(&tx_path).unwrap().as_ref())
                .unwrap();
            let tx_fmt = format!("{:?}", tx);

            assert!(tx_fmt.contains(" args: [U64(1), U64(2)]"));
            assert!(tx_fmt.contains(" type_args: []"));
            assert!(tx_fmt.contains(" signers_count: 0"));

            remove_file(&tx_path).unwrap();
        }

        // $ cargo run -- ct -f sdemo_4 -n zero -a 1 2 -o demo_4
        // $ dove ct -f sdemo_4 -n zero -a 1 2 -o demo_4
        {
            // project_folder/scripts/demo_4.move
            {
                let mut path = project_folder.clone();
                path.push("scripts");
                path.push("sdemo_4.move");
                write_all(
                    &path,
                    "script {
                        fun main(a1:u64,a2:u64) { assert(a1==a2,1); }
                    }
                    script {
                        fun zero(a1:u64,a2:u64) { assert(a1==a2,1); }
                    }",
                )
                .unwrap();
            }
            let mut dove_ct = Command::new("cargo");
            dove_ct
                .args(&["run", "--", "ct"])
                .args(&["-f", "sdemo_4"])
                .args(&["-n", "zero"])
                .args(&["-a", "1", "2"])
                .args(&["-o", "demo_4"])
                .current_dir(&project_folder);
            let command_string = format!("{:?} ", dove_ct).replace("\"", "");
            let result = dove_ct.output();
            assert!(
                result.is_ok(),
                "[ERROR]: {}\r\n[RUN]: {}\r\n",
                result.err().unwrap(),
                command_string,
            );

            let result = result.unwrap();
            let code = result.status.code().unwrap();
            assert_eq!(
                0,
                code,
                "[ERROR] Command: {}\r\nCode: {}\r\nError: {}\r\nOut: {}",
                command_string,
                code,
                String::from_utf8(result.stderr.clone()).unwrap(),
                String::from_utf8(result.stdout.clone()).unwrap(),
            );

            let mut tx_path = project_folder.clone();
            tx_path.push("target");
            tx_path.push("transactions");
            tx_path.push("demo_4.mvt");

            assert!(
                tx_path.exists(),
                "Transaction not found: {}\r\n[Command] {}",
                tx_path.to_str().unwrap(),
                &command_string,
            );

            let tx = bcs::from_bytes::<Transaction>(std::fs::read(&tx_path).unwrap().as_ref())
                .unwrap();
            let tx_fmt = format!("{:?}", tx);
            assert!(tx_fmt.contains(" args: [U64(1), U64(2)]"));
            assert!(tx_fmt.contains(" type_args: []"));
            assert!(tx_fmt.contains(" signers_count: 0"));

            remove_file(&tx_path).unwrap();
        }

        // $ cargo run -- ct "zero()" -f sdemo_4 -a 1 2 -o demo_4_1
        // $ dove ct "zero()" -f sdemo_4 -a 1 2 -o demo_4_1
        {
            let mut dove_ct = Command::new("cargo");
            dove_ct
                .args(&["run", "--", "ct", "zero()"])
                .args(&["-f", "sdemo_4"])
                .args(&["-a", "1", "2"])
                .args(&["-o", "demo_4_1"])
                .current_dir(&project_folder);
            let command_string = format!("{:?} ", dove_ct).replace("\"", "");
            let result = dove_ct.output();
            assert!(
                result.is_ok(),
                "[ERROR]: {}\r\n[RUN]: {}\r\n",
                result.err().unwrap(),
                command_string,
            );

            let result = result.unwrap();
            let code = result.status.code().unwrap();
            assert_eq!(
                0,
                code,
                "[ERROR] Command: {}\r\nCode: {}\r\nError: {}\r\nOut: {}",
                command_string,
                code,
                String::from_utf8(result.stderr.clone()).unwrap(),
                String::from_utf8(result.stdout.clone()).unwrap(),
            );

            let mut tx_path = project_folder.clone();
            tx_path.push("target");
            tx_path.push("transactions");
            tx_path.push("demo_4_1.mvt");

            assert!(
                tx_path.exists(),
                "Transaction not found: {}\r\n[Command] {}",
                tx_path.to_str().unwrap(),
                &command_string,
            );

            let tx = bcs::from_bytes::<Transaction>(std::fs::read(&tx_path).unwrap().as_ref())
                .unwrap();
            let tx_fmt = format!("{:?}", tx);
            assert!(tx_fmt.contains(" args: [U64(1), U64(2)]"));
            assert!(tx_fmt.contains(" type_args: []"));
            assert!(tx_fmt.contains(" signers_count: 0"));

            remove_file(&tx_path).unwrap();
        }

        // $ cargo run -- ct "zero(1,2)" -f sdemo_4 -o demo_4_2
        // $ dove ct "zero(1,2)" -f sdemo_4 -o demo_4_2
        {
            let mut dove_ct = Command::new("cargo");
            dove_ct
                .args(&["run", "--", "ct", "zero(1,2)"])
                .args(&["-f", "sdemo_4"])
                .args(&["-o", "demo_4_2"])
                .current_dir(&project_folder);
            let command_string = format!("{:?} ", dove_ct).replace("\"", "");
            let result = dove_ct.output();
            assert!(
                result.is_ok(),
                "[ERROR]: {}\r\n[RUN]: {}\r\n",
                result.err().unwrap(),
                command_string,
            );

            let result = result.unwrap();
            let code = result.status.code().unwrap();
            assert_eq!(
                0,
                code,
                "[ERROR] Command: {}\r\nCode: {}\r\nError: {}\r\nOut: {}",
                command_string,
                code,
                String::from_utf8(result.stderr.clone()).unwrap(),
                String::from_utf8(result.stdout.clone()).unwrap(),
            );

            let mut tx_path = project_folder.clone();
            tx_path.push("target");
            tx_path.push("transactions");
            tx_path.push("demo_4_2.mvt");

            assert!(
                tx_path.exists(),
                "Transaction not found: {}\r\n[Command] {}",
                tx_path.to_str().unwrap(),
                &command_string,
            );

            let tx = bcs::from_bytes::<Transaction>(std::fs::read(&tx_path).unwrap().as_ref())
                .unwrap();
            let tx_fmt = format!("{:?}", tx);
            assert!(tx_fmt.contains(" args: [U64(1), U64(2)]"));
            assert!(tx_fmt.contains(" type_args: []"));
            assert!(tx_fmt.contains(" signers_count: 0"));

            remove_file(&tx_path).unwrap();
        }

        // $ cargo run -- ct "test_fun<u8>(1)" -f sdemo_5 -o demo_5
        // $ dove ct "test_fun<u8>(1)" -f sdemo_5 -o demo_5
        {
            // project_folder/modules/mdedmo_2.move
            {
                let mut path = project_folder.clone();
                path.push("modules");
                path.push("mdedmo_2.move");
                write_all(
                    &path,
                    "module ModuleDemo2{
                        struct T1 {}
                        struct T2 {}

                        struct Demo<T> has drop{
                            value:u8
                        }
                        public fun new<T:drop>(value:u8): Demo<T>{
                            Demo<T>{
                                value
                            }
                        }
                    }",
                )
                .unwrap();
            }
            // project_folder/scripts/sdemo_5.move
            {
                let mut path = project_folder.clone();
                path.push("scripts");
                path.push("sdemo_5.move");
                write_all(
                    &path,
                    "script {
                        use 0x1::ModuleDemo2;

                        fun test_fun<T:drop>(value:u8) {
                            let _tmp:ModuleDemo2::Demo<T> = ModuleDemo2::new<T>(value);
                        }
                    }",
                )
                .unwrap();
            }

            let mut dove_ct = Command::new("cargo");
            dove_ct
                .args(&["run", "--", "ct", "test_fun<u8>(1)"])
                .args(&["-f", "sdemo_5"])
                .args(&["-o", "demo_5"])
                .current_dir(&project_folder);
            let command_string = format!("{:?} ", dove_ct).replace("\"", "");
            let result = dove_ct.output();
            assert!(
                result.is_ok(),
                "[ERROR]: {}\r\n[RUN]: {}\r\n",
                result.err().unwrap(),
                command_string,
            );

            let result = result.unwrap();
            let code = result.status.code().unwrap();
            assert_eq!(
                0,
                code,
                "[ERROR] Command: {}\r\nCode: {}\r\nError: {}\r\nOut: {}",
                command_string,
                code,
                String::from_utf8(result.stderr.clone()).unwrap(),
                String::from_utf8(result.stdout.clone()).unwrap(),
            );

            let mut tx_path = project_folder.clone();
            tx_path.push("target");
            tx_path.push("transactions");
            tx_path.push("demo_5.mvt");

            assert!(
                tx_path.exists(),
                "Transaction not found: {}\r\n[Command] {}",
                tx_path.to_str().unwrap(),
                &command_string,
            );

            let tx = bcs::from_bytes::<Transaction>(std::fs::read(&tx_path).unwrap().as_ref())
                .unwrap();
            let tx_fmt = format!("{:?}", tx);
            assert!(tx_fmt.contains(" args: [U8(1)]"));
            assert!(tx_fmt.contains(" type_args: [U8]"));
            assert!(tx_fmt.contains(" signers_count: 0"));

            remove_file(&tx_path).unwrap();
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
