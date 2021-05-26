use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::PathBuf;

use anyhow::Result;
use http::Uri;
use move_core_types::language_storage::ModuleId;
use tiny_keccak::{Hasher, Sha3};

use lang::compiler::dialects::Dialect;
use net::{make_net, Net};

/// Module loader.
pub trait BytecodeLoader {
    /// Loads module bytecode by it module id.
    fn load(&self, module_id: ModuleId) -> Result<Vec<u8>>;
}

/// Empty module loader.
/// Mock.
pub struct ZeroLoader;

impl BytecodeLoader for ZeroLoader {
    fn load(&self, module_id: ModuleId) -> Result<Vec<u8>> {
        Err(anyhow!("Module {:?} not found", module_id))
    }
}

/// Bytecode loader which loads bytecode by dnode REST api.
pub struct RestBytecodeLoader {
    net: Box<dyn Net>,
}

impl RestBytecodeLoader {
    /// Create a new `RestBytecodeLoader` with dnode api base url.
    pub fn new(dialect: &dyn Dialect, url: Uri) -> Result<RestBytecodeLoader> {
        Ok(RestBytecodeLoader {
            net: make_net(url, dialect.name())?,
        })
    }
}

impl BytecodeLoader for RestBytecodeLoader {
    fn load(&self, module_id: ModuleId) -> Result<Vec<u8>> {
        self.net
            .get_module(&module_id, &None)?
            .map(|bytes| bytes.0)
            .ok_or_else(|| anyhow!("Failed to load dependencies"))
    }
}

/// Module loader.
#[derive(Clone)]
pub struct Loader<S: BytecodeLoader> {
    cache_path: Option<PathBuf>,
    source: S,
}

impl<S> Loader<S>
where
    S: BytecodeLoader,
{
    /// Create a new module loader with cache path and external module source.
    pub fn new(cache_path: Option<PathBuf>, source: S) -> Loader<S> {
        Loader { cache_path, source }
    }

    /// Loads module by module id.
    /// Tries to load the module from the local cache.
    ///  Then tries to load the module from the external module source if the module doesn't exist in cache.
    pub fn get(&self, module_id: &ModuleId) -> Result<Vec<u8>> {
        let name = self.make_local_name(&module_id);

        if let Some(cache_path) = &self.cache_path {
            let local_path = cache_path.join(name);
            if local_path.exists() {
                let mut f = File::open(local_path)?;
                let mut bytecode = Vec::new();
                f.read_to_end(&mut bytecode)?;
                Ok(bytecode)
            } else {
                let bytecode = self.source.load(module_id.to_owned())?;
                let mut f = OpenOptions::new()
                    .create(true)
                    .write(true)
                    .open(&local_path)?;
                f.write_all(&bytecode)?;
                Ok(bytecode)
            }
        } else {
            self.source.load(module_id.to_owned())
        }
    }

    fn make_local_name(&self, module_id: &ModuleId) -> String {
        let mut digest = Sha3::v256();
        digest.update(module_id.name().as_bytes());
        digest.update(module_id.address().as_ref());
        let mut output = [0; 32];
        digest.finalize(&mut output);
        hex::encode(&output)
    }
}
