#[cfg(test)]
mod test_dove_cmd {
    use std::io::{Write};
    use termcolor::{ColorChoice, WriteColor, ColorSpec, Color};
    use std::path::{PathBuf, Path};
    use std::process::{Command};
    use std::fs::create_dir_all;
    use fs_extra::file::write_all;

    // =============================================================================================
    // Tests
    // =============================================================================================
    /// Create a new move project
    /// Correct url: http://demo.ru/api, https://demo.ru/api, http://127.0.0.1/api, http://localhost/api, http://localhost:8080/api
    ///
    /// $ cargo run -- new ### -d ### -a ### -r http://localhost/api
    /// $ dove new ### -d ### -a ### -r http://localhost/api
    #[test]
    fn success_create_new_project() {
        vec![
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
        ]
        .iter()
        .for_each(|(dialect, address, api)| {
            success_create_new_project_and_build_with_settings(
                "demoproject_1".to_string(),
                dialect.map(|d: &str| d.to_string()),
                address.map(|a: &str| a.to_string()),
                api.map(|a: &str| a.to_string()),
            )
        });
    }

    /// Fail is expected
    /// Create a new move project
    /// invalid url: demo, /demo, /demo/api, //demo/api, //demo:8080/api, 127.0.0.1/api, ftp://demo.ru/api, ssh://demo.ru/api, smb://demo.ru/api
    /// $ cargo run -- new ### -d ### -a ### -r URL
    /// $ dove new ### -d ### -a ### -r URL
    #[test]
    fn fail_create_new_project_dealect_incorectdialect() {
        vec![
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
        ]
        .iter()
        .for_each(|(dialect,address,api)| {
            fail_create_new_project_with_settings(
                "demoproject_2".to_string(),
                dialect.map(|d| d.to_string()),
                address.map(|a: &str| a.to_string()),
                api.map(|a: &str| a.to_string())
            )
        });
    }

    /// Init directory as move project
    /// valid url: http://demo.ru/api, https://demo.ru/api, http://127.0.0.1/api, http://localhost/api, http://localhost:8080/api
    ///
    /// $ cargo run -- init -d ### -a ### -r http://localhost/api
    #[test]
    fn success_init_project_in_folder() {
        vec![
            (None, None, None),
            (Some("pont"), None, None),
            (
                Some("pont"),
                Some("1exaAg2VJRQbyUBAeXcktChCAqjVP9TUxF3zo23R2T6EGdE"),
                None,
            ),
            (Some("pont"), Some("0x1"), None),
            (Some("pont"), Some("0x1"), Some("http://demo.ru/api")),
            (Some("pont"), Some("0x1"), Some("https://demo.ru/api")),
            (Some("pont"), Some("0x1"), Some("http://127.0.0.1/api")),
            (Some("pont"), Some("0x1"), Some("http://localhost/api")),
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
            (Some("diem"), None, None),
            (Some("diem"), Some("0x1"), None),
            (Some("diem"), Some("0x1"), Some("http://demo.ru/api")),
            (Some("diem"), Some("0x1"), Some("https://demo.ru/api")),
            (Some("diem"), Some("0x1"), Some("http://127.0.0.1/api")),
            (Some("diem"), Some("0x1"), Some("http://localhost/api")),
        ]
        .iter()
        .for_each(|(dialect, address, api)| {
            init_project_with_settings(
                "demoproject_3".to_string(),
                dialect.map(|d| d.to_string()),
                address.map(|a| a.to_string()),
                api.map(|a: &str| a.to_string()),
            )
        });
    }

