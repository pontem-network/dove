use std::collections::HashMap;

use libra_crypto::hash::CryptoHash;
use libra_types::access_path::AccessPath;
use libra_types::language_storage::{ResourceKey, StructTag};
use libra_types::vm_error::{StatusCode, VMStatus};
use libra_types::write_set::{WriteOp, WriteSet, WriteSetMut};
use move_core_types::identifier::Identifier;
use vm::access::ScriptAccess;
use vm::file_format::CompiledScript;

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

pub(crate) fn changes_into_writeset(changes: Vec<ResourceChange>) -> WriteSet {
    let mut write_set = WriteSetMut::default();
    for change in changes {
        write_set.push(change.into_write_op());
    }
    write_set.freeze().unwrap()
}

pub(crate) fn get_resource_structs(
    compiled_script: &CompiledScript,
) -> HashMap<Vec<u8>, StructTag> {
    let mut resource_structs = HashMap::new();
    for struct_handle in compiled_script.struct_handles() {
        if struct_handle.is_nominal_resource {
            let module = compiled_script.module_handle_at(struct_handle.module);
            let module_address = compiled_script.address_identifier_at(module.address);
            let module_name =
                Identifier::new(compiled_script.identifier_at(module.name).as_str()).unwrap();
            let struct_name =
                Identifier::new(compiled_script.identifier_at(struct_handle.name).as_str())
                    .unwrap();
            let struct_tag = StructTag {
                address: *module_address,
                module: module_name,
                name: struct_name,
                type_params: vec![],
            };
            resource_structs.insert(struct_tag.hash().to_vec(), struct_tag);
        }
    }
    resource_structs
}

fn get_struct_tag_at(
    access_path: AccessPath,
    resource_structs: &HashMap<Vec<u8>, StructTag>,
) -> Option<StructTag> {
    // resource tag
    let path = access_path.path;
    let struct_sha3 = &path[1..];
    if path[0] == 1 {
        if let Some(struct_tag) = resource_structs.get(struct_sha3) {
            return Some(struct_tag.clone());
        }
    }
    None
}

pub(crate) fn serialize_write_set(
    write_set: WriteSet,
    resource_structs: &HashMap<Vec<u8>, StructTag>,
) -> Vec<ResourceChange> {
    let mut changed = vec![];
    for (access_path, write_op) in write_set {
        let struct_tag = get_struct_tag_at(access_path, resource_structs);
        if let Some(struct_tag) = struct_tag {
            let change = ResourceChange::new(struct_tag, write_op);
            changed.push(change);
        }
    }
    changed
}

#[derive(Debug, serde::Serialize)]
pub(crate) struct VMStatusVerbose {
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
