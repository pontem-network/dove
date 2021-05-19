use anyhow::Result;
use crate::compiler::dialects::{Dialect, DialectName};
use move_core_types::gas_schedule::{CostTable};
use crate::compiler::source_map::FileOffsetMap;
use std::ops::Deref;
use move_core_types::account_address::AccountAddress;

#[derive(Default)]
pub struct DiemDialect;

impl Dialect for DiemDialect {
    fn address_length(&self) -> usize {
        16
    }

    fn name(&self) -> DialectName {
        DialectName::Diem
    }

    fn adapt_to_target(&self, bytecode: &mut Vec<u8>) -> Result<()> {
        compat::adapt_from_basis(bytecode, compat::AddressType::Diem)
    }

    fn adapt_to_basis(&self, bytecode: &mut Vec<u8>) -> Result<()> {
        compat::adapt_to_basis(bytecode, compat::AddressType::Diem)
    }

    fn adapt_address_to_target(&self, address: AccountAddress) -> Vec<u8> {
        compat::adapt_address_to_target(address, compat::AddressType::Diem)
    }

    fn adapt_address_to_basis(&self, address: &[u8]) -> Result<AccountAddress> {
        compat::adapt_address_to_basis(address, compat::AddressType::Diem)
    }

    fn parse_address(&self, addr: &str) -> Result<AccountAddress> {
        let max_hex_len = self.address_length() * 2 + 2;
        if addr.len() > max_hex_len {
            return Err(anyhow::anyhow!(
                "Unable to parse AccountAddress. Maximum address length is {}. Actual {}",
                max_hex_len,
                addr
            ));
        }

        Ok(AccountAddress::from_hex_literal(&addr)?)
    }

    fn cost_table(&self) -> CostTable {
        INITIAL_GAS_SCHEDULE.deref().clone()
    }

    fn replace_addresses(&self, _: &str, _: &mut MutString, _: &mut FileOffsetMap) {
        // No-op
    }
}

use once_cell::sync::Lazy;
use move_core_types::gas_schedule::GasCost;
use crate::compiler::mut_string::MutString;