    /// demoproject_4
    ///
    /// $ cargo run -- clean
    /// $ dove clean
    #[test]
    fn check_clean() {
        let project_name = "demoproject_4".to_string();
        let dove_path = get_path_dove().expect("Dove path - not found");
        print_h1(format!("Dove: clean move project. {}", &project_name).as_str());
        print_ln();

        let mut project_folder = dove_path.clone();
        project_folder.push(&project_name);
        if project_folder.exists() {
            remove_project(&project_folder, &project_name);
        }

        let mut project_target = project_folder.clone();
        project_target.push("target");

        assert!(
            create_dir_all(project_target.as_path()).is_ok(),
            "Create dir: {}",
            project_target.to_str().unwrap_or(" - "),
        );

        let mut debug_dir = project_target.clone();
        debug_dir.push("debug");
        assert!(
            create_dir_all(debug_dir.as_path()).is_ok(),
            "Create dir: {}",
            debug_dir.to_str().unwrap_or(" - "),
        );

        let mut dove_toml = project_folder.clone();
        dove_toml.push("Dove.toml");
        if let Err(err) = write_all(
            &dove_toml,
            "\
                [package]\r\n\
                name = \"demoproject_4\"\r\n\
                dialect = \"pont\"\r\n\
                dependencies = [{ git = \"https://github.com/pontem-network/move-stdlib\" }]\r\n
            ",
        ) {
            assert!(
                false,
                "failed to create file {}; [ERROR] {}",
                dove_toml.to_str().unwrap_or(" - "),
                err.to_string()
            )
        }
        let mut create_command = Command::new("cargo");
        create_command
            .args(&["run", "--", "clean"])
            .current_dir(&project_folder);

        let command_string = format!("{:?} ", create_command).replace("\"", "");
        print!("{}", command_string);

        let result = create_command.output().map_or_else(
            |err| {
                print_ln();
                print_color_red("[ERROR] ");
                print_default(&command_string);
                print_ln();
                print_bold("Message: ");
                print_default(err.to_string().as_str());
                print_ln();
                None
            },
            |result| Some(result),
        );

        print_color_green("[SUCCESS]");
        print_ln();
        assert_ne!(result, None, "failed: {}", &command_string);
        assert!(
            !project_target.exists(),
            "Directory was not deleted: {}",
            project_target.to_str().unwrap_or(" - ")
        );
        // Deleting a project folder
        remove_project(&project_folder, &project_name);
    }

