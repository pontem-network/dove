use crate::context::Context;
use crate::index::Index;
use anyhow::Error;
use std::fs;
use std::path::{Path, PathBuf};
use lang::compiler::build;
use move_lang::shared::Flags;
use move_lang::{unwrap_or_report_errors, interface_generator};
use serde::{Deserialize, Serialize};
use move_lang::compiled_unit::CompiledUnit;

/// Move modules interface builder.
pub struct InterfaceBuilder<'a> {
    ctx: &'a Context,
    index: &'a Index,
}

impl<'a> InterfaceBuilder<'a> {
    /// Creates new interface builder.
    pub fn new(ctx: &'a Context, index: &'a Index) -> InterfaceBuilder<'a> {
        InterfaceBuilder { ctx, index }
    }

    /// Build.
    pub fn build(&self) -> Result<PathBuf, Error> {
        let dir = self.ctx.interface_files_dir();
        if !dir.exists() {
            fs::create_dir_all(&dir)?;
        }

        let lock_path = self.ctx.interface_files_lock();
        let lock = read_lock(&lock_path);
        if let Some(lock) = lock {
            if lock.revision == self.index.package_hash {
                return Ok(dir);
            }
        }

        let lock = self
            .make_interfaces(&dir)
            .map_err(|err| anyhow!("Failed to generate dependencies interface: {}", err))?;
        write_lock(lock_path, &lock)?;

        Ok(dir)
    }

    fn make_interfaces(&self, dir: &Path) -> Result<InterfaceLock, Error> {
        let vm_dir = dir.join("mv");
        if vm_dir.exists() {
            fs::remove_dir_all(&vm_dir)?;
        }
        fs::create_dir_all(&vm_dir)?;
        let (files, res) = build(
            &self.index.deps_roots,
            &[],
            self.ctx.dialect.as_ref(),
            Some(self.ctx.account_address()?),
            None,
            Flags::empty(),
        )?;
        let units = unwrap_or_report_errors!(files, res);
        for (i, unit) in units.into_iter().enumerate() {
            if let CompiledUnit::Module { module, .. } = unit {
                let mut buff = Vec::new();
                module.serialize(&mut buff)?;
                let mv = vm_dir.join(format!("{}_.mv", i));
                fs::write(&mv, &buff)?;
                let (id, interface_contents) =
                    interface_generator::write_to_string(&mv.to_string_lossy())?;
                let addr_dir = dir.join(format!("{}", id.address()));
                if !addr_dir.exists() {
                    fs::create_dir_all(&addr_dir)?;
                }
                let f_name = addr_dir.join(format!("{}.move", id.name()));

                if f_name.exists() {
                    println!("In dependencies, there are several modules with the same identifier:[{}]", id);
                    continue;
                }
                fs::write(f_name, interface_contents)?;
            }
        }

        fs::remove_dir_all(vm_dir)?;

        Ok(InterfaceLock {
            revision: self.index.package_hash.to_owned(),
        })
    }
}

/// Interface lock.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Default)]
pub struct InterfaceLock {
    /// Interface revision.
    pub revision: String,
}

fn read_lock<P: AsRef<Path>>(path: P) -> Option<InterfaceLock> {
    let path = path.as_ref();
    if path.exists() {
        bcs::from_bytes(&fs::read(path).ok()?).ok()
    } else {
        None
    }
}

fn write_lock<P: AsRef<Path>>(path: P, lock: &InterfaceLock) -> Result<(), Error> {
    let path = path.as_ref();
    if path.exists() {
        fs::remove_file(path)?;
    }
    Ok(fs::write(path, &bcs::to_bytes(lock)?)?)
}
