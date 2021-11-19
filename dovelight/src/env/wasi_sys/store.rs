use std::fs;
use std::path::PathBuf;
use anyhow::Error;

const CACHE_DIR: &str = ".dovelight";

pub fn store(key: String, val: Vec<u8>) -> Result<(), Error> {
    let path = make_path(&key);
    if path.exists() {
        fs::remove_file(&path)?;
    }
    fs::write(path, val)?;
    Ok(())
}

pub fn load(key: String) -> Result<Option<Vec<u8>>, Error> {
    let path = make_path(&key);
    if !path.exists() {
        return Ok(None);
    } else {
        Ok(Some(fs::read(path)?))
    }
}

pub fn drop(key: String) -> Result<(), Error> {
    let path = make_path(&key);
    if path.exists() {
        fs::remove_file(&path)?;
    }
    Ok(())
}

fn make_path(key: &str) -> PathBuf {
    PathBuf::from(CACHE_DIR).join(key)
}