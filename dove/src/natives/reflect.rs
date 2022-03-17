use move_binary_format::errors::PartialVMResult;
use move_core_types::language_storage::TypeTag;
use move_vm_runtime::native_functions::NativeContext;
use move_vm_types::{
    loaded_data::runtime_types::Type,
    natives::function::{native_gas, NativeResult},
    values::Value,
};
use smallvec::smallvec;
use std::{collections::VecDeque, vec::Vec};
use crate::natives::PontNativeCostIndex;

const INVALID_TYPE_PARAM: u64 = 1;

pub fn mod_address_of(
    context: &mut NativeContext,
    ty_args: Vec<Type>,
    arguments: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert!(ty_args.len() == 1);
    debug_assert!(arguments.is_empty());

    let cost = native_gas(context.cost_table(), PontNativeCostIndex::MOD_ADDRESS_OF, 0);

    let type_tag = context.type_to_type_tag(&ty_args[0])?;
    if let TypeTag::Struct(struct_tag) = type_tag {
        Ok(NativeResult::ok(
            cost,
            smallvec![Value::address(struct_tag.address)],
        ))
    } else {
        Ok(NativeResult::err(cost, INVALID_TYPE_PARAM))
    }
}
