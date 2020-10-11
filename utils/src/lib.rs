use std::path::Path;

pub mod location;
pub mod tests;

pub type MoveFilePath = &'static str;
pub type MoveFile = (MoveFilePath, String);

pub fn leaked_fpath<P: AsRef<Path>>(path: P) -> MoveFilePath {
    let s = path.as_ref().to_str().unwrap();
    Box::leak(Box::new(s.to_owned()))
}
