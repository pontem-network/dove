use move_core_types::language_storage::{StructTag, CORE_CODE_ADDRESS, TypeTag, ModuleId};
use move_core_types::identifier::Identifier;
use move_core_types::account_address::AccountAddress;

const COIN_MODULE: &str = "Coins";
const PRICE_STRUCT: &str = "Price";

const ACCOUNT_MODULE: &str = "Account";
const ACCOUNT_BALANCE_STRUCT: &str = "Balance";

const PONT_MODULE: &str = "PONT";
const PONT_RESOURCE: &str = "T";

const BLOCK_RESOURCE: &str = "BlockMetadata";

/// Currency price.
#[derive(Debug, PartialEq, Eq)]
pub struct Price {
    /// Currency price.
    pub price: u128,
}

pub fn currency_struct(curr: &str) -> StructTag {
    let curr = curr.to_uppercase();
    if curr == PONT_MODULE {
        StructTag {
            address: CORE_CODE_ADDRESS,
            module: Identifier::new(PONT_MODULE).expect("Valid module name."),
            name: Identifier::new(PONT_RESOURCE).expect("Valid currency name."),
            type_params: vec![],
        }
    } else {
        StructTag {
            address: CORE_CODE_ADDRESS,
            module: Identifier::new(COIN_MODULE).expect("Valid module name."),
            name: Identifier::new(curr).expect("Valid currency name."),
            type_params: vec![],
        }
    }
}

fn currency_type(curr: &str) -> TypeTag {
    TypeTag::Struct(currency_struct(curr))
}

pub fn oracle_coins_module() -> ModuleId {
    let addr = AccountAddress::from_hex_literal("0x1").unwrap();
    ModuleId::new(addr, Identifier::new("Coins").expect("Valid module ident."))
}

/// Returns oracle metadata struct tag.
pub fn oracle_metadata(first: &str, second: &str) -> StructTag {
    StructTag {
        address: CORE_CODE_ADDRESS,
        module: Identifier::new(COIN_MODULE).expect("Valid module name."),
        name: Identifier::new(PRICE_STRUCT).expect("Valid struct name."),
        type_params: vec![currency_type(first), currency_type(second)],
    }
}

pub fn block_metadata() -> StructTag {
    StructTag {
        address: CORE_CODE_ADDRESS,
        name: Identifier::new(BLOCK_RESOURCE).expect("Valid module name."),
        module: Identifier::new("Block").expect("Valid module name."),
        type_params: vec![],
    }
}

pub fn time_metadata() -> StructTag {
    StructTag {
        address: CORE_CODE_ADDRESS,
        module: Identifier::new("Time").expect("Valid module name."),
        name: Identifier::new("CurrentTimestamp").expect("Valid module name."),
        type_params: vec![],
    }
}

pub fn coin_balance_metadata(currency: &str) -> StructTag {
    StructTag {
        address: CORE_CODE_ADDRESS,
        module: Identifier::new(ACCOUNT_MODULE).expect("Valid module name."),
        name: Identifier::new(ACCOUNT_BALANCE_STRUCT).expect("Valid module name."),
        type_params: vec![currency_type(currency)],
    }
}
