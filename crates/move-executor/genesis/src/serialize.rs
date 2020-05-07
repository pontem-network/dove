use crate::ResourceChange;
use dialects::dfinance::types::{StructTag, WriteSet};
use std::collections::HashMap;

fn get_struct_tag_at(
    path: &[u8],
    resource_structs: &HashMap<Vec<u8>, StructTag>,
) -> Option<StructTag> {
    // resource tag
    let struct_sha3 = &path[1..];
    if path[0] == 1 {
        if let Some(struct_tag) = resource_structs.get(struct_sha3) {
            return Some(struct_tag.clone());
        }
    }
    None
}

#[allow(clippy::implicit_hasher)]
pub fn serialize_write_set(
    write_set: WriteSet,
    resource_structs: &HashMap<Vec<u8>, StructTag>,
) -> Vec<ResourceChange> {
    let mut changed = vec![];
    for (access_path, write_op) in write_set {
        let struct_tag = get_struct_tag_at(&access_path.path, resource_structs);
        if let Some(struct_tag) = struct_tag {
            let change = ResourceChange::new(struct_tag, write_op);
            changed.push(change);
        }
    }
    changed
}
