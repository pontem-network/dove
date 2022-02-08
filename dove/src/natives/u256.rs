#![allow(clippy::ptr_offset_with_cast, clippy::assign_op_pattern)]

use core::ops::Div;
use move_binary_format::errors::{PartialVMError, PartialVMResult};
use move_core_types::vm_status::StatusCode;
use move_vm_runtime::native_functions::NativeContext;
use move_vm_types::{
    loaded_data::runtime_types::Type,
    natives::function::{native_gas, NativeResult},
    pop_arg,
    values::{values_impl::Struct, Value},
};
use smallvec::smallvec;
use std::{borrow::ToOwned, collections::VecDeque, format, vec, vec::Vec};
use crate::natives::PontNativeCostIndex;

uint::construct_uint! {
    pub struct U256(4);
}

pub fn from_u8(
    context: &mut NativeContext,
    ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert!(ty_args.is_empty());
    debug_assert!(arguments.len() == 1);

    let u256 = U256::from(pop_arg!(arguments, u8));
    let cost = native_gas(context.cost_table(), PontNativeCostIndex::U256_FROM_U8, 0);
    Ok(NativeResult::ok(cost, smallvec![wrap_u256(u256)]))
}

pub fn from_u64(
    context: &mut NativeContext,
    ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert!(ty_args.is_empty());
    debug_assert!(arguments.len() == 1);

    let u256 = U256::from(pop_arg!(arguments, u64));
    let cost = native_gas(context.cost_table(), PontNativeCostIndex::U256_FROM_U64, 0);
    Ok(NativeResult::ok(cost, smallvec![wrap_u256(u256)]))
}

pub fn from_u128(
    context: &mut NativeContext,
    ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert!(ty_args.is_empty());
    debug_assert!(arguments.len() == 1);
    let u256 = U256::from(pop_arg!(arguments, u128));
    let cost = native_gas(context.cost_table(), PontNativeCostIndex::U256_FROM_U128, 0);
    Ok(NativeResult::ok(cost, smallvec![wrap_u256(u256)]))
}

pub fn as_u8(
    context: &mut NativeContext,
    ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert!(ty_args.is_empty());
    debug_assert!(arguments.len() == 1);

    let u256 = unwrap_u256(pop_arg!(arguments, Struct))?;
    let value = if u256 > U256::from(u8::MAX) {
        Err(PartialVMError::new(StatusCode::ARITHMETIC_ERROR)
            .with_message(format!("Cannot cast u256({}) to u8", u256)))
    } else {
        Ok(u256.as_u64() as u8)
    }?;

    let cost = native_gas(context.cost_table(), PontNativeCostIndex::U256_AS_U8, 0);
    Ok(NativeResult::ok(cost, smallvec![Value::u8(value)]))
}

pub fn as_u64(
    context: &mut NativeContext,
    ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert!(ty_args.is_empty());
    debug_assert!(arguments.len() == 1);

    let u256 = unwrap_u256(pop_arg!(arguments, Struct))?;
    let value = if u256 > U256::from(u64::MAX) {
        Err(PartialVMError::new(StatusCode::ARITHMETIC_ERROR)
            .with_message(format!("Cannot cast u256({}) to u64", u256)))
    } else {
        Ok(u256.as_u64())
    }?;

    let cost = native_gas(context.cost_table(), PontNativeCostIndex::U256_AS_U64, 0);
    Ok(NativeResult::ok(cost, smallvec![Value::u64(value)]))
}

pub fn as_u128(
    context: &mut NativeContext,
    ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert!(ty_args.is_empty());
    debug_assert!(arguments.len() == 1);

    let u256 = unwrap_u256(pop_arg!(arguments, Struct))?;

    let value = if u256 > U256::from(u128::MAX) {
        Err(PartialVMError::new(StatusCode::ARITHMETIC_ERROR)
            .with_message(format!("Cannot cast u256({}) to u128", u256)))
    } else {
        Ok(u256.as_u128())
    }?;

    let cost = native_gas(context.cost_table(), PontNativeCostIndex::U256_AS_U128, 0);
    Ok(NativeResult::ok(cost, smallvec![Value::u128(value)]))
}

