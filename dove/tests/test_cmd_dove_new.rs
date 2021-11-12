mod helper;

use std::str::FromStr;
use crate::helper::{
    pre_start, execute_dove_at, delete_project, get_project_name_from_toml,
    get_project_dialect_from_toml, assert_basic_project_dirs_exist,
    get_account_address_from_toml,
};
use dialect::Dialect;

/// Creating a default project without additional parameters
/// $ dove new project_new_without_arguments
#[test]
fn test_cmd_dove_new_without_arguments() {
    // Project name and path
    let project_name = "project_new_without_arguments";
    let (base_path, project_path) = pre_start(&project_name).unwrap();

    execute_dove_at(&["new", project_name], &base_path).unwrap();

    assert_eq!(
        get_project_name_from_toml(&project_path),
        Some(project_name.to_string())
    );
    assert_eq!(get_project_dialect_from_toml(&project_path), None);
    assert!(assert_basic_project_dirs_exist(&project_path).is_ok());

    delete_project(&project_path).unwrap();
}

/// Checking the "minimal" parameter
/// $ dove new project_new_with_minimal --minimal
#[test]
fn test_cmd_dove_new_with_minimal() {
    // Project name and path
    let project_name = "project_new_with_minimal";
    let (base_path, project_path) = pre_start(&project_name).unwrap();

    execute_dove_at(&["new", project_name, "--minimal"], &base_path).unwrap();
    assert!(assert_basic_project_dirs_exist(&project_path).is_err());

    delete_project(&project_path).unwrap();
}

/// Creating a project with different dialects. Dialects: "pont", "diem", "dfinance"
/// $ dove new project_new_with_dialect --dialect ###
#[test]
fn test_cmd_dove_new_with_dialect() {
    // Project name and path
    let project_name = "project_new_with_dialect";
    let (base_path, project_path) = pre_start(&project_name).unwrap();

    for dialect_name in ["pont", "diem", "dfinance"] {
        execute_dove_at(
            &["new", project_name, "--dialect", dialect_name],
            &base_path,
        )
        .unwrap();
        assert_eq!(
            get_project_dialect_from_toml(&project_path),
            Some(dialect_name.to_string())
        );

        delete_project(&project_path).unwrap();
    }
}

/// Creating a project with a non-existent dialect
/// $ dove new project_new_with_nonexistent_dialect --dialect noname
#[test]
fn test_cmd_dove_new_with_nonexistent_dialect() {
    // Project name and path
    let project_name = "project_new_with_nonexistent_dialect";
    let (base_path, _) = pre_start(&project_name).unwrap();

    assert!(execute_dove_at(&["new", project_name, "--dialect", "noname"], &base_path).is_err());
}

/// Creating a project with an address
/// $ dove new project_new_with_address --dialect ### -a ###
#[test]
fn test_cmd_dove_new_with_address() {
    // Project name and path
    let project_name = "project_new_with_address";

    let (base_path, project_path) = pre_start(&project_name).unwrap();

    for (dialect_name, addresses) in [
        (
            "dfinance",
            vec!["0x1", "wallet1me0cdn52672y7feddy7tgcj6j4dkzq2su745vh"],
        ),
        ("diem", vec!["0x1"]),
        (
            "pont",
            vec!["0x1", "5CdCiQzNRZXWx7wNVCVjPMzGBFpkYHe3WKrGzd6TG97vKbnv"],
        ),
    ] {
        let dialect = Dialect::from_str(dialect_name).unwrap();
        for address in addresses {
            let account_address = dialect.parse_address(address).unwrap().to_hex_literal();
            execute_dove_at(
                &[
                    "new",
                    project_name,
                    "--dialect",
                    &dialect_name,
                    "-a",
                    &format!("Account={}", &account_address),
                ],
                &base_path,
            )
            .unwrap();

            assert_eq!(
                get_project_dialect_from_toml(&project_path),
                Some(dialect_name.to_string())
            );
            assert_eq!(
                get_account_address_from_toml(&project_path),
                Some(account_address)
            );

            delete_project(&project_path).unwrap();
        }
    }
}
