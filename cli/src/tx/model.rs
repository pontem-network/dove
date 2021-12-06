use move_core_types::identifier::Identifier;
use move_core_types::language_storage::TypeTag;
use anyhow::Error;
use move_core_types::value::MoveValue;
use std::str::FromStr;
use move_core_types::account_address::AccountAddress;
use move_core_types::transaction_argument::TransactionArgument;
use serde::{Deserialize, Serialize};
use move_symbol_pool::Symbol;
use lang::bytecode::info::BytecodeInfo;

/// Transaction model.
#[derive(Serialize, Deserialize, Debug)]
pub enum Transaction {
    /// Version 1.
    V1(V1),
}

/// Transaction model.
#[derive(Serialize, Deserialize, Debug)]
pub struct V1 {
    /// Signers.
    pub signers: Vec<Signer>,
    /// Call declaration.
    pub call: Call,
    /// Script args.
    pub args: Vec<Vec<u8>>,
    /// Script type arguments.
    pub type_args: Vec<TypeTag>,
}

/// Call declaration.
#[derive(Serialize, Deserialize, Debug)]
pub enum Call {
    /// Script
    Script {
        /// Script bytecode.
        code: Vec<u8>,
    },
    /// Function in module with script viability.
    ScriptFunction {
        /// Module address.
        mod_address: AccountAddress,
        /// Module name.
        mod_name: Identifier,
        /// Function name.
        func_name: Identifier,
    },
}

impl Transaction {
    /// Create a new script transaction.
    pub fn new_script_tx(
        signers: Vec<Signer>,
        code: Vec<u8>,
        args: Vec<ScriptArg>,
        type_args: Vec<TypeTag>,
    ) -> Result<Transaction, Error> {
        Ok(Transaction::V1(V1 {
            signers,
            call: Call::Script { code },
            args: Self::make_args(args)?,
            type_args,
        }))
    }

    /// Create a new function transaction.
    pub fn new_func_tx(
        signers: Vec<Signer>,
        mod_address: AccountAddress,
        mod_name: Identifier,
        func_name: Identifier,
        args: Vec<ScriptArg>,
        type_args: Vec<TypeTag>,
    ) -> Result<Transaction, Error> {
        Ok(Transaction::V1(V1 {
            signers,
            call: Call::ScriptFunction {
                mod_address,
                func_name,
                mod_name,
            },
            args: Self::make_args(args)?,
            type_args,
        }))
    }

    fn make_args(args: Vec<ScriptArg>) -> Result<Vec<Vec<u8>>, Error> {
        args.into_iter()
            .map(ScriptArg::into)
            .map(|val: MoveValue| bcs::to_bytes(&val))
            .collect::<Result<_, _>>()
            .map_err(Error::msg)
    }

    /// Returns last version.
    pub fn inner_mut(&mut self) -> &mut V1 {
        match self {
            Transaction::V1(v) => v,
        }
    }

    /// Returns last version.
    pub fn inner(self) -> V1 {
        match self {
            Transaction::V1(v) => v,
        }
    }
}

/// Script argument type.
#[derive(Debug, PartialEq, Eq)]
pub enum ScriptArg {
    /// u8
    U8(u8),
    /// u64
    U64(u64),
    /// u128
    U128(u128),
    /// bool
    Bool(bool),
    /// address
    Address(AccountAddress),
    /// vector<u8>
    VectorU8(Vec<u8>),
    /// vector<u64>
    VectorU64(Vec<u64>),
    /// vector<u128>
    VectorU128(Vec<u128>),
    /// vector<bool>
    VectorBool(Vec<bool>),
    /// vector<address>
    VectorAddress(Vec<AccountAddress>),
}

impl From<ScriptArg> for MoveValue {
    fn from(arg: ScriptArg) -> Self {
        match arg {
            ScriptArg::U8(val) => MoveValue::U8(val),
            ScriptArg::U64(val) => MoveValue::U64(val),
            ScriptArg::U128(val) => MoveValue::U128(val),
            ScriptArg::Bool(val) => MoveValue::Bool(val),
            ScriptArg::Address(val) => MoveValue::Address(val),
            ScriptArg::VectorU8(val) => MoveValue::vector_u8(val),
            ScriptArg::VectorU64(val) => {
                MoveValue::Vector(val.into_iter().map(MoveValue::U64).collect())
            }
            ScriptArg::VectorU128(val) => {
                MoveValue::Vector(val.into_iter().map(MoveValue::U128).collect())
            }
            ScriptArg::VectorBool(val) => {
                MoveValue::Vector(val.into_iter().map(MoveValue::Bool).collect())
            }
            ScriptArg::VectorAddress(val) => {
                MoveValue::Vector(val.into_iter().map(MoveValue::Address).collect())
            }
        }
    }
}

impl TryInto<TransactionArgument> for ScriptArg {
    type Error = Error;

    fn try_into(self) -> Result<TransactionArgument, Self::Error> {
        Ok(match self {
            ScriptArg::U8(val) => TransactionArgument::U8(val),
            ScriptArg::U64(val) => TransactionArgument::U64(val),
            ScriptArg::U128(val) => TransactionArgument::U128(val),
            ScriptArg::Bool(val) => TransactionArgument::Bool(val),
            ScriptArg::Address(val) => TransactionArgument::Address(val),
            ScriptArg::VectorU8(val) => TransactionArgument::U8Vector(val),
            ScriptArg::VectorU64(_)
            | ScriptArg::VectorU128(_)
            | ScriptArg::VectorBool(_)
            | ScriptArg::VectorAddress(_) => bail!("Unssuported transaction args."),
        })
    }
}

/// Signer type.
#[derive(Serialize, Deserialize, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum Signer {
    /// Root signer.
    Root,
    /// Treasury signer.
    Treasury,
    /// Template to replace.
    Placeholder,
    /// Named address.
    Name(Symbol),
}

impl FromStr for Signer {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_lowercase().as_str() {
            "root" | "rt" | "dr" => Self::Root,
            "treasury" | "tr" | "tc" => Self::Treasury,
            "_" => Self::Placeholder,
            _ => Self::Name(Symbol::from(s)),
        })
    }
}

#[derive(Debug, PartialEq)]
pub(crate) enum Signers {
    Explicit(Vec<AccountAddress>),
    Implicit(Vec<Signer>),
}

impl Signers {
    #[cfg(test)]
    pub fn len(&self) -> usize {
        match self {
            Signers::Explicit(v) => v.len(),
            Signers::Implicit(v) => v.len(),
        }
    }
}

/// Transaction with additional info.
#[derive(Debug)]
pub enum EnrichedTransaction {
    /// A transaction intended for execution in the local executor.
    Local {
        /// Bytecode info.
        bi: BytecodeInfo,
        /// Script args.
        args: Vec<ScriptArg>,
        /// Type tags.
        type_tag: Vec<TypeTag>,
        /// Function name.
        func_name: Option<String>,
        /// Signers.
        signers: Vec<AccountAddress>,
    },
    /// A transaction intended for execution in the chain executor.
    Global {
        /// Bytecode info.
        bi: BytecodeInfo,
        /// Transaction.
        tx: Transaction,
        /// Transaction name.
        name: String,
    },
}