    /// demoproject_5
    /// $ cargo run -- metadata
    /// $ dove metadata
    #[test]
    fn check_metadata() {
        let dove_path = get_path_dove().expect("Dove path - not found");

        let project_name = "demoproject_5".to_string();
        let project_dialect = Some("pont".to_string());
        let blockchain_api = Some("https://localhost/api".to_string());
        let project_address =
            Some("5Csxuy81dNEVYbRA9K7tyHypu7PivHmwCZSKxcbU78Cy2v7v".to_string());

        let project_folder = {
            let mut folder = dove_path.clone();
            folder.push(&project_name);
            folder
        };

        if project_folder.exists() {
            remove_project(&project_folder, &project_name);
        }

        new_project(
            &project_name,
            &project_dialect,
            &project_address,
            &blockchain_api,
        );
        print_color_green("[SUCCESS]");
        print_ln();
        // =========================================================================================
        // $ cargo run -- metadata
        // =========================================================================================
        print_h2("Metadata project: ");
        let mut create_command = Command::new("cargo");
        create_command
            .args(&["run", "--", "metadata"])
            .current_dir(&project_folder);

        let command_string = format!("{:?} ", create_command).replace("\"", "");
        print!("{}", command_string);

        let result = create_command.output().map_or_else(
            |err| {
                print_ln();
                print_color_red("[ERROR] ");
                print_default(&command_string);
                print_ln();
                print_bold("Message: ");
                print_default(err.to_string().as_str());
                print_ln();
                None
            },
            |result| Some(result),
        );
        assert_ne!(result, None, "failed: {}", &command_string);
        let result = result.unwrap();
        let code = result.status.code().unwrap_or(0);
        let stderr = String::from_utf8(result.stderr).unwrap();
        if code != 0 {
            print_ln();
            print_color_red("[ERROR] ");
            print_default(&command_string);
            print_ln();
            print_bold("Code: ");
            print_default(result.status.to_string().as_str());
            print_ln();
            print_bold("Message: ");
            print_default(stderr.as_str());
            print_ln();
            assert_eq!(code, 0, "[ERROR] {}", stderr.as_str());
        }
        print_color_green("[SUCCESS]");
        print_ln();
        let stdout = String::from_utf8(result.stdout).unwrap();
        assert!(
            stdout.contains(&project_name),
            "Not found in metadata name: {}",
            &project_name
        );
        assert!(
            stdout.contains(project_address.as_ref().unwrap()),
            "Not found in metadata account_address: {}",
            project_address.as_ref().unwrap()
        );
        assert!(
            stdout.contains(project_dialect.as_ref().unwrap()),
            "Not found in metadata dialect: {}",
            project_dialect.as_ref().unwrap()
        );
        assert!(
            stdout.contains(blockchain_api.as_ref().unwrap()),
            "Not found in metadata blockchain_api: {}",
            blockchain_api.as_ref().unwrap()
        );

        remove_project(&project_folder, &project_name);
    }
    // =============================================================================================
    fn success_create_new_project_and_build_with_settings(
        project_name: String,
        project_dialect: Option<String>,
        project_address: Option<String>,
        blockchain_api: Option<String>,
    ) {
        let dove_path = get_path_dove().expect("Dove path - not found");
        let mut project_folder = dove_path.clone();
        project_folder.push(&project_name);

        print_h1(format!("Dove: New move project. {}", &project_name).as_str());
        print_ln();
        // Print project setting
        print_newproject_settings(
            &project_name,
            match &project_dialect {
                Some(dialect) => dialect,
                None => "pont (default)",
            },
            match &project_address {
                Some(address) => address,
                None => "None (default)",
            },
            match &blockchain_api {
                Some(api) => api,
                None => "None (default)",
            },
        );

        let mut list_projects = get_list_projects();
        if_exists_project_then_remove(&mut list_projects, &project_name);
        print_h2("Existing projects: ");
        print_ln();
        print_projects(&list_projects);

        new_project(
            &project_name,
            &project_dialect,
            &project_address,
            &blockchain_api,
        );

        let mut project_path = dove_path.clone();
        project_path.push(&project_name);

        success_check_config(
            &project_path,
            &project_name,
            &project_dialect,
            &project_address,
            &blockchain_api,
        );

        print_color_green("[SUCCESS]");
        print_ln();

        // $ cargo run -- build demoproject_#
        print_h2("Building project ");
        let mut create_command = Command::new("cargo");
        create_command
            .args(&["run", "--", "build", &project_name])
            .current_dir(&project_folder);
        let command_string = format!("{:?} ", create_command).replace("\"", "");
        print!("{}", command_string);
        let result = create_command.output().map_or_else(
            |err| {
                print_ln();
                print_color_red("[ERROR] ");
                print_default(&command_string);
                print_ln();
                print_bold("Message: ");
                print_default(err.to_string().as_str());
                print_ln();
                None
            },
            |result| Some(result),
        );

        assert_ne!(result, None, "failed: {}", command_string);
        let result = result.unwrap();
        let code = result.status.code().unwrap_or(0);
        let stderr = String::from_utf8(result.stderr).unwrap();
        if code != 0 {
            print_ln();
            print_color_red("[ERROR] ");
            print_default(&command_string);
            print_ln();
            print_bold("Code: ");
            print_default(result.status.to_string().as_str());
            print_ln();
            print_bold("Message: ");
            print_default(stderr.as_str());
            print_ln();
            assert_eq!(code, 0, "[ERROR] {}", stderr.as_str());
        }

        print_color_green("[SUCCESS]");
        print_ln();

        list_projects = get_list_projects();
        print_ln();
        print_h2("Current list of projects: ");
        print_ln();
        print_projects(&list_projects);

        // Deleting a project folder
        if let Some(finded) = list_projects.as_ref().unwrap().iter().find(|it| {
            it.as_os_str()
                .to_str()
                .unwrap_or("")
                .contains(&project_name)
        }) {
            assert!(
                remove_project(finded, &project_name),
                "[ERROR] remove project {};",
                project_name
            );
        }
        assert!(true);
    }
    fn fail_create_new_project_with_settings(
        project_name: String,
        project_dialect: Option<String>,
        project_address: Option<String>,
        blockchain_api: Option<String>,
    ) {
        let dove_path = get_path_dove().expect("Dove path - not found");
        print_h1(format!("Dove: New move project. {}", &project_name).as_str());
        print_ln();
        // Print project settings
        print_newproject_settings(
            &project_name,
            match &project_dialect {
                Some(dialect) => dialect,
                None => "pont (default)",
            },
            match &project_address {
                Some(address) => address,
                None => "None (default)",
            },
            match &blockchain_api {
                Some(api) => api,
                None => "None (default)",
            },
        );

        let mut list_projects = get_list_projects();
        if_exists_project_then_remove(&mut list_projects, &project_name);

        print_h2("Existing projects: ");
        print_ln();
        print_projects(&list_projects);

        // $ cargo run -- new demoproject_### [-d dealect] [-a address]
        print_h2("Create project: ");
        let mut create_command = Command::new("cargo");
        create_command
            .args(&["run", "--", "new", &project_name])
            .current_dir(&dove_path);
        if let Some(dialect) = project_dialect.as_ref() {
            create_command.args(&["-d", dialect]);
        }
        if let Some(address) = project_address.as_ref() {
            create_command.args(&["-a", address]);
        }
        if let Some(api) = blockchain_api.as_ref() {
            create_command.args(&["-r", api]);
        }

        let command_string = format!("{:?} ", create_command).replace("\"", "");
        print!("{}", command_string);
        let result = create_command.output().map_or_else(
            |err| {
                print_ln();
                print_color_red("[ERROR] ");
                print_default(&command_string);
                print_ln();
                print_bold("Message: ");
                print_default(err.to_string().as_str());
                print_ln();
                None
            },
            |result| Some(result),
        );
        assert_ne!(result, None, "failed: {}", &command_string);

        let result = result.unwrap();
        let code = result.status.code().unwrap_or(0);
        let stderr = String::from_utf8(result.stderr).unwrap();
        if code == 0 {
            print_ln();
            print_color_red("[ERROR] was created: ");
            print_default(&command_string);
            print_ln();
            print_bold("Code: ");
            print_default(result.status.to_string().as_str());
            print_ln();
            print_bold("Message: ");
            print_default(stderr.as_str());
            print_ln();
            assert_ne!(code, 0, "[ERROR] was created: {} ", &command_string);
        }
        print_ln();
        print_color_green("[NOT CREATED]");
        print_ln();
        print_ln();

        if let Some(finded) = list_projects.as_ref().unwrap().iter().find(|it| {
            it.as_os_str()
                .to_str()
                .unwrap_or("")
                .contains(&project_name)
        }) {
            assert!(
                remove_project(finded, &project_name),
                "[ERROR] remove project: {};",
                project_name
            );
        }
        assert!(true);
    }
    fn init_project_with_settings(
        project_name: String,
        project_dialect: Option<String>,
        project_address: Option<String>,
        blockchain_api: Option<String>,
    ) {
        let dove_path = get_path_dove().expect("Dove path - not found");
        print_h1(format!("Dove: init move project. {}", &project_name).as_str());
        print_ln();

        let project_folder_str =
            dove_path.as_path().to_str().unwrap().to_string() + "/" + project_name.as_str();
        let project_folder = Path::new(&project_folder_str);

        print_newproject_settings(
            &project_name,
            match &project_dialect {
                Some(dialect) => dialect,
                None => "pont (default)",
            },
            match &project_address {
                Some(address) => address,
                None => "None (default)",
            },
            match &blockchain_api {
                Some(api) => api,
                None => "None (default)",
            },
        );
        print_bold("Directory: ");
        print_default(&project_folder_str);
        print_ln();

        if project_folder.exists() {
            print_color_yellow("[WARNING] ");
            print_default(format!("directory exists {}", &project_folder_str).as_str());
            print_ln();
            assert!(
                remove_project(&project_folder.to_path_buf(), &project_name),
                "[ERROR] remove project: {};",
                project_name
            );
        }
        match std::fs::create_dir(&project_folder) {
            Ok(_) => {
                print_color_green("[SUCCESS] ");
                print_default(
                    format!("Project directory created  {}", &project_folder_str).as_str(),
                );
                print_ln();
            }
            Err(err) => {
                print_color_red("[ERROR] ");
                print_default(
                    format!(
                        "Couldn't create project directory {}; {}",
                        &project_folder_str,
                        err.to_string()
                    )
                    .as_str(),
                );
                print_ln();
                assert!(
                    false,
                    "Couldn't create project directory {}",
                    &project_folder_str
                );
            }
        }
        // =========================================================================================
        // $ cargo run -- init
        // =========================================================================================
        print_h2("init project: ");
        let mut init_command = Command::new("cargo");
        init_command
            .args(&["run", "--", "init"])
            .current_dir(&project_folder);
        if let Some(dialect) = project_dialect.as_ref() {
            init_command.args(&["-d", dialect]);
        }
        if let Some(address) = project_address.as_ref() {
            init_command.args(&["-a", address]);
        }
        if let Some(api) = blockchain_api.as_ref() {
            init_command.args(&["-r", api]);
        }

        let command_string = format!("{:?} ", init_command).replace("\"", "");
        print!("{}", command_string);

        let result = init_command.output().map_or_else(
            |err| {
                print_ln();
                print_color_red("[ERROR] ");
                print_default(&command_string);
                print_ln();
                print_bold("Message: ");
                print_default(err.to_string().as_str());
                print_ln();
                None
            },
            |result| Some(result),
        );
        assert_ne!(result, None, "failed: {}", command_string);
        let result = result.unwrap();
        let code = result.status.code().unwrap_or(0);
        let stderr = String::from_utf8(result.stderr).unwrap();
        if code != 0 {
            print_ln();
            print_color_red("[ERROR] ");
            print_default(&command_string);
            print_ln();
            print_bold("Code: ");
            print_default(result.status.to_string().as_str());
            print_ln();
            print_bold("Message: ");
            print_default(stderr.as_str());
            print_ln();
            assert_eq!(code, 0, "[ERROR] {}", stderr.as_str());
        }

        success_check_config(
            &project_folder.to_path_buf(),
            &project_name,
            &project_dialect,
            &project_address,
            &blockchain_api,
        );

        print_color_green("[SUCCESS]");
        print_ln();
        // =========================================================================================
        // $ cargo run -- build
        // =========================================================================================
        print_h2("Building project ");
        let mut create_command = Command::new("cargo");
        create_command
            .args(&["run", "--", "build"])
            .current_dir(&project_folder);
        let command_string = format!("{:?} ", create_command).replace("\"", "");
        print!("{}", command_string);
        let result = create_command.output().map_or_else(
            |err| {
                print_ln();
                print_color_red("[ERROR] ");
                print_default(&command_string);
                print_ln();
                print_bold("Message: ");
                print_default(err.to_string().as_str());
                print_ln();
                None
            },
            |result| Some(result),
        );

        assert_ne!(result, None, "{}", &command_string);
        let result = result.unwrap();
        let code = result.status.code().unwrap_or(0);
        let stderr = String::from_utf8(result.stderr).unwrap();
        if code != 0 {
            print_ln();
            print_color_red("[ERROR] ");
            print_default(&command_string);
            print_ln();
            print_bold("Code: ");
            print_default(result.status.to_string().as_str());
            print_ln();
            print_bold("Message: ");
            print_default(stderr.as_str());
            print_ln();
            assert_eq!(code, 0, "[ERROR] {}", stderr.as_str());
        }
        print_color_green("[SUCCESS]");
        print_ln();
        assert!(true);
    }

