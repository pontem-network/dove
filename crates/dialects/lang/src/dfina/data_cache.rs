use dfin_libra_types::access_path::AccessPath;
use dfin_libra_types::contract_event::ContractEvent;

use dfin_move_core_types::language_storage::ModuleId;

use dfin_move_vm_runtime::data_cache::{RemoteCache, TransactionDataCache};

use dfin_move_vm_types::data_store::DataStore;
use dfin_move_vm_types::loaded_data::types::FatStructType;

use dfin_move_vm_types::values::GlobalValue;
use dfin_vm::errors::VMResult;
use std::collections::HashMap;

pub struct DataCache<'txn> {
    inner: TransactionDataCache<'txn>,
    ap_to_struct_type: HashMap<AccessPath, FatStructType>,
}

impl<'txn> DataCache<'txn> {
    pub fn new(data_cache: &'txn dyn RemoteCache) -> Self {
        DataCache {
            inner: TransactionDataCache::new(data_cache),
            ap_to_struct_type: HashMap::new(),
        }
    }

    pub fn resource_changes(self) -> Vec<(FatStructType, Option<GlobalValue>)> {
        let mut resources = vec![];
        for (ap, change) in self.inner.data_map {
            match change {
                None => {
                    let ty = self
                        .ap_to_struct_type.get(&ap)
                        .expect("AccessPath should have been added to the mapping in move_resource_from() at execution time");
                    resources.push((ty.to_owned(), None));
                }
                Some((ty, val)) => {
                    resources.push((ty, Some(val)));
                }
            }
        }
        resources
    }
}

impl<'txn> DataStore for DataCache<'txn> {
    fn publish_resource(
        &mut self,
        ap: &AccessPath,
        g: (FatStructType, GlobalValue),
    ) -> VMResult<()> {
        self.inner.publish_resource(ap, g)
    }

    fn borrow_resource(
        &mut self,
        ap: &AccessPath,
        ty: &FatStructType,
    ) -> VMResult<Option<&GlobalValue>> {
        self.inner.borrow_resource(ap, ty)
    }

    fn move_resource_from(
        &mut self,
        ap: &AccessPath,
        ty: &FatStructType,
    ) -> VMResult<Option<GlobalValue>> {
        self.ap_to_struct_type.insert(ap.clone(), ty.clone());
        self.inner.move_resource_from(ap, ty)
    }

    fn load_module(&self, module: &ModuleId) -> VMResult<Vec<u8>> {
        self.inner.load_module(module)
    }

    fn publish_module(&mut self, module_id: ModuleId, module: Vec<u8>) -> VMResult<()> {
        self.inner.publish_module(module_id, module)
    }

    fn exists_module(&self, key: &ModuleId) -> bool {
        self.inner.exists_module(key)
    }

    fn emit_event(&mut self, event: ContractEvent) {
        self.inner.emit_event(event)
    }
}
