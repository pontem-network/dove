use std::path::{Path, PathBuf};
use fs_extra::file::write_all;
use dove::tests_helper::{
    project_remove, execute_dove_bin_at, project_start_new_and_add_dependencies, execute_dove_at,
    project_start_new_default, set_dependency_in_toml,
};
use dove::manifest::Git;
use itertools::Itertools;

#[test]
fn test_dependency_with_git_tag() {
    for v in 1..=2 {
        let project_folder = create_project_for_test_dependency(
            "project_dependency_with_git_tag",
            None,
            Some(&format!("v{}", v)),
            None,
        );

        // project_folder/scripts/version.move
        add_script_getversion(&project_folder);

        let output = execute_dove_bin_at(
            env!("CARGO_BIN_EXE_dove"),
            &["dove", "run", "version()"],
            &project_folder,
        )
        .unwrap();

        assert!(output.contains(&format!("[debug] {}", v)));

        project_remove(&project_folder);
    }
}

#[test]
fn test_dependency_with_rev() {
    for (value, rev) in &[
        (1, "049200421c880f9f4269d6406c2b1537891b23c7"),
        (2, "c276307c355d3c72e3daeb80f46e21272c6fab97"),
    ] {
        let project_folder = create_project_for_test_dependency(
            "project_dependency_with_rev",
            None,
            None,
            Some(rev),
        );

        // project_folder/scripts/version.move
        add_script_getversion(&project_folder);

        let output = execute_dove_bin_at(
            env!("CARGO_BIN_EXE_dove"),
            &["dove", "run", "version()"],
            &project_folder,
        )
        .unwrap();

        assert!(output.contains(&format!("[debug] {}", value)));

        project_remove(&project_folder);
    }
}

#[test]
fn test_dependency_without_dove_toml() {
    let project_folder = create_project_for_test_dependency(
        "project_dependency_without_dove_toml",
        Some("no_dove_toml"),
        None,
        None,
    );

    // project_folder/scripts/version.move
    add_script_getversion(&project_folder);

    let output = execute_dove_bin_at(
        env!("CARGO_BIN_EXE_dove"),
        &["dove", "run", "version()"],
        &project_folder,
    )
    .unwrap();

    assert!(output.contains("[debug] 3"));

    project_remove(&project_folder);
}

#[test]
fn test_dependency_in_dependencies() {
    let project_folder = create_project_for_test_dependency(
        "project_dependency_in_dependencies",
        Some("path"),
        None,
        None,
    );

    // project_folder/scripts/sum.move
    write_all(
        &project_folder.join("scripts").join("sum.move"),
        r#"script {
            use 0x1::Sum;
            use 0x1::Debug;
            fun sum(){
                Debug::print<u8>(&Sum::sum(1,2));
            }
        }"#,
    )
    .unwrap();

    let output = execute_dove_bin_at(
        env!("CARGO_BIN_EXE_dove"),
        &["dove", "run", "sum()"],
        &project_folder,
    )
    .unwrap();

    assert!(output.contains("[debug] 3"));

    project_remove(&project_folder);
}

#[test]
fn test_dependency_cyclic_dependency_in_the_repository() {
    let project_folder = create_project_for_test_dependency(
        "project_cyclic_dependency_in_the_repository",
        Some("test_rec"),
        None,
        None,
    );

    execute_dove_bin_at(
        env!("CARGO_BIN_EXE_dove"),
        &["dove", "run", "main()"],
        &project_folder,
    )
    .unwrap();

    project_remove(&project_folder);
}

#[test]
fn test_dependency_running_a_script_in_dependencies() {
    let project_folder = create_project_for_test_dependency(
        "project_running_a_script_in_dependencies",
        Some("script_main"),
        None,
        None,
    );

    let output = execute_dove_bin_at(
        env!("CARGO_BIN_EXE_dove"),
        &["dove", "run", "main()"],
        &project_folder,
    )
    .unwrap();

    assert_eq!(output.matches("[debug]").count(), 1);

    project_remove(&project_folder);
}

#[test]
fn test_removing_old_external_dependencies() {
    let project_folder = project_start_new_default("project_removing_old_external_dependencies");
    let stdlib: Git = Git {
        git: "https://github.com/pontem-network/move-stdlib".to_string(),
        branch: None,
        tag: Some("v0.1.2".to_string()),
        rev: None,
        path: None,
    };

    let steps = vec![
        vec![
            stdlib.clone(),
            Git {
                git: "https://github.com/pontem-network/test-dove-dependency".to_string(),
                branch: None,
                tag: Some("v1".to_string()),
                rev: None,
                path: None,
            },
        ],
        vec![],
        vec![stdlib.clone()],
        vec![
            stdlib.clone(),
            Git {
                git: "https://github.com/pontem-network/test-dove-dependency".to_string(),
                branch: None,
                tag: None,
                rev: None,
                path: None,
            },
        ],
        vec![
            stdlib.clone(),
            Git {
                git: "https://github.com/pontem-network/test-dove-dependency".to_string(),
                branch: Some("master".to_string()),
                tag: None,
                rev: None,
                path: None,
            },
        ],
        vec![
            stdlib,
            Git {
                git: "https://github.com/pontem-network/test-dove-dependency".to_string(),
                branch: Some("no_dove_toml".to_string()),
                tag: None,
                rev: None,
                path: None,
            },
        ],
    ];

    for dep in steps {
        set_dependency_in_toml(&project_folder, &dep).unwrap();
        execute_dove_at(&["dove", "build"], &project_folder).unwrap();
        check_external(&project_folder, &dep);
    }

    project_remove(&project_folder);
}

fn add_script_getversion(project_folder: &Path) {
    // project_folder/scripts/version.move
    write_all(
        &project_folder.join("scripts").join("version.move"),
        r#"script {
            use 0x1::Version;
            use 0x1::Debug;
            fun version(){
                Debug::print<u8>(&Version::get());
            }
        }"#,
    )
    .unwrap();
}

fn create_project_for_test_dependency(
    project_name: &str,
    branch: Option<&str>,
    tag: Option<&str>,
    rev: Option<&str>,
) -> PathBuf {
    let rep = Git {
        git: "https://github.com/pontem-network/test-dove-dependency".to_string(),
        branch: branch.map(|b| b.to_string()),
        tag: tag.map(|b| b.to_string()),
        rev: rev.map(|b| b.to_string()),
        path: None,
    };

    let project_folder = project_start_new_and_add_dependencies(project_name, vec![rep]);

    // project_folder/scripts/main.move
    write_all(
        &project_folder.join("scripts").join("main.move"),
        "script {\
                use 0x1::Debug;\
                fun main(){\
                    Debug::print<u8>(&1);\
                }\
            }",
    )
    .unwrap();

    project_folder
}

fn check_external(project_folder: &Path, dep: &[Git]) {
    let finded_folders: Vec<String> = project_folder
        .join("artifacts")
        .join(".external")
        .read_dir()
        .unwrap()
        .map(|path| {
            path.unwrap()
                .path()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string()
        })
        .sorted()
        .collect();
    let expected_folders: Vec<String> = dep
        .iter()
        .map(|g| g.local_name().unwrap())
        .sorted()
        .collect();

    assert_eq!(finded_folders, expected_folders);
}
