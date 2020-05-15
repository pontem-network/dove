use crate::dfinance::types::{AccessPath, WriteOp, WriteSet, WriteSetMut};
use libra_types::language_storage::ResourceKey;
use move_vm_types::loaded_data::types::FatStructType;

pub fn struct_type_into_access_path(struct_type: FatStructType) -> AccessPath {
    let resource_key = ResourceKey::new(struct_type.address, struct_type.struct_tag().unwrap());
    AccessPath::resource_access_path(&resource_key)
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type")]
enum ResourceChangeOp {
    SetValue { values: Vec<u8> },
    Delete,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ResourceChange {
    struct_type: FatStructType,
    op: ResourceChangeOp,
}

impl ResourceChange {
    pub fn new(struct_type: FatStructType, write_op: WriteOp) -> Self {
        match write_op {
            WriteOp::Value(values) => ResourceChange {
                struct_type,
                op: ResourceChangeOp::SetValue { values },
            },
            WriteOp::Deletion => ResourceChange {
                struct_type,
                op: ResourceChangeOp::Delete,
            },
        }
    }

    pub fn into_write_op(self) -> (AccessPath, WriteOp) {
        let access_path = struct_type_into_access_path(self.struct_type);
        let write_op = match self.op {
            ResourceChangeOp::Delete => WriteOp::Deletion,
            ResourceChangeOp::SetValue { values } => WriteOp::Value(values),
        };
        (access_path, write_op)
    }
}

pub fn changes_into_writeset(changes: Vec<ResourceChange>) -> WriteSet {
    let mut write_set = WriteSetMut::default();
    for change in changes {
        write_set.push(change.into_write_op());
    }
    write_set.freeze().unwrap()
}
