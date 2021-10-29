use crate::context::Context;
use std::path::{Path, PathBuf};
use anyhow::Error;
use tiny_keccak::{Sha3, Hasher};
use loader::{RestBytecodeLoader, BytecodeLoader};
use decompiler::{Config, Decompiler, unit::CompiledUnit as Unit};
use std::fs::OpenOptions;
use std::io::Write;
use move_core_types::language_storage::ModuleId;

/// Dependencies loader.
pub mod loader;

/// Returns module path by its identifier.
/// Downloads a module tree if it is not in the cache.
pub fn resolve(ctx: &Context, module_id: &ModuleId) -> Result<(), Error> {
    let dep = make_path(ctx, module_id)?;
    if !dep.exists() {
        if let Some(chain_url) = &ctx.manifest.package.blockchain_api {
            let loader = RestBytecodeLoader::new(ctx.dialect.as_ref(), chain_url.parse()?)?;
            load(&dep, &loader, module_id)?;
        } else {
            return Err(anyhow!(
                "Failed to resolve module[{}::{}]",
                module_id.address(),
                module_id.name()
            ));
        }
    }
    Ok(())
}

fn load(path: &Path, loader: &RestBytecodeLoader, module_id: &ModuleId) -> Result<(), Error> {
    let bytecode = loader.load(module_id.to_owned())?;
    let config = Config {
        light_version: true,
    };
    let unit = Unit::new(&bytecode)?;
    let disasm = Decompiler::new(&unit, config);
    let source_unit = disasm.make_source_unit();
    let signature = source_unit.code_string()?;

    let mut f = OpenOptions::new().create(true).write(true).open(path)?;
    f.write_all(signature.as_bytes())?;

    Ok(())
}

fn make_path(ctx: &Context, module_id: &ModuleId) -> Result<PathBuf, Error> {
    let deps_dir = ctx.path_for(&ctx.manifest.layout.chain_deps);
    if !deps_dir.exists() {
        std::fs::create_dir_all(&deps_dir)?;
    }
    Ok(deps_dir.join(make_local_name(module_id)))
}

fn make_local_name(module_name: &ModuleId) -> String {
    let mut digest = Sha3::v256();
    digest.update(module_name.name().as_bytes());
    digest.update(module_name.address().as_ref().as_ref());
    let mut output = [0; 32];
    digest.finalize(&mut output);
    format!("{}.move", hex::encode(&output))
}
