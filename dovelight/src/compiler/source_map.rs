use std::collections::HashMap;
use std::iter::FromIterator;

use serde::{Serialize, Deserialize};
use move_lang::errors::FilesSourceText;

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct SourceMap {
    source_map: HashMap<String, String>,
}

impl SourceMap {
    pub fn new() -> SourceMap {
        SourceMap {
            source_map: HashMap::new(),
        }
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.source_map.get(key).map(|k| k.as_str())
    }

    pub fn keys(&self) -> Vec<String> {
        self.source_map.keys().map(|k| k.to_owned()).collect()
    }
    pub fn to_files_source_text(&self) -> FilesSourceText {
        self.source_map
            .iter()
            .map(|(k, v)| {
                let s: &'static str = Box::leak(k.clone().into_boxed_str());
                (s, v.clone())
            })
            .collect()
    }
}

impl From<HashMap<String, String>> for SourceMap {
    fn from(from: HashMap<String, String>) -> Self {
        SourceMap { source_map: from }
    }
}

impl<L: ToString> From<(L, L)> for SourceMap {
    fn from(from: (L, L)) -> Self {
        SourceMap::from(HashMap::from_iter([(
            from.0.to_string(),
            from.1.to_string(),
        )]))
    }
}
