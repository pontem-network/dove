use crate::context::Context;
use std::path::{Path, PathBuf};
use anyhow::Error;
use crate::index::meta::{source_meta, FileMeta, extract_bytecode_dependencies};
use tiny_keccak::{Sha3, Hasher};
use loader::{RestBytecodeLoader, BytecodeLoader};
use decompiler::{Config, Decompiler, unit::CompiledUnit as Unit};
use std::fs::OpenOptions;
use std::io::Write;
use move_core_types::language_storage::ModuleId;

/// Dependencies loader.
pub mod loader;

/// Cache prefix.
pub const PREFIX: &str = "chain";

/// Returns module path by its identifier.
/// Downloads a module tree if it is not in the cache.
pub fn resolve(ctx: &Context, module_id: &ModuleId) -> Result<PathBuf, Error> {
    let dep = make_path(ctx, module_id);

    if !dep.exists() {
        if let Some(chain_url) = &ctx.manifest.package.blockchain_api {
            let loader = RestBytecodeLoader::new(chain_url.parse()?);
            load_tree(ctx, &loader, module_id)?;
        } else {
            return Err(anyhow!(
                "Failed to resolve module[{}::{}]",
                module_id.address(),
                module_id.name()
            ));
        }
    }
    Ok(dep)
}

fn load_tree(
    ctx: &Context,
    loader: &RestBytecodeLoader,
    module_id: &ModuleId,
) -> Result<(), Error> {
    let bytecode = loader.load(module_id.to_owned())?;
    for import in extract_bytecode_dependencies(&bytecode)? {
        load_tree(ctx, loader, &import)?;
    }

    let config = Config {
        light_version: true,
    };
    let unit = Unit::new(&bytecode)?;
    let disasm = Decompiler::new(&unit, config);
    let source_unit = disasm.make_source_unit();
    let signature = source_unit.code_string()?;

    let mut f = OpenOptions::new()
        .create(true)
        .write(true)
        .open(make_path(ctx, module_id))?;
    f.write_all(signature.as_bytes())?;

    Ok(())
}

/// Index of chain dependencies.
pub struct ChainIndex<'a> {
    ctx: &'a Context,
    path: &'a Path,
}

impl<'a> ChainIndex<'a> {
    /// Create a new `ChainIndex` instance.
    pub fn new(ctx: &'a Context, path: &'a Path) -> ChainIndex<'a> {
        ChainIndex { ctx, path }
    }

    /// Returns all metadata of this `ChainIndex`.
    pub fn meta(&self) -> Result<Vec<FileMeta>, Error> {
        Ok(vec![source_meta(
            self.path,
            None,
            self.ctx.dialect.as_ref(),
        )?])
    }
}

fn make_path(ctx: &Context, module_id: &ModuleId) -> PathBuf {
    let deps_dir = ctx.path_for(&ctx.manifest.layout.target_deps);
    deps_dir.join(make_local_name(module_id))
}

fn make_local_name(module_name: &ModuleId) -> String {
    let mut digest = Sha3::v256();
    digest.update(module_name.name().as_bytes());
    digest.update(module_name.address().as_ref().as_ref());
    let mut output = [0; 32];
    digest.finalize(&mut output);
    format!("{}_{}", PREFIX, hex::encode(&output))
}
