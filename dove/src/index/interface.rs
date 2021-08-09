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
use move_binary_format::CompiledModule;

/// Dependencies interface.
pub struct Interface {
    /// Path to interface directory.
    pub dir: PathBuf,
    /// Path to dependencies bytecode.
    pub vm_dir: PathBuf,
}

impl Interface {
    /// Loads dependencies bytecode from disk.
    pub fn load_mv(&self) -> Result<Vec<CompiledModule>, Error> {
        fs::read_dir(&self.vm_dir)?
            .map(|dir| {
                dir.map_err(Error::new)
                    .map(|dir| dir.path())
                    .and_then(|path| {
                        Ok(if path.is_file() {
                            Some(CompiledModule::deserialize(&fs::read(path)?)?)
                        } else {
                            None
                        })
                    })
            })
            .filter_map(|mv| match mv {
                Ok(Some(mv)) => Some(Ok(mv)),
                Ok(None) => None,
                Err(err) => Some(Err(err)),
            })
            .collect()
    }
}

/// Move modules interface builder.
/// The builder interface allows you to convert the source code in the move language into
/// a minimalistic representation of this module.
/// The conversion process:
///     1) Build.
///     2) Decompile the module interface.
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
    pub fn build(&self) -> Result<Interface, Error> {
        let interface_dir = self.ctx.interface_files_dir();
        let mv_dir = self.ctx.deps_mv_dir();

        let lock_path = self.ctx.interface_files_lock();
        let lock = read_lock(&lock_path);
        if let Some(lock) = lock {
            if lock.revision == self.index.package_hash {
                return Ok(Interface {
                    dir: interface_dir,
                    vm_dir: mv_dir,
                });
            }
        }

        if interface_dir.exists() {
            fs::remove_dir_all(&interface_dir)?;
        }
        fs::create_dir_all(&interface_dir)?;

        if mv_dir.exists() {
            fs::remove_dir_all(&mv_dir)?;
        }
        fs::create_dir_all(&mv_dir)?;

        let lock = self
            .make_interfaces(&interface_dir, &mv_dir)
            .map_err(|err| anyhow!("Failed to generate dependencies interface: {}", err))?;
        write_lock(lock_path, &lock)?;

        Ok(Interface {
            dir: interface_dir,
            vm_dir: mv_dir,
        })
    }

    fn make_interfaces(&self, dir: &Path, mv_dir: &Path) -> Result<InterfaceLock, Error> {
        let (files, res) = build(
            &self.index.deps_roots,
            &[],
            self.ctx.dialect.as_ref(),
            &self.ctx.account_address_str()?,
            None,
            Flags::empty(),
        )?;
        let units = unwrap_or_report_errors!(files, res);
        for (i, unit) in units.into_iter().enumerate() {
            if let CompiledUnit::Module { module, .. } = unit {
                let mut buff = Vec::new();
                module.serialize(&mut buff)?;
                let mv = mv_dir.join(format!("{}_.mv", i));
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
