use crate::deps::Store;
use serde::Serialize;
use anyhow::Error;
use web_sys::{window, Storage};
use wasm_bindgen::JsValue;
use serde::de::DeserializeOwned;

pub struct WebStorage {
    family: &'static str,
    storage: Storage,
}

impl WebStorage {
    pub fn new_in_family(family: &'static str) -> Result<WebStorage, Error> {
        let window = window().ok_or_else(|| anyhow::anyhow!("Window is expected."))?;
        let storage = window
            .local_storage()
            .map_err(|err| anyhow::anyhow!("Failed to get local storage.{:?}", err))?
            .ok_or_else(|| anyhow::anyhow!("Window is expected."))?;
        Ok(WebStorage { family, storage })
    }

    fn key(&self, key: &str) -> String {
        format!("{}_{}", self.family, key)
    }
}

impl Store for WebStorage {
    fn set<V: Serialize>(&self, key: &str, val: &V) -> Result<(), Error> {
        self.storage
            .set_item(&self.key(key), &base64::encode(bcs::to_bytes(val)?))
            .map_err(js_err)?;
        Ok(())
    }

    fn set_string(&self, key: &str, val: &str) -> Result<(), Error> {
        self.storage.set_item(&self.key(key), val).map_err(js_err)?;
        Ok(())
    }

    fn get<'a, V: DeserializeOwned>(&self, key: &str) -> Result<Option<V>, Error> {
        if let Some(val) = self.storage.get_item(&self.key(key)).map_err(js_err)? {
            Ok(Some(bcs::from_bytes(&base64::decode(val)?)?))
        } else {
            Ok(None)
        }
    }

    fn get_string(&self, key: &str) -> Result<Option<String>, Error> {
        self.storage.get_item(&self.key(key)).map_err(js_err)
    }

    fn delete(&self, key: &str) -> Result<(), Error> {
        self.storage.remove_item(&self.key(key)).map_err(js_err)
    }
}

fn js_err(val: JsValue) -> Error {
    anyhow::anyhow!("{:?}", val)
}
