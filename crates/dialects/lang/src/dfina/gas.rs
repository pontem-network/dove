use dfin_move_core_types::gas_schedule::{CostTable, GasCost};
use dfin_move_vm_types::gas_schedule::new_from_instructions;
use dfin_vm::{
    file_format::{
        ConstantPoolIndex, FieldHandleIndex, FieldInstantiationIndex, FunctionHandleIndex,
        FunctionInstantiationIndex, StructDefInstantiationIndex, StructDefinitionIndex,
        NUMBER_OF_NATIVE_FUNCTIONS,
    },
    file_format_common::instruction_key,
};

pub fn fetch_cost_table() -> CostTable {
    use dfin_vm::file_format::Bytecode::*;

    let mut instrs = vec![
        (
            MoveToSender(StructDefinitionIndex::new(0)),
            GasCost::new(774, 1),
        ),
        (
            MoveToSenderGeneric(StructDefInstantiationIndex::new(0)),
            GasCost::new(774, 1),
        ),
        (
            MoveTo(StructDefinitionIndex::new(0)),
            /* MoveToSender + ReadRef == 774 + 51 == 825 */
            GasCost::new(825, 1),
        ),
        (
            MoveToGeneric(StructDefInstantiationIndex::new(0)),
            /* MoveToSender + ReadRef == 774 + 51 == 825 */
            GasCost::new(825, 1),
        ),
        (GetTxnSenderAddress, GasCost::new(30, 1)),
        (
            MoveFrom(StructDefinitionIndex::new(0)),
            GasCost::new(917, 1),
        ),
        (
            MoveFromGeneric(StructDefInstantiationIndex::new(0)),
            GasCost::new(917, 1),
        ),
        (BrTrue(0), GasCost::new(31, 1)),
        (WriteRef, GasCost::new(65, 1)),
        (Mul, GasCost::new(41, 1)),
        (MoveLoc(0), GasCost::new(41, 1)),
        (And, GasCost::new(49, 1)),
        (Pop, GasCost::new(27, 1)),
        (BitAnd, GasCost::new(44, 1)),
        (ReadRef, GasCost::new(51, 1)),
        (Sub, GasCost::new(44, 1)),
        (
            MutBorrowField(FieldHandleIndex::new(0)),
            GasCost::new(58, 1),
        ),
        (
            MutBorrowFieldGeneric(FieldInstantiationIndex::new(0)),
            GasCost::new(58, 1),
        ),
        (
            ImmBorrowField(FieldHandleIndex::new(0)),
            GasCost::new(58, 1),
        ),
        (
            ImmBorrowFieldGeneric(FieldInstantiationIndex::new(0)),
            GasCost::new(58, 1),
        ),
        (Add, GasCost::new(45, 1)),
        (CopyLoc(0), GasCost::new(41, 1)),
        (StLoc(0), GasCost::new(28, 1)),
        (Ret, GasCost::new(28, 1)),
        (Lt, GasCost::new(49, 1)),
        (LdU8(0), GasCost::new(29, 1)),
        (LdU64(0), GasCost::new(29, 1)),
        (LdU128(0), GasCost::new(29, 1)),
        (CastU8, GasCost::new(29, 1)),
        (CastU64, GasCost::new(29, 1)),
        (CastU128, GasCost::new(29, 1)),
        (Abort, GasCost::new(39, 1)),
        (MutBorrowLoc(0), GasCost::new(45, 1)),
        (ImmBorrowLoc(0), GasCost::new(45, 1)),
        (LdConst(ConstantPoolIndex::new(0)), GasCost::new(36, 1)),
        (Ge, GasCost::new(46, 1)),
        (Xor, GasCost::new(46, 1)),
        (Shl, GasCost::new(46, 1)),
        (Shr, GasCost::new(46, 1)),
        (Neq, GasCost::new(51, 1)),
        (Not, GasCost::new(35, 1)),
        (Call(FunctionHandleIndex::new(0)), GasCost::new(197, 1)),
        (
            CallGeneric(FunctionInstantiationIndex::new(0)),
            GasCost::new(197, 1),
        ),
        (Le, GasCost::new(47, 1)),
        (Branch(0), GasCost::new(10, 1)),
        (Unpack(StructDefinitionIndex::new(0)), GasCost::new(94, 1)),
        (
            UnpackGeneric(StructDefInstantiationIndex::new(0)),
            GasCost::new(94, 1),
        ),
        (Or, GasCost::new(43, 1)),
        (LdFalse, GasCost::new(30, 1)),
        (LdTrue, GasCost::new(29, 1)),
        (Mod, GasCost::new(42, 1)),
        (BrFalse(0), GasCost::new(29, 1)),
        (Exists(StructDefinitionIndex::new(0)), GasCost::new(856, 1)),
        (
            ExistsGeneric(StructDefInstantiationIndex::new(0)),
            GasCost::new(856, 1),
        ),
        (BitOr, GasCost::new(45, 1)),
        (FreezeRef, GasCost::new(10, 1)),
        (
            MutBorrowGlobal(StructDefinitionIndex::new(0)),
            GasCost::new(1000, 3),
        ),
        (
            MutBorrowGlobalGeneric(StructDefInstantiationIndex::new(0)),
            GasCost::new(1000, 3),
        ),
        (
            ImmBorrowGlobal(StructDefinitionIndex::new(0)),
            GasCost::new(1000, 3),
        ),
        (
            ImmBorrowGlobalGeneric(StructDefInstantiationIndex::new(0)),
            GasCost::new(1000, 3),
        ),
        (Div, GasCost::new(41, 1)),
        (Eq, GasCost::new(48, 1)),
        (Gt, GasCost::new(46, 1)),
        (Pack(StructDefinitionIndex::new(0)), GasCost::new(73, 1)),
        (
            PackGeneric(StructDefInstantiationIndex::new(0)),
            GasCost::new(73, 1),
        ),
        (Nop, GasCost::new(10, 1)),
    ];
    instrs.sort_by_key(|cost| instruction_key(&cost.0));
    let native_table = (0..NUMBER_OF_NATIVE_FUNCTIONS)
        .map(|_| GasCost::new(0, 0))
        .collect::<Vec<GasCost>>();

    new_from_instructions(instrs, native_table)
}

//
// pub fn executor_gas_cost(dvm_gas_cost: DvmGasCost) -> GasCost {
//     GasCost::new(
//         dvm_gas_cost.instruction_gas.get(),
//         dvm_gas_cost.memory_gas.get(),
//     )
// }
//
// pub fn fetch_cost_table() -> CostTable {
//     let DvmCostTable {
//         instruction_table,
//         native_table,
//         ..
//     } = dvm_runtime::gas_schedule::cost_table();
//
//     let executor_instruction_table = instruction_table
//         .into_iter()
//         .map(executor_gas_cost)
//         .collect();
//     let executor_native_table = native_table.into_iter().map(executor_gas_cost).collect();
//
//     CostTable {
//         instruction_table: executor_instruction_table,
//         native_table: executor_native_table,
//         gas_constants: GasConstants::default(),
//     }
// }
