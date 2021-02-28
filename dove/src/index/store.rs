use crate::context::Context;
use anyhow::Error;
use std::fs;
use toml::Value;
use std::fs::OpenOptions;
use std::io::Write;
use serde::{Serialize, Deserialize};
use crate::index::Index;
use std::rc::Rc;
use std::collections::{HashMap, HashSet};
use diem::prelude::ModuleId;

/// Modules holder.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Modules {
    /// Modules vector.
    pub modules: Vec<Module>,
}

/// Modules references holder.
#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct ModulesRef<'a> {
    /// Vector of modules references.
    pub modules: Vec<&'a Module>,
}

/// Dependency module source type.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum SourceType {
    /// Local dependencies.
    Local,
    /// Git dependencies.
    Git,
    /// Blockchain dependencies.
    Chain,
}

/// Module model.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Module {
    /// Module address and name.
    pub name: Rc<ModuleId>,
    /// Dependency name.
    pub dep_name: Rc<str>,
    /// Path to the dependencies.
    pub path: Rc<str>,
    /// Dependency type.
    pub source_type: SourceType,
    /// Dependency dependencies.
    pub dependencies: HashSet<Rc<ModuleId>>,
}

impl<'a> Index<'a> {
    /// Load index form disk.
    pub fn load(ctx: &'a Context) -> Result<Index<'a>, Error> {
        let index_path = ctx.path_for(&ctx.manifest.layout.index);
        if index_path.exists() {
            let index = toml::from_str::<Modules>(&fs::read_to_string(index_path)?)?;

            let dep_names = index.modules.iter().map(|m| m.dep_name.clone()).collect();

            let modules = index.modules.into_iter().map(|m| (m.name.clone(), m)).fold(
                HashMap::new(),
                |mut acc, (name, m)| {
                    let entry = acc.entry(name).or_insert_with(HashMap::default);
                    entry.insert(m.source_type, m);
                    acc
                },
            );

            Ok(Index {
                modules,
                dep_names,
                ctx,
            })
        } else {
            Ok(Index {
                modules: Default::default(),
                dep_names: Default::default(),
                ctx,
            })
        }
    }

    /// Store index to the disk.
    pub fn store(&self) -> Result<(), Error> {
        let modules: Vec<&Module> = self
            .modules
            .iter()
            .map(|(_, module)| module)
            .flat_map(|m| m.values())
            .collect();

        let modules = ModulesRef { modules };

        let value = toml::to_vec(&Value::try_from(modules)?)?;

        let path = self.ctx.path_for(&self.ctx.manifest.layout.index);

        let mut f = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .open(path)?;
        f.set_len(0)?;
        f.write_all(&value)?;

        Ok(())
    }
}