    fn if_exists_project_then_remove(
        list_projects: &mut Option<Vec<PathBuf>>,
        project_name: &str,
    ) {
        if let Some(list) = list_projects.as_ref() {
            if let Some(finded) = list.iter().find(|it| {
                it.as_os_str()
                    .to_str()
                    .unwrap_or("")
                    .contains(&project_name)
            }) {
                print_color_yellow("[WARNING] ");
                print_default(format!("directory exists {}", project_name).as_str());
                print_ln();
                assert!(
                    remove_project(finded, &project_name),
                    "[ERROR] remove directory {};",
                    project_name
                );

                *list_projects = get_list_projects();
                print_ln();
            }
        }
    }

    fn new_project(
        name: &str,
        dialect: &Option<String>,
        address: &Option<String>,
        blockchain_api: &Option<String>,
    ) {
        let dove_path = get_path_dove().expect("Dove path - not found");

        // $ cargo run -- new demoproject_### [-d ###] [-a ###] [-r ###]
        print_h2("Create project: ");
        let mut create_command = Command::new("cargo");
        create_command
            .args(&["run", "--", "new", &name])
            .current_dir(&dove_path);
        if let Some(dialect) = dialect.as_ref() {
            create_command.args(&["-d", dialect]);
        }
        if let Some(address) = address.as_ref() {
            create_command.args(&["-a", address]);
        }
        if let Some(api) = blockchain_api.as_ref() {
            create_command.args(&["-r", api]);
        }

        let command_string = format!("{:?} ", create_command).replace("\"", "");
        print!("{}", command_string);

        let result = create_command.output().map_or_else(
            |err| {
                print_ln();
                print_color_red("[ERROR] ");
                print_default(&command_string);
                print_ln();
                print_bold("Message: ");
                print_default(err.to_string().as_str());
                print_ln();
                None
            },
            |result| Some(result),
        );
        assert_ne!(result, None, "failed: {}", &command_string);
        let result = result.unwrap();
        let code = result.status.code().unwrap_or(0);
        let stderr = String::from_utf8(result.stderr).unwrap();
        if code != 0 {
            print_ln();
            print_color_red("[ERROR] ");
            print_default(&command_string);
            print_ln();
            print_bold("Code: ");
            print_default(result.status.to_string().as_str());
            print_ln();
            print_bold("Message: ");
            print_default(stderr.as_str());
            print_ln();
            assert_eq!(code, 0, "[ERROR] {}", stderr.as_str());
        }
    }
    fn success_check_config(
        path_project: &PathBuf,
        need_name: &String,
        need_dialect: &Option<String>,
        need_address: &Option<String>,
        need_blockchain_api: &Option<String>,
    ) {
        use std::fs::read_to_string;
        use toml::Value;

        let mut path_toml = path_project.clone();
        path_toml.push("Dove.toml");

        let toml_str = read_to_string(path_toml).unwrap();
        let package = toml_str
            .parse::<Value>()
            .unwrap()
            .get("package")
            .map_or(toml::Value::String("- NULL -".to_string()), |d| d.clone());

        let project_name = package
            .get("name")
            .and_then(|v| v.as_str())
            .map_or("- NULL -".to_string(), |v| v.to_string());
        assert_eq!(
            project_name,
            need_name.clone(),
            "Dove.toml: invalid name or not found",
        );

        let project_dialect = package
            .get("dialect")
            .and_then(|v| v.as_str())
            .map_or("- NULL -".to_string(), |v| v.to_string());
        assert_eq!(
            project_dialect,
            need_dialect
                .as_ref()
                .map_or("- NULL -".to_string(), |s| s.clone()),
            "Dove.toml: invalid dialect or not found",
        );

        let project_account_address = package
            .get("account_address")
            .and_then(|v| v.as_str())
            .map_or("- NULL -".to_string(), |v| v.to_string());
        assert_eq!(
            project_account_address,
            need_address
                .as_ref()
                .map_or("- NULL -".to_string(), |s| s.clone()),
            "Dove.toml: invalid account_address or not found",
        );

        let project_api = package
            .get("blockchain_api")
            .and_then(|v| v.as_str())
            .map_or("- NULL -".to_string(), |v| v.to_string());
        assert_eq!(
            project_api,
            need_blockchain_api
                .as_ref()
                .map_or("- NULL -".to_string(), |s| s.clone()),
            "Dove.toml: invalid blockchain_api or not found",
        );
    }
    // =============================================================================================
    fn get_path_dove() -> Option<PathBuf> {
        isset_path_dove(".").or(isset_path_dove("./dove"))
    }
    fn isset_path_dove(path: &str) -> Option<PathBuf> {
        Path::new(path)
            .canonicalize()
            .map_or(None, |p| p.to_str().map_or(None, |p| Some(p.to_string())))
            .and_then(|p| {
                #[cfg(not(windows))]
                if let Some(pos) = (p.clone() + "/").find("/dove/") {
                    let p = { &p[..pos] }.to_string() + "/dove";
                    return Some(PathBuf::from(&p));
                }
                #[cfg(windows)]
                if let Some(pos) = (p.clone() + "\\").find("\\dove\\") {
                    let p = { &p[..pos] }.to_string() + "\\dove";
                    return Some(PathBuf::from(&p));
                }

                None
            })
    }
    fn get_list_projects() -> Option<Vec<PathBuf>> {
        use std::fs::read_dir;
        let need_folders = vec!["modules", "scripts", "tests"];
        let need_files = vec!["Dove.toml"];

        get_path_dove()
            .and_then(|folder| read_dir(folder).map_or(None, |resource| Some(resource)))
            .and_then(|resource| {
                Some(
                    resource
                        .filter_map(|path| path.ok())
                        .map(|path| path.path())
                        .filter(|path| path.is_dir())
                        .filter_map(|path| {
                            read_dir(&path)
                                .map_or(None, |dir| Some(dir))
                                .and_then(|dir| {
                                    let finded: Vec<PathBuf> = dir
                                        .filter_map(|p| p.ok())
                                        .map(|p| p.path())
                                        .filter_map(|p| {
                                            let file_name = p
                                                .file_name()
                                                .map_or("", |name| name.to_str().unwrap_or(""));
                                            if (p.is_dir() && need_folders.contains(&file_name))
                                                || (p.is_file()
                                                    && need_files.contains(&file_name))
                                            {
                                                Some(p)
                                            } else {
                                                None
                                            }
                                        })
                                        .collect();
                                    if finded.len() == need_files.len() + need_folders.len() {
                                        Some(path.clone())
                                    } else {
                                        None
                                    }
                                })
                        })
                        .collect(),
                )
            })
    }
    fn remove_project(path: &PathBuf, project_name: &str) -> bool {
        if let Err(error) = std::fs::remove_dir_all(path) {
            print_default(
                format!("Couldn't delete project directory {}  ", project_name).as_str(),
            );
            print_color_red("[ERROR]");
            print_ln();
            print_bold("Message: ");
            print_default(error.to_string().as_str());
            print_ln();
            false
        } else {
            print_default(format!("Project directory was deleted {} ", project_name).as_str());
            print_color_green("[SUCCESS]");
            print_ln();
            true
        }
    }
    // =============================================================================================
    // Print
    // =============================================================================================
    fn print_newproject_settings(
        project_name: &str,
        project_dialect: &str,
        project_address: &str,
        blockchain_api: &str,
    ) {
        print_h2("New project settings:\n");
        print_bold(format!("Project will be created: ").as_str());
        print_reset();
        print_default(format!("{} \n", project_name).as_str());

        print_bold(format!("Dialect: ").as_str());
        print_reset();
        print_default(format!("{} \n", project_dialect).as_str());

        print_bold(format!("Address: ").as_str());
        print_reset();
        print_default(format!("{} \n", project_address).as_str());

        print_bold(format!("Blockchain API: ").as_str());
        print_reset();
        print_default(format!("{} \n", blockchain_api).as_str());
        print_ln();
    }
    fn print_project(project_path: &PathBuf) {
        use std::fs::read_to_string;
        use toml::Value;

        let mut path_toml = project_path.clone();
        path_toml.push("Dove.toml");

        let toml_str = read_to_string(path_toml).unwrap();
        let package = toml_str
            .parse::<Value>()
            .unwrap()
            .get("package")
            .map_or(toml::Value::String("- NULL -".to_string()), |d| d.clone());
        let project_name = package
            .get("name")
            .and_then(|v| v.as_str())
            .map_or("- NULL -".to_string(), |v| v.to_string());
        let project_dialect = package
            .get("dialect")
            .and_then(|v| v.as_str())
            .map_or("- NULL -".to_string(), |v| v.to_string());
        let project_account_address = package
            .get("account_address")
            .and_then(|v| v.as_str())
            .map_or("- NULL -".to_string(), |v| v.to_string());
        let project_api = package
            .get("blockchain_api")
            .and_then(|v| v.as_str())
            .map_or("- NULL -".to_string(), |v| v.to_string());
        let project_dependencies = package
            .get("dependencies")
            .and_then(|v| v.get(0))
            .and_then(|v| v.get("git"))
            .and_then(|v| v.as_str())
            .map_or("- NULL -".to_string(), |v| v.to_string());

        print_h3({ "Project: ".to_string() + &project_name }.as_str());
        print_ln();

        print_bold("Name: ");
        print_default(&project_name);
        print_ln();

        print_bold("Dialect: ");
        print_default(&project_dialect);
        print_ln();

        print_bold("Account address: ");
        print_default(&project_account_address);
        print_ln();

        print_bold("Blockchain API: ");
        print_default(&project_api);
        print_ln();

        print_bold("Dependencies: ");
        print_default(&project_dependencies);
        print_ln();

        print_ln();
    }
    fn print_projects(projects: &Option<Vec<PathBuf>>) {
        projects.as_ref().map_or_else(
            || {
                print_default("- empty list -");
            },
            |projects| {
                projects.iter().for_each(|p| print_project(p));
            },
        );
    }
    // =============================================================================================
    // Decoration
    // =============================================================================================
    fn print_reset() {
        let bufwrt = termcolor::BufferWriter::stdout(ColorChoice::Always);
        let mut buffer = bufwrt.buffer();
        buffer.reset().unwrap();
        write!(&mut buffer, "").unwrap();
        let t = String::from_utf8_lossy(buffer.as_slice()).to_string();
        print!("{}", t);
    }
    fn print_default(text: &str) {
        let bufwrt = termcolor::BufferWriter::stdout(ColorChoice::Always);
        let mut buffer = bufwrt.buffer();
        buffer
            .set_color(ColorSpec::new().set_bold(false).set_underline(false))
            .unwrap();
        write!(&mut buffer, "{}", text).unwrap();
        let t = String::from_utf8_lossy(buffer.as_slice()).to_string();
        print!("{}", t);
    }
    fn print_ln() {
        print_default("\n");
    }

