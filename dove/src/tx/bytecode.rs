use std::fs;
use std::path::{Path, PathBuf};
use anyhow::Error;
use move_binary_format::CompiledModule;
use move_binary_format::file_format::{CompiledScript, empty_module};
use move_package::compilation::package_layout::CompiledPackageLayout;
use lang::bytecode::accessor::{Bytecode, BytecodeAccess, BytecodeRef, BytecodeType};
use crate::context::Context;

/// Dove bytecode resolver.
pub struct DoveBytecode {
    path: PathBuf,
}

impl DoveBytecode {
    /// Creates a new [DoveBytecode].
    pub fn new(ctx: &Context) -> DoveBytecode {
        DoveBytecode {
            path: ctx.path_for_build(None, CompiledPackageLayout::Root),
        }
    }
}

impl BytecodeAccess for DoveBytecode {
    fn list<'a>(
        &self,
        package: Option<&'a str>,
        name: Option<&'a str>,
        tp: Option<BytecodeType>,
    ) -> Result<Vec<BytecodeRef>, Error> {
        Ok(fs::read_dir(&self.path)?
            .filter_map(|path| path.ok())
            .filter(|path| path.path().is_dir())
            .filter_map(|path| {
                if let Some(package) = package {
                    if package == path.file_name().to_string_lossy() {
                        Some(path.path())
                    } else {
                        None
                    }
                } else {
                    Some(path.path())
                }
            })
            .filter_map(|path| {
                let mut path_vec = Vec::with_capacity(2);
                fn push_if_exists(
                    base: &Path,
                    buffer: &mut Vec<(PathBuf, BytecodeType)>,
                    tp: BytecodeType,
                ) {
                    let tp_path = match tp {
                        BytecodeType::Script => CompiledPackageLayout::CompiledScripts.path(),
                        BytecodeType::Module => CompiledPackageLayout::CompiledModules.path(),
                    };

                    let path = base.join(tp_path);
                    if path.exists() {
                        buffer.push((path, tp));
                    }
                }
                match tp {
                    None => {
                        push_if_exists(&path, &mut path_vec, BytecodeType::Script);
                        push_if_exists(&path, &mut path_vec, BytecodeType::Module);
                    }
                    Some(BytecodeType::Script) => {
                        push_if_exists(&path, &mut path_vec, BytecodeType::Script);
                    }
                    Some(BytecodeType::Module) => {
                        push_if_exists(&path, &mut path_vec, BytecodeType::Module);
                    }
                }
                if path_vec.is_empty() {
                    None
                } else {
                    Some(path_vec)
                }
            })
            .flatten()
            .filter_map(|(path, tp)| {
                let dir = fs::read_dir(path)
                    .ok()?
                    .filter_map(|path| path.ok())
                    .map(|entry| entry.path())
                    .filter(|path| path.is_file())
                    .filter(|path| {
                        if let Some(ext) = path.extension() {
                            ext.to_string_lossy().as_ref() == "mv"
                        } else {
                            false
                        }
                    })
                    .filter(|path| {
                        if let Some(name) = name {
                            if let Some(file_name) = path.file_name() {
                                let file_name =
                                    &file_name.to_string_lossy()[0..file_name.len() - 3];
                                if !file_name.starts_with(name) {
                                    return false;
                                }
                                let suffix = &file_name[name.len()..];
                                for ch in suffix.chars() {
                                    if ch != '_' && !ch.is_numeric() {
                                        return false;
                                    }
                                }
                                true
                            } else {
                                false
                            }
                        } else {
                            true
                        }
                    })
                    .map(move |path| (path, tp));
                Some(dir)
            })
            .flatten()
            .map(|(path, tp)| BytecodeRef(path.to_string_lossy().to_string(), tp))
            .collect())
    }

    fn load(&self, rf: BytecodeRef) -> Result<Option<Bytecode>, Error> {
        let path: &Path = rf.0.as_ref();
        if !path.exists() {
            return Ok(None);
        }

        let bytecode = fs::read(&rf.0)?;
        Ok(Some(match rf.1 {
            BytecodeType::Script => {
                let name = path
                    .file_name()
                    .map(|name| {
                        let name = name.to_string_lossy();
                        name[0..name.len() - 3].to_string()
                    })
                    .unwrap_or_else(|| "main".to_string());
                Bytecode::Script(
                    name,
                    CompiledScript::deserialize(&bytecode)?,
                    Box::new(empty_module()),
                    rf,
                )
            }
            BytecodeType::Module => Bytecode::Module(CompiledModule::deserialize(&bytecode)?, rf),
        }))
    }
}
