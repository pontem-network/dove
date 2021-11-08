use crate::lang::deps::Store;
use serde::Serialize;
use anyhow::Error;
use serde::de::DeserializeOwned;
use crate::env::store::*;

pub struct EnvStorage {
    family: &'static str,
}

impl EnvStorage {
    pub fn new_in_family(family: &'static str) -> Result<EnvStorage, Error> {
        Ok(EnvStorage { family })
    }

    fn map_key(&self, key: &str) -> String {
        format!("{}_{}", self.family, key)
    }
}

impl Store for EnvStorage {
    fn set<V: Serialize>(&self, key: &str, val: &V) -> Result<(), Error> {
        set(self.map_key(key), val)
    }

    fn set_string(&self, key: &str, val: &str) -> Result<(), Error> {
        set_string(self.map_key(key), val)
    }

    fn get<'a, V: DeserializeOwned>(&self, key: &str) -> Result<Option<V>, Error> {
        get(self.map_key(key))
    }

    fn get_string(&self, key: &str) -> Result<Option<String>, Error> {
        get_string(self.map_key(key))
    }

    fn delete(&self, key: &str) -> Result<(), Error> {
        delete(self.map_key(key))
    }
}
