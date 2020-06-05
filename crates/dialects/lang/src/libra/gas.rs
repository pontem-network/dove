use orig_move_core_types::gas_schedule::{CostTable, GasConstants, GasCost};

pub fn fetch_cost_table() -> CostTable {
    let instructions_table_bytes = libra_vm_genesis::genesis_gas_schedule::INITIAL_GAS_SCHEDULE
        .0
        .clone();
    let instruction_table: Vec<GasCost> =
        orig_libra_canonical_serialization::from_bytes(&instructions_table_bytes).unwrap();

    let native_table_bytes = libra_vm_genesis::genesis_gas_schedule::INITIAL_GAS_SCHEDULE
        .1
        .clone();
    let native_table: Vec<GasCost> =
        orig_libra_canonical_serialization::from_bytes(&native_table_bytes).unwrap();

    CostTable {
        instruction_table,
        native_table,
        gas_constants: GasConstants::default(),
    }
}
