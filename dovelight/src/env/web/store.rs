use anyhow::Error;
use web_sys::window;
use web_sys::Storage;
use crate::env::web::js_err;

pub fn store(key: String, val: Vec<u8>) -> Result<(), Error> {
    let storage = get_store()?;
    storage.set_item(&key, &hex::encode(&val)).map_err(js_err)
}

pub fn load(key: String) -> Result<Option<Vec<u8>>, Error> {
    let storage = get_store()?;
    Ok(if let Some(val) = storage.get_item(&key).map_err(js_err)? {
        Some(hex::decode(val)?)
    } else {
        None
    })
}

pub fn drop(key: String) -> Result<(), Error> {
    let storage = get_store()?;
    storage.remove_item(&key).map_err(js_err)
}

fn get_store() -> Result<Storage, Error> {
    let window = window().ok_or_else(|| anyhow::anyhow!("Window is expected."))?;
    window
        .local_storage()
        .map_err(|err| anyhow::anyhow!("Failed to get local storage.{:?}", err))?
        .ok_or_else(|| anyhow::anyhow!("Window is expected."))
}
