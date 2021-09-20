use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct SourceMap {
    source_map: HashMap<String, String>,
}

impl SourceMap {
    pub fn insert(&mut self, name: String, content: String) {
        self.source_map.insert(name, content);
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.source_map.get(key).map(|k| k.as_str())
    }

    pub fn keys(&self) -> Vec<String> {
        self.source_map.keys().map(|k| k.to_owned()).collect()
    }
}