    fn print_h1(text: &str) {
        let bufwrt = termcolor::BufferWriter::stdout(ColorChoice::Always);
        let mut buffer = bufwrt.buffer();
        buffer
            .set_color(ColorSpec::new().set_bold(true).set_underline(true))
            .unwrap();
        write!(&mut buffer, "{}", text.to_uppercase()).unwrap();
        let t = String::from_utf8_lossy(buffer.as_slice()).to_string();
        print!("{}", t);
    }
    fn print_h2(text: &str) {
        let bufwrt = termcolor::BufferWriter::stdout(ColorChoice::Always);
        let mut buffer = bufwrt.buffer();
        buffer.set_color(ColorSpec::new().set_bold(true)).unwrap();
        write!(&mut buffer, "{}", text.to_uppercase()).unwrap();
        let t = String::from_utf8_lossy(buffer.as_slice()).to_string();
        print!("{}", t);
    }
    fn print_h3(text: &str) {
        let bufwrt = termcolor::BufferWriter::stdout(ColorChoice::Always);
        let mut buffer = bufwrt.buffer();
        buffer
            .set_color(ColorSpec::new().set_underline(true))
            .unwrap();
        write!(&mut buffer, "{}", text.to_uppercase()).unwrap();
        let t = String::from_utf8_lossy(buffer.as_slice()).to_string();
        print!("{}", t);
    }
    fn print_bold(text: &str) {
        let bufwrt = termcolor::BufferWriter::stdout(ColorChoice::Always);
        let mut buffer = bufwrt.buffer();
        buffer.set_color(ColorSpec::new().set_bold(true)).unwrap();
        write!(&mut buffer, "{}", text).unwrap();
        let t = String::from_utf8_lossy(buffer.as_slice()).to_string();
        print!("{}", t);
    }
    fn print_color_red(text: &str) {
        let bufwrt = termcolor::BufferWriter::stdout(ColorChoice::Always);
        let mut buffer = bufwrt.buffer();
        buffer
            .set_color(ColorSpec::new().set_fg(Some(Color::Red)))
            .unwrap();
        write!(&mut buffer, "{}", text).unwrap();
        let t = String::from_utf8_lossy(buffer.as_slice()).to_string();
        print!("{}", t);
    }
    fn print_color_green(text: &str) {
        let bufwrt = termcolor::BufferWriter::stdout(ColorChoice::Always);
        let mut buffer = bufwrt.buffer();
        buffer
            .set_color(ColorSpec::new().set_fg(Some(Color::Green)))
            .unwrap();
        write!(&mut buffer, "{}", text).unwrap();
        let t = String::from_utf8_lossy(buffer.as_slice()).to_string();
        print!("{}", t);
    }
    fn print_color_yellow(text: &str) {
        let bufwrt = termcolor::BufferWriter::stdout(ColorChoice::Always);
        let mut buffer = bufwrt.buffer();
        buffer
            .set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))
            .unwrap();
        write!(&mut buffer, "{}", text).unwrap();
        let t = String::from_utf8_lossy(buffer.as_slice()).to_string();
        print!("{}", t);
    }
}