pub static INITIAL_GAS_SCHEDULE: Lazy<CostTable> = Lazy::new(|| {
    use move_vm_types::gas_schedule::{self, NativeCostIndex as N};
    use vm::{
        file_format::{
            Bytecode, ConstantPoolIndex, FieldHandleIndex, FieldInstantiationIndex,
            FunctionHandleIndex, FunctionInstantiationIndex, StructDefInstantiationIndex,
            StructDefinitionIndex,
        },
        file_format_common::instruction_key,
    };
    use Bytecode::*;
    let mut instrs = vec![
        (MoveTo(StructDefinitionIndex::new(0)), GasCost::new(13, 1)),
        (
            MoveToGeneric(StructDefInstantiationIndex::new(0)),
            GasCost::new(27, 1),
        ),
        (
            MoveFrom(StructDefinitionIndex::new(0)),
            GasCost::new(459, 1),
        ),
        (
            MoveFromGeneric(StructDefInstantiationIndex::new(0)),
            GasCost::new(13, 1),
        ),
        (BrTrue(0), GasCost::new(1, 1)),
        (WriteRef, GasCost::new(1, 1)),
        (Mul, GasCost::new(1, 1)),
        (MoveLoc(0), GasCost::new(1, 1)),
        (And, GasCost::new(1, 1)),
        (Pop, GasCost::new(1, 1)),
        (BitAnd, GasCost::new(2, 1)),
        (ReadRef, GasCost::new(1, 1)),
        (Sub, GasCost::new(1, 1)),
        (MutBorrowField(FieldHandleIndex::new(0)), GasCost::new(1, 1)),
        (
            MutBorrowFieldGeneric(FieldInstantiationIndex::new(0)),
            GasCost::new(1, 1),
        ),
        (ImmBorrowField(FieldHandleIndex::new(0)), GasCost::new(1, 1)),
        (
            ImmBorrowFieldGeneric(FieldInstantiationIndex::new(0)),
            GasCost::new(1, 1),
        ),
        (Add, GasCost::new(1, 1)),
        (CopyLoc(0), GasCost::new(1, 1)),
        (StLoc(0), GasCost::new(1, 1)),
        (Ret, GasCost::new(638, 1)),
        (Lt, GasCost::new(1, 1)),
        (LdU8(0), GasCost::new(1, 1)),
        (LdU64(0), GasCost::new(1, 1)),
        (LdU128(0), GasCost::new(1, 1)),
        (CastU8, GasCost::new(2, 1)),
        (CastU64, GasCost::new(1, 1)),
        (CastU128, GasCost::new(1, 1)),
        (Abort, GasCost::new(1, 1)),
        (MutBorrowLoc(0), GasCost::new(2, 1)),
        (ImmBorrowLoc(0), GasCost::new(1, 1)),
        (LdConst(ConstantPoolIndex::new(0)), GasCost::new(1, 1)),
        (Ge, GasCost::new(1, 1)),
        (Xor, GasCost::new(1, 1)),
        (Shl, GasCost::new(2, 1)),
        (Shr, GasCost::new(1, 1)),
        (Neq, GasCost::new(1, 1)),
        (Not, GasCost::new(1, 1)),
        (Call(FunctionHandleIndex::new(0)), GasCost::new(1132, 1)),
        (
            CallGeneric(FunctionInstantiationIndex::new(0)),
            GasCost::new(582, 1),
        ),
        (Le, GasCost::new(2, 1)),
        (Branch(0), GasCost::new(1, 1)),
        (Unpack(StructDefinitionIndex::new(0)), GasCost::new(2, 1)),
        (
            UnpackGeneric(StructDefInstantiationIndex::new(0)),
            GasCost::new(2, 1),
        ),
        (Or, GasCost::new(2, 1)),
        (LdFalse, GasCost::new(1, 1)),
        (LdTrue, GasCost::new(1, 1)),
        (Mod, GasCost::new(1, 1)),
        (BrFalse(0), GasCost::new(1, 1)),
        (Exists(StructDefinitionIndex::new(0)), GasCost::new(41, 1)),
        (
            ExistsGeneric(StructDefInstantiationIndex::new(0)),
            GasCost::new(34, 1),
        ),
        (BitOr, GasCost::new(2, 1)),
        (FreezeRef, GasCost::new(1, 1)),
        (
            MutBorrowGlobal(StructDefinitionIndex::new(0)),
            GasCost::new(21, 1),
        ),
        (
            MutBorrowGlobalGeneric(StructDefInstantiationIndex::new(0)),
            GasCost::new(15, 1),
        ),
        (
            ImmBorrowGlobal(StructDefinitionIndex::new(0)),
            GasCost::new(23, 1),
        ),
        (
            ImmBorrowGlobalGeneric(StructDefInstantiationIndex::new(0)),
            GasCost::new(14, 1),
        ),
        (Div, GasCost::new(3, 1)),
        (Eq, GasCost::new(1, 1)),
        (Gt, GasCost::new(1, 1)),
        (Pack(StructDefinitionIndex::new(0)), GasCost::new(2, 1)),
        (
            PackGeneric(StructDefInstantiationIndex::new(0)),
            GasCost::new(2, 1),
        ),
        (Nop, GasCost::new(1, 1)),
    ];
    // Note that the DiemVM is expecting the table sorted by instruction order.
    instrs.sort_by_key(|cost| instruction_key(&cost.0));

    let mut native_table = vec![
        (N::SHA2_256, GasCost::new(21, 1)),
        (N::SHA3_256, GasCost::new(64, 1)),
        (N::ED25519_VERIFY, GasCost::new(61, 1)),
        (N::ED25519_THRESHOLD_VERIFY, GasCost::new(3351, 1)),
        (N::BCS_TO_BYTES, GasCost::new(181, 1)),
        (N::LENGTH, GasCost::new(98, 1)),
        (N::EMPTY, GasCost::new(84, 1)),
        (N::BORROW, GasCost::new(1334, 1)),
        (N::BORROW_MUT, GasCost::new(1902, 1)),
        (N::PUSH_BACK, GasCost::new(53, 1)),
        (N::POP_BACK, GasCost::new(227, 1)),
        (N::DESTROY_EMPTY, GasCost::new(572, 1)),
        (N::SWAP, GasCost::new(1436, 1)),
        (N::ED25519_VALIDATE_KEY, GasCost::new(26, 1)),
        (N::SIGNER_BORROW, GasCost::new(353, 1)),
        (N::CREATE_SIGNER, GasCost::new(24, 1)),
        (N::DESTROY_SIGNER, GasCost::new(212, 1)),
        (N::EMIT_EVENT, GasCost::new(52, 1)),
    ];
    native_table.sort_by_key(|cost| cost.0 as u64);
    let raw_native_table = native_table
        .into_iter()
        .map(|(_, cost)| cost)
        .collect::<Vec<_>>();
    gas_schedule::new_from_instructions(instrs, raw_native_table)
});
