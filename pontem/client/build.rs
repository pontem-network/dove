use std::path::PathBuf;

fn main() {
    println!(r#"Building: pontemapi"#);

    let lib_path = PathBuf::from("../pontemapi").canonicalize().unwrap();
    let result = std::process::Command::new("cargo")
        .args(["build", "--release"])
        .current_dir(&lib_path)
        .output()
        .unwrap();

    if result.status.code() != Some(0) {
        let error = String::from_utf8(result.stderr).unwrap_or_default();
        println!("Error: {}", error);
        panic!(r#"Failed to create "pontemapi" library"#);
    }
    println!(r#"Building of the "pontempi" library is completed"#)
}
