use anyhow::{Context, Result};
use libra_types::{
    access_path::AccessPath,
    write_set::{WriteOp, WriteSet, WriteSetMut},
};
use move_core_types::{
    account_address::AccountAddress, identifier::Identifier, language_storage::ResourceKey,
};

use crate::shared::results::{ResourceChange, ResourceChangeOp, ResourceType};
use move_core_types::language_storage::{StructTag, TypeTag};

pub fn resource_into_access_path(
    account_address: AccountAddress,
    ty: ResourceType,
) -> Result<AccessPath> {
    let mut type_params: Vec<TypeTag> = Vec::with_capacity(ty.ty_args.len());
    for ty_arg_s in ty.ty_args {
        let quoted = format!(r#""{}""#, ty_arg_s);
        let item = serde_json::from_str::<TypeTag>(&quoted)
            .unwrap_or_else(|_| panic!("Not a valid ty_arg type {:?}", quoted));
        type_params.push(item);
    }
    let struct_tag = StructTag {
        address: AccountAddress::from_hex_literal(&ty.address)?,
        module: Identifier::new(ty.module)?,
        name: Identifier::new(ty.name)?,
        type_params,
    };
    let resource_key = ResourceKey::new(account_address, struct_tag);
    Ok(AccessPath::resource_access_path(&resource_key))
}

pub struct ResourceWriteOp(pub WriteOp);

impl Into<ResourceChangeOp> for ResourceWriteOp {
    fn into(self) -> ResourceChangeOp {
        match self.0 {
            WriteOp::Value(values) => ResourceChangeOp::SetValue { values },
            WriteOp::Deletion => ResourceChangeOp::Delete,
        }
    }
}

pub fn into_write_op(op: ResourceChangeOp) -> WriteOp {
    match op {
        ResourceChangeOp::SetValue { values } => WriteOp::Value(values),
        ResourceChangeOp::Delete => WriteOp::Deletion,
    }
}

pub fn changes_into_writeset(changes: Vec<ResourceChange>) -> Result<WriteSet> {
    let mut write_set = WriteSetMut::default();
    for ResourceChange {
        account: account_address,
        ty: resource_type,
        op: change_op,
    } in changes
    {
        // account_address here is already in Libra 0x format and validated, even for dfinance case
        let account_address = AccountAddress::from_hex_literal(&account_address)?;
        let access_path = resource_into_access_path(account_address, resource_type.clone())
            .with_context(|| {
                format!(
                    "Cannot form a valid resource AccessPath from a string {:?}",
                    &resource_type.to_string()
                )
            })?;
        let write_op = into_write_op(change_op);
        write_set.push((access_path, write_op));
    }
    Ok(write_set
        .freeze()
        .expect("WriteSetMut should always be convertible to WriteSet"))
}
