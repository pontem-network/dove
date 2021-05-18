/// Fail is expected
/// Create a new move project
/// invalid url: demo, /demo, /demo/api, //demo/api, //demo:8080/api, 127.0.0.1/api, ftp://demo.ru/api, ssh://demo.ru/api, smb://demo.ru/api
/// $ cargo run -- new ### [-d ###] [-a ###] [-r ###]
/// $ dove new ### [-d ###] [-a ###] [-r ###]
#[cfg(test)]
mod dove_new_cmd_fail {
    use std::path::{Path};
    use std::process::{Command};
    use std::fs::{remove_dir_all};

    /// project name: demoproject_14
    #[test]
    fn fail() {
        // Path to dove folder
        let dove_folder = {
            let mut folder = Path::new(".").canonicalize().unwrap();
            if folder.to_str().unwrap().find("dove").is_none() {
                folder.push("dove");
            }
            folder
        };
        // Project name and path
        let project_name = "demoproject_14";
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
        // $ dove new demoproject_14 [-d ###] [-a ###] [-r ###]
        // $ dove build
        for (dialect, address, blockchain_api) in vec![
            (Some("incorectdialect"), None, None),
            // Max address 32 byte
            (
                Some("pont"),
                Some("w01234567890123456789012345678901234567890123456789012345678901234567890123456789"),
                None
            ),
            (Some("pont"), Some("5GuurAd1g85AqSk9fhvA8iZ6QLQUSUBrsfUGBbLQCG9Fg8ct"), Some("demo")),
            (Some("pont"), Some("5F6mdBEfR19qj64hRK5yUSGyhAiqAHRUTuA2w3SATo46Y4ph"), Some("/demo")),
            (Some("pont"), Some("5DDYt7bqnnWSJGyySkVgfB918NngEvj77v4AaF5S6zRpfLc2"), Some("/demo/api")),
            (Some("pont"), Some("5DUcaioKfthyqaFfuLvpcxqhtpRt3XSJ6ouN9rmbXn4YEako"), Some("//demo/api")),
            (Some("pont"), Some("5CmBg2tSBUqYxXzYS1uXXdb1N1XZLidD43pswY4TeLNLc4Rb"), Some("//demo:8080/api")),
            (Some("pont"), Some("5D9uikgB3eK1hmN7neiopJxkXuh4bcAafiLpu2eVJmN3AmsG"), Some("127.0.0.1/api")),
            (Some("pont"), Some("5CkHD1imfRDhSEg8yZ2vqot3zWnwmpi2EFaczzKp8WipFpMV"), Some("ftp://demo.ru/api")),
            (
                Some("pont"),
                Some("5D2Z74VUUkK6YDKDPhewG9KQRQRPPbmv8A8N2pG6rZ7sLdBP"),
                Some("ssh://demo.ru/api")
            ),
            (
                Some("pont"),
                Some("5FL5CCppHTKYEwZ2A6HdBvp4BcrauqmJ6bW3r7vJ8pgrJRnQ"),
                Some("smb://demo.ru/api")
            ),
            // Max address 16 byte
            (
                Some("dfinance"),
                Some("w01234567890123456789012345678901234567890123456789012345678901234567890123456789"),
                None
            ),
            (Some("dfinance"), Some("5CQ1xHD9FekYCBWRu7RmFqoF5obnym9agvSiotSoHsxftQyx"), None),
            // Max address 16 byte
            (
                Some("diem"),
                Some("w01234567890123456789012345678901234567890123456789012345678901234567890123456789"),
                None
            ),
            (Some("diem"), Some("5HTUSpP8tEbNDKWu2Jabd9Px7epMXHusSpyAMmDZkV5QucnF"), None)
        ] {
            // $ cargo run -- new demoproject_14 [-d ###] [-a ###] [-r ###]
            // $ dove new demoproject_14 [-d ###] [-a ###] [-r ###]
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
                assert_ne!(code, 0, "[ERROR] was created\r\nCommand: {}\r\n", &command_string);
            }
        }
    }
}
