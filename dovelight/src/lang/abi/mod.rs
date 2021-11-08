use anyhow::Error;
use move_binary_format::CompiledModule;
pub use model::ModuleAbi;
mod model;

pub fn make_module_abi(bytecode: &[u8]) -> Result<ModuleAbi, Error> {
    CompiledModule::deserialize(bytecode)
        .map(ModuleAbi::from)
        .map_err(Error::new)
}
