use crate::compiler::deps::DependencyLoader;
use move_core_types::language_storage::ModuleId;
use anyhow::Error;

pub struct Loader {}

impl DependencyLoader for Loader {
    fn get_module(&self, _id: &ModuleId) -> Result<Vec<u8>, Error> {
        todo!()
    }
}
