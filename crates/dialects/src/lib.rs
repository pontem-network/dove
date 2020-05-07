use std::collections::HashMap;

pub mod dfinance;

pub type FilePath = &'static str;
pub type FilesSourceText = HashMap<&'static str, String>;
