use std::sync::Arc;
use std::path::PathBuf;
use anyhow::Error;
use std::fs;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
pub struct MFile {
    pub path: Arc<PathBuf>,
    pub content: Arc<String>,
    pub last_access: u64,
    pub hash: Arc<String>,
}

impl MFile {
    pub fn load(path: Arc<PathBuf>) -> Result<MFile, Error> {
        let content = Arc::new(fs::read_to_string(path.as_ref())?);
        let last_access = fs::metadata(path.as_ref())?
            .accessed()?
            .elapsed()?
            .as_secs();
        let hash = Arc::new(hash(content.as_str()));

        Ok(MFile {
            path,
            content,
            last_access,
            hash,
        })
    }
}

fn hash(content: &str) -> String {
    let mut s = DefaultHasher::new();
    content.hash(&mut s);
    let id = s.finish();
    hex::encode(&id.to_le_bytes())
}
