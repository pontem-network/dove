use move_binary_format::file_format_common::instruction_key;
use move_core_types::account_address::AccountAddress;
use move_core_types::gas_schedule::{CostTable, GasCost};
use move_core_types::identifier::Identifier;
use move_core_types::language_storage::CORE_CODE_ADDRESS;
use move_vm_runtime::native_functions::{NativeFunction, NativeFunctionTable};
use move_vm_types::gas_schedule::{bytecode_instruction_costs, new_from_instructions};

mod account;
mod reflect;
mod signature;
mod u256;

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(u8)]
pub enum PontNativeCostIndex {
    BCS_TO_BYTES = 0,
    EMIT_EVENT = 1,
    SHA2_256 = 2,
    SHA3_256 = 3,
    SIGNER_BORROW = 4,
    LENGTH = 5,
    EMPTY = 6,
    BORROW = 7,
    BORROW_MUT = 8,
    PUSH_BACK = 9,
    POP_BACK = 10,
    DESTROY_EMPTY = 11,
    SWAP = 12,
    CREATE_SIGNER = 13,
    DESTROY_SIGNER = 14,
    ED25519_VERIFY = 15,
    ED25519_THRESHOLD_VERIFY = 16,
    ED25519_VALIDATE_KEY = 17,
    U256_FROM_U8 = 18,
    U256_FROM_U64 = 19,
    U256_FROM_U128 = 20,
    U256_AS_U8 = 21,
    U256_AS_U64 = 22,
    U256_AS_U128 = 23,
    U256_MUL = 24,
    U256_DIV = 25,
    U256_SUB = 26,
    U256_ADD = 27,
    TYPE_INFO = 28,
}

impl From<PontNativeCostIndex> for u8 {
    fn from(ind: PontNativeCostIndex) -> Self {
        ind as u8
    }
}

pub fn pontem_cost_table() -> CostTable {
    let mut instrs = bytecode_instruction_costs();
    // Note that the DiemVM is expecting the table sorted by instruction order.
    instrs.sort_by_key(|cost| instruction_key(&cost.0));

    use crate::natives::PontNativeCostIndex as N;

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
        (N::U256_FROM_U8, GasCost::new(10, 1)),
        (N::U256_FROM_U64, GasCost::new(10, 1)),
        (N::U256_FROM_U128, GasCost::new(10, 1)),
        (N::U256_AS_U8, GasCost::new(10, 1)),
        (N::U256_AS_U64, GasCost::new(10, 1)),
        (N::U256_AS_U128, GasCost::new(10, 1)),
        (N::U256_MUL, GasCost::new(10, 1)),
        (N::U256_DIV, GasCost::new(10, 1)),
        (N::U256_SUB, GasCost::new(10, 1)),
        (N::U256_ADD, GasCost::new(10, 1)),
        (N::TYPE_INFO, GasCost::new(10, 1)),
    ];
    native_table.sort_by_key(|cost| cost.0 as u64);
    let raw_native_table = native_table
        .into_iter()
        .map(|(_, cost)| cost)
        .collect::<Vec<_>>();
    new_from_instructions(instrs, raw_native_table)
}

pub fn all_natives() -> NativeFunctionTable {
    move_stdlib::natives::all_natives(CORE_CODE_ADDRESS)
        .into_iter()
        .chain(pontem_natives(CORE_CODE_ADDRESS))
        .collect()
}

pub fn pontem_natives(diem_framework_addr: AccountAddress) -> NativeFunctionTable {
    const NATIVES: &[(&str, &str, NativeFunction)] = &[
        ("U256", "from_u8", u256::from_u8),
        ("U256", "from_u64", u256::from_u64),
        ("U256", "from_u128", u256::from_u128),
        ("U256", "as_u8", u256::as_u8),
        ("U256", "as_u64", u256::as_u64),
        ("U256", "as_u128", u256::as_u128),
        ("U256", "add", u256::add),
        ("U256", "sub", u256::sub),
        ("U256", "mul", u256::mul),
        ("U256", "div", u256::div),
        ("Reflect", "type_info", reflect::type_info),
        (
            "PontAccount",
            "create_signer",
            account::native_create_signer,
        ),
        (
            "PontAccount",
            "destroy_signer",
            account::native_destroy_signer,
        ),
        (
            "Signature",
            "ed25519_validate_pubkey",
            signature::native_ed25519_publickey_validation,
        ),
        (
            "Signature",
            "ed25519_verify",
            signature::native_ed25519_signature_verification,
        ),
    ];
    NATIVES
        .iter()
        .cloned()
        .map(|(module_name, func_name, func)| {
            (
                diem_framework_addr,
                Identifier::new(module_name).unwrap(),
                Identifier::new(func_name).unwrap(),
                func,
            )
        })
        .collect()
}
