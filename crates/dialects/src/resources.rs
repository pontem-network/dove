use crate::dfinance::{AccessPath, StatusCode, StructTag, VMStatus, WriteOp};
use libra_types::language_storage::ResourceKey;

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
        let resource_key = ResourceKey::new(self.struct_tag.address, self.struct_tag);
        let access_path = AccessPath::resource_access_path(&resource_key);
        let write_op = match self.op {
            ResourceChangeOp::Delete => WriteOp::Deletion,
            ResourceChangeOp::SetValue { values } => WriteOp::Value(values),
        };
        (access_path, write_op)
    }
}

#[derive(Debug, serde::Serialize)]
pub struct VMStatusVerbose {
    pub major_status: StatusCode,
    pub major_status_description: String,
    pub sub_status: Option<u64>,
    pub message: Option<String>,
}

impl From<VMStatus> for VMStatusVerbose {
    fn from(vm_status: VMStatus) -> Self {
        let status_desc = format!("{:?}", vm_status.major_status);
        VMStatusVerbose {
            major_status: vm_status.major_status,
            major_status_description: status_desc,
            sub_status: vm_status.sub_status,
            message: vm_status.message,
        }
    }
}
