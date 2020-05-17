use crate::struct_type_into_access_path;
use crate::types::{AccessPath, FatStructType, WriteOp, WriteSet, WriteSetMut};

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
