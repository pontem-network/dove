use std::process::Command;

fn main() {
    run("./client", "sh", &["./build.sh"]);
}

pub fn run(path: &str, cmd: &str, args: &[&str]) {
    let out = Command::new(cmd)
        .current_dir(path)
        .args(args)
        .output()
        .unwrap();
    if !out.status.success() {
        println!(
            "Code:{}\nError:\n{}\n",
            out.status.code().unwrap_or_default(),
            String::from_utf8(out.stderr).unwrap_or_default()
        );
        panic!("Failed to run {} {} {:?}", path, cmd, args);
    }
}
