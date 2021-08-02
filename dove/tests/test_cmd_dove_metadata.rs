use dove::tests_helper::{
    execute_dove_at, project_remove, project_new_with_args, project_start, project_build,
    execute_dove_bin_at,
};
use std::io::Write;
use std::fs::{create_dir_all, File};
use serde_json::value::Value::Null;
use serde_json::Value;
use std::path::Path;

/// $ dove metadata
#[test]
fn test_cmd_dove_metadata() {
    // Project name and path
    let project_name = "project_metadata";
    let (base_folder, project_folder) = project_start(project_name);
    project_new_with_args(
        &base_folder,
        &project_folder,
        project_name,
        "pont",
        "5Csxuy81dNEVYbRA9K7tyHypu7PivHmwCZSKxcbU78Cy2v7v",
        "https://localhost/api",
    );
    project_build(&project_folder);
    execute_dove_at(&["dove", "metadata"], &project_folder).unwrap();
    project_remove(&project_folder);
}

/// $ dove metadata --validate
#[test]
fn test_cmd_dove_metadata_validate() {
    use serde_json::json;

    // Project name and path
    let project_name = "project_metadata_validate";
    let (_, project_folder) = project_start(project_name);
    create_dir_all(&project_folder).unwrap();
    // =============================================================================================
    let result = execute_dove_bin_at(
        env!("CARGO_BIN_EXE_dove"),
        &["dove", "metadata", "--validate"],
        &project_folder,
    )
    .unwrap();
    let json = serde_json::from_str::<serde_json::Value>(&result).unwrap();
    assert_eq!(json.get("code"), Some(&json!(1)));
    // =============================================================================================
    let json = validate_manifest(
        &project_folder,
        "[package]\n\
            dialect = \"pont\"\n\
            name = \"project_metadata_validate\"\n\
            \n\
            dependencies = [\n\
                { git = \"https://github.com/pontem-network/move-stdlib\", tag = \"v0.1.2\" },\n\
            ]",
    );
    assert_eq!(
        json,
        json!({
            "code":0,
            "error":Null
        })
    );
    // =============================================================================================
    let json = validate_manifest(
        &project_folder,
        "[package]\n\
                dialect = \"pont\"\n\
                name = project_metadata_validate\"\n",
    );
    assert_eq!(json.get("code"), Some(&json!(3)));
    assert_eq!(json.get("error").unwrap().get("line"), Some(&json!(2)));
    assert_eq!(json.get("error").unwrap().get("column"), Some(&json!(7)));
    assert_eq!(json.get("error").unwrap().get("offset"), Some(&json!(34)));
    // =============================================================================================
    let json = validate_manifest(
        &project_folder,
        "[package]\n\
                dialect = \"pont2\"\n\
                name = \"project_metadata_validate\"\n",
    );
    assert_eq!(json.get("code"), Some(&json!(5)));
    assert_eq!(json.get("error").unwrap().get("line"), Some(&json!(1)));
    assert_eq!(json.get("error").unwrap().get("column"), Some(&json!(10)));
    assert_eq!(json.get("error").unwrap().get("offset"), Some(&json!(20)));
    // =============================================================================================
    let json = validate_manifest(
        &project_folder,
        "[package]\n\
                dialect = \"pont\"\n\
                name = \"project_metadata_validate\"\n\
                \n\
                dependencies = [\n\
                    { path = \"/incorect_path_tmp/\" },\n\
                ]",
    );
    assert_eq!(json.get("code"), Some(&json!(6)));
    assert_eq!(json.get("error").unwrap().get("line"), Some(&json!(5)));
    assert_eq!(json.get("error").unwrap().get("column"), Some(&json!(10)));
    assert_eq!(json.get("error").unwrap().get("offset"), Some(&json!(90)));

    project_remove(&project_folder);
}

fn validate_manifest(project_folder: &Path, dove_toml_str: &str) -> Value {
    recreate_dovetoml_file(project_folder.join("Dove.toml").as_ref())
        .write_all(dove_toml_str.as_bytes())
        .unwrap();
    let result = execute_dove_bin_at(
        env!("CARGO_BIN_EXE_dove"),
        &["dove", "metadata", "--validate"],
        project_folder,
    )
    .unwrap();
    serde_json::from_str::<serde_json::Value>(&result).unwrap()
}

fn recreate_dovetoml_file(dove_toml_path: &Path) -> File {
    if dove_toml_path.exists() {
        std::fs::remove_file(dove_toml_path).unwrap();
    }
    std::fs::File::create(dove_toml_path).unwrap()
}
