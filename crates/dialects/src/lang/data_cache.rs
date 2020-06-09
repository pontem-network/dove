use libra_types::access_path::AccessPath;
use libra_types::contract_event::ContractEvent;
use libra_types::vm_error::StatusCode;
use libra_types::write_set::WriteOp;

use move_core_types::language_storage::ModuleId;

use move_vm_runtime::data_cache::{RemoteCache, TransactionDataCache};

use move_vm_types::data_store::DataStore;
use move_vm_types::loaded_data::types::FatStructType;

use crate::lang::resources::{ResourceStructType, ResourceWriteOp};

use crate::shared::results::ResourceChange;
use move_vm_types::values::GlobalValue;
use std::collections::HashMap;
use std::ops::Deref;
use vm::errors::{vm_error, Location, VMResult};

fn convert_set_value(struct_type: &FatStructType, val: GlobalValue) -> VMResult<Vec<u8>> {
    // into_owned_struct will check if all references are properly released at the end of a transaction
    let data = val.into_owned_struct()?;
    match data.simple_serialize(struct_type) {
        Some(blob) => Ok(blob),
        None => Err(vm_error(
            Location::new(),
            StatusCode::VALUE_SERIALIZATION_ERROR,
        )),
    }
}

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

    pub fn resource_changes(self) -> VMResult<Vec<ResourceChange>> {
        let mut resources = vec![];
        for (ap, change) in self.inner.data_map {
            let account_address = format!("0x{}", ap.address.to_string());
            match change {
                None => {
                    let ty = self
                        .ap_to_struct_type.get(&ap)
                        .expect("AccessPath should have been added to the mapping in move_resource_from() at execution time");
                    resources.push(ResourceChange::new(
                        account_address,
                        ResourceStructType(ty.to_owned()),
                        ResourceWriteOp(WriteOp::Deletion),
                    ));
                }
                Some((ty, val)) => {
                    if !val.is_clean().unwrap() {
                        let val = convert_set_value(&ty, val)?;
                        resources.push(ResourceChange::new(
                            account_address,
                            ResourceStructType(ty),
                            ResourceWriteOp(WriteOp::Value(val)),
                        ));
                    }
                }
            }
        }
        Ok(resources)
    }

    pub fn events(&self) -> Vec<serde_json::Value> {
        self.inner
            .event_data()
            .iter()
            .map(|event| serde_json::to_value(event.deref()).unwrap())
            .collect()
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