pub fn mul(
    context: &mut NativeContext,
    ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert!(ty_args.is_empty());
    debug_assert!(arguments.len() == 2);

    let r = unwrap_u256(pop_arg!(arguments, Struct))?;
    let l = unwrap_u256(pop_arg!(arguments, Struct))?;

    let (res, overflowed) = l.overflowing_mul(r);
    if overflowed {
        return Err(PartialVMError::new(StatusCode::ARITHMETIC_ERROR)
            .with_message(format!("Cannot mul {:?} and {:?}", l, r)));
    }

    let cost = native_gas(context.cost_table(), PontNativeCostIndex::U256_MUL, 0);
    Ok(NativeResult::ok(cost, smallvec![wrap_u256(res)]))
}

pub fn div(
    context: &mut NativeContext,
    ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert!(ty_args.is_empty());
    debug_assert!(arguments.len() == 2);

    let r = unwrap_u256(pop_arg!(arguments, Struct))?;
    let l = unwrap_u256(pop_arg!(arguments, Struct))?;

    if r == U256::zero() {
        return Err(PartialVMError::new(StatusCode::ARITHMETIC_ERROR)
            .with_message(format!("Cannot div {:?} by {:?}", l, r)));
    }

    let res = l.div(r);
    let cost = native_gas(context.cost_table(), PontNativeCostIndex::U256_DIV, 0);
    Ok(NativeResult::ok(cost, smallvec![wrap_u256(res)]))
}

pub fn sub(
    context: &mut NativeContext,
    ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert!(ty_args.is_empty());
    debug_assert!(arguments.len() == 2);

    let r = unwrap_u256(pop_arg!(arguments, Struct))?;
    let l = unwrap_u256(pop_arg!(arguments, Struct))?;

    let (res, overflowed) = l.overflowing_sub(r);
    if overflowed {
        return Err(PartialVMError::new(StatusCode::ARITHMETIC_ERROR)
            .with_message(format!("Cannot sub {:?} from {:?}", r, l)));
    }

    let cost = native_gas(context.cost_table(), PontNativeCostIndex::U256_SUB, 0);
    Ok(NativeResult::ok(cost, smallvec![wrap_u256(res)]))
}

pub fn add(
    context: &mut NativeContext,
    ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert!(ty_args.is_empty());
    debug_assert!(arguments.len() == 2);

    let r = unwrap_u256(pop_arg!(arguments, Struct))?;
    let l = unwrap_u256(pop_arg!(arguments, Struct))?;

    let (res, overflowed) = l.overflowing_add(r);
    if overflowed {
        return Err(PartialVMError::new(StatusCode::ARITHMETIC_ERROR)
            .with_message(format!("Cannot add {:?} and {:?}", l, r)));
    }

    let cost = native_gas(context.cost_table(), PontNativeCostIndex::U256_ADD, 0);
    Ok(NativeResult::ok(cost, smallvec![wrap_u256(res)]))
}

pub fn unwrap_u256(u256: Struct) -> PartialVMResult<U256> {
    u256.unpack()?
        .next()
        .ok_or_else(|| {
            PartialVMError::new(StatusCode::TYPE_MISMATCH)
                .with_sub_status(0)
                .with_message("Expected U256 struct.".to_owned())
        })
        .and_then(|field| field.value_as::<Vec<u8>>())
        .and_then(|value| {
            if value.len() != 32 {
                Err(PartialVMError::new(StatusCode::TYPE_MISMATCH)
                    .with_sub_status(1)
                    .with_message("Expected vector with length of 32.".to_owned()))
            } else {
                Ok(U256::from_little_endian(&value))
            }
        })
}

fn wrap_u256(val: U256) -> Value {
    let mut bytes = vec![0; 32];
    val.to_little_endian(&mut bytes);
    Value::struct_(Struct::pack(vec![Value::vector_u8(bytes)]))
}
