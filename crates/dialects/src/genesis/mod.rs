use crate::dfinance;
use crate::dfinance::types::{AccessPath, StructTag, WriteOp, WriteSet, WriteSetMut};

pub mod serialize;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type")]
enum ResourceChangeOp {
    SetValue { values: Vec<u8> },
    Delete,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ResourceChange {
    struct_tag: StructTag,
    op: ResourceChangeOp,
}

impl ResourceChange {
    pub fn new(struct_tag: StructTag, write_op: WriteOp) -> Self {
        match write_op {
            WriteOp::Value(values) => ResourceChange {
                struct_tag,
                op: ResourceChangeOp::SetValue { values },
            },
            WriteOp::Deletion => ResourceChange {
                struct_tag,
                op: ResourceChangeOp::Delete,
            },
        }
    }

    pub fn into_write_op(self) -> (AccessPath, WriteOp) {
        let access_path = dfinance::struct_tag_into_access_path(self.struct_tag);
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
