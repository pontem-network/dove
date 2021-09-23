use move_core_types::language_storage::ModuleId;
use anyhow::Error;
use serde::ser::Serialize;
use serde::de::DeserializeOwned;

pub mod extractor;
pub mod index;
pub mod resolver;

pub trait DependencyLoader {
    fn get_module(&self, id: &ModuleId) -> Result<Vec<u8>, Error>;
}

pub trait Store {
    fn set<V: Serialize>(&self, key: &str, val: &V) -> Result<(), Error>;
    fn set_string(&self, key: &str, val: &str) -> Result<(), Error>;
    fn get<'a, V: DeserializeOwned>(&self, key: &str) -> Result<Option<V>, Error>;
    fn get_string(&self, key: &str) -> Result<Option<String>, Error>;
    fn delete(&self, key: &str) -> Result<(), Error>;
}
