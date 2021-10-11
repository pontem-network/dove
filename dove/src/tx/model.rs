use move_core_types::account_address::AccountAddress;
use move_binary_format::CompiledModule;
use dove_lib::tx::model::Transaction;

/// Transaction with additional info.
#[derive(Debug)]
pub enum EnrichedTransaction {
    /// A transaction intended for execution in the local executor.
    Local {
        /// Transaction.
        tx: Transaction,
        /// Signers.
        signers: Vec<AccountAddress>,
        /// Execution dependence.
        deps: Vec<CompiledModule>,
    },
    /// A transaction intended for execution in the chain executor.
    Global {
        /// Transaction.
        tx: Transaction,
        /// Transaction name.
        name: String,
    },
}
