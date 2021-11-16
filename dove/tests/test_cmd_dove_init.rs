mod helper;

use std::str::FromStr;
use crate::helper::{
    pre_start_dove_init, execute_dove_at, delete_project, get_project_name_from_toml,
    get_project_dialect_from_toml, assert_basic_project_dirs_exist,
    get_account_address_from_toml,
};
use dialect::Dialect;

/// Creating a default project without additional parameters
/// $ dove init project_init_without_arguments
#[test]
fn test_cmd_dove_init_without_arguments() {
    // Project name and path
    let project_name = "project_init_without_arguments";
    let project_path = pre_start_dove_init(&project_name).unwrap();

    execute_dove_at(&["init"], &project_path).unwrap();

    assert_eq!(
        get_project_name_from_toml(&project_path),
        Some(project_name.to_string())
    );
    assert_eq!(get_project_dialect_from_toml(&project_path), None);
    assert!(assert_basic_project_dirs_exist(&project_path).is_ok());

    delete_project(&project_path).unwrap();
}

/// Checking the "minimal" parameter
/// $ dove init project_init_with_minimal --minimal
#[test]
fn test_cmd_dove_init_with_minimal() {
    // Project name and path
    let project_name = "project_init_with_minimal";
    let project_path = pre_start_dove_init(&project_name).unwrap();

    execute_dove_at(&["init", "--minimal"], &project_path).unwrap();
    assert!(assert_basic_project_dirs_exist(&project_path).is_err());

    delete_project(&project_path).unwrap();
}

/// Creating a project with different dialects. Dialects: "pont", "diem", "dfinance"
/// $ dove init project_init_with_dialect --dialect ###
#[test]
fn test_cmd_dove_init_with_dialect() {
    // Project name and path
    let project_name = "project_init_with_dialect";
    let project_path = pre_start_dove_init(&project_name).unwrap();

    for dialect_name in ["pont", "diem", "dfinance"] {
        execute_dove_at(&["init", "--dialect", dialect_name], &project_path).unwrap();
        assert_eq!(
            get_project_dialect_from_toml(&project_path),
            Some(dialect_name.to_string())
        );
        pre_start_dove_init(&project_name).unwrap();
    }
}

/// Creating a project with a non-existent dialect
/// $ dove init project_init_with_nonexistent_dialect --dialect noname
#[test]
fn test_cmd_dove_init_with_nonexistent_dialect() {
    // Project name and path
    let project_name = "project_init_with_nonexistent_dialect";
    let project_path = pre_start_dove_init(&project_name).unwrap();

    assert!(execute_dove_at(&["init", "--dialect", "noname"], &project_path).is_err());
}

/// Creating a project with an address
/// $ dove init project_init_with_address --dialect ### -a ###
#[test]
fn test_cmd_dove_init_with_address() {
    // Project name and path
    let project_name = "project_init_with_address";

    let project_path = pre_start_dove_init(&project_name).unwrap();

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
                    "init",
                    "--dialect",
                    &dialect_name,
                    "-a",
                    &format!("Account={}", &account_address),
                ],
                &project_path,
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

            pre_start_dove_init(&project_name).unwrap();
        }
    }
}
