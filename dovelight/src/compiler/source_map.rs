use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct SourceMap {
    source_map: HashMap<String, String>,
}

impl SourceMap {
    pub fn get(&self, key: &str) -> Option<&str> {
        self.source_map.get(key).map(|k| k.as_str())
    }

    pub fn keys(&self) -> Vec<String> {
        self.source_map.keys().map(|k| k.to_owned()).collect()
    }
}
