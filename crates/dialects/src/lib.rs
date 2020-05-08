use std::collections::HashMap;
use std::path::Path;

pub mod dfinance;

pub type FilePath = &'static str;
pub type FilesSourceText = HashMap<&'static str, String>;

pub fn leaked_fpath<P: AsRef<Path>>(path: P) -> FilePath {
    let s = path.as_ref().to_str().unwrap();
    Box::leak(Box::new(s.to_owned()))
}
