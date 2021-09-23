use anyhow::Error;
use move_core_types::language_storage::ModuleId;
use std::collections::{HashSet};
use crate::deps::{DependencyLoader, Store};
use crate::deps::index::{Index, str_to_id};
use move_lang::interface_generator::make_interface;
use crate::deps::extractor::extract_source_deps;
use lang::compiler::dialects::Dialect;

const INDEX_KEY: &str = "dependency_index";

pub struct DependencyResolver<'a, L: DependencyLoader, S: Store> {
    loader: L,
    store: S,
    dialect: &'a dyn Dialect,
}

impl<'a, L: DependencyLoader, S: Store> DependencyResolver<'a, L, S> {
    pub fn new(dialect: &'a dyn Dialect, loader: L, store: S) -> DependencyResolver<L, S> {
        DependencyResolver { loader, store, dialect }
    }

    pub fn load_interface(&self, name: &str) -> Result<(ModuleId, String), Error> {
        let id = str_to_id(name)?;
        if let Some(interface) = self.store.get_string(name)? {
            return Ok((id, interface));
        }
        let mut bytecode = self.loader.get_module(&id)?;
        self.dialect.adapt_to_basis(&mut bytecode)?;
        self.store.set(&format!("bytecode_{}", name), &bytecode)?;
        let (_, interface) = make_interface(name, &bytecode)?;
        self.store.set_string(name, &interface)?;
        Ok((id, interface))
    }

    fn load(&self, index: &mut Index, id: &ModuleId) -> Result<(), Error> {
        if !index.contains(&id) {
            let (id, interface) = self.load_interface(&id.to_string())?;
            let deps = extract_source_deps(&interface)?;
            index.insert(id, deps.clone());

            for id in &deps {
                self.load(index, id)?;
            }
        }
        Ok(())
    }

    pub fn load_tree(&self, root_set: HashSet<ModuleId>) -> Result<Vec<String>, Error> {
        let mut index: Index = self.store.get(INDEX_KEY)?.unwrap_or_default();

        for root in root_set {
            self.load(&mut index, &root)?;
        }

        self.store.set(INDEX_KEY, &index)?;
        Ok(index.all_deps())
    }
}
