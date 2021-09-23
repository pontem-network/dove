use anyhow::Error;
use move_core_types::language_storage::ModuleId;
use std::collections::{HashSet};
use crate::compiler::deps::{DependencyLoader, Store};
use crate::compiler::deps::index::{Index, str_to_id};
use move_lang::interface_generator::make_interface;
use crate::compiler::deps::extractor::extract_source_deps;

const INDEX_KEY: &str = "dependency_index";

pub struct DependencyResolver<L: DependencyLoader, S: Store> {
    loader: L,
    store: S,
}

impl<L: DependencyLoader, S: Store> DependencyResolver<L, S> {
    pub fn new(loader: L, store: S) -> DependencyResolver<L, S> {
        DependencyResolver { loader, store }
    }

    pub fn load_interface(&self, name: &str) -> Result<(ModuleId, String), Error> {
        let id = str_to_id(name)?;
        if let Some(interface) = self.store.get_string(name)? {
            return Ok((id, interface));
        }
        let bytecode = self.loader.get_module(&id)?;
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
