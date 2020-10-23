use move_core_types::language_storage::{StructTag, CORE_CODE_ADDRESS, TypeTag, ModuleId};
use move_core_types::identifier::Identifier;
use move_core_types::account_address::AccountAddress;

const COIN_MODULE: &str = "Coins";
const PRICE_STRUCT: &str = "Price";

const XFI_MODULE: &str = "XFI";
const XFI_RESOURCE: &str = "T";

/// Currency price.
#[derive(Debug, PartialEq, Eq)]
pub struct Price {
    /// Currency price.
    pub price: u128,
}

fn currency_type(curr: &str) -> TypeTag {
    let curr = curr.to_uppercase();
    if curr == XFI_MODULE {
        TypeTag::Struct(StructTag {
            address: CORE_CODE_ADDRESS,
            name: Identifier::new(XFI_RESOURCE).expect("Valid currency name."),
            module: Identifier::new(XFI_MODULE).expect("Valid module name."),
            type_params: vec![],
        })
    } else {
        TypeTag::Struct(StructTag {
            address: CORE_CODE_ADDRESS,
            name: Identifier::new(curr).expect("Valid currency name."),
            module: Identifier::new(COIN_MODULE).expect("Valid module name."),
            type_params: vec![],
        })
    }
}

pub fn oracle_coins_module() -> ModuleId {
    let addr = AccountAddress::from_hex_literal("0x1").unwrap();
    ModuleId::new(addr, Identifier::new("Coins").expect("Valid module ident."))
}

/// Returns oracle metadata struct tag.
pub fn oracle_metadata(first: &str, second: &str) -> StructTag {
    StructTag {
        address: CORE_CODE_ADDRESS,
        name: Identifier::new(PRICE_STRUCT).expect("Valid struct name."),
        module: Identifier::new(COIN_MODULE).expect("Valid module name."),
        type_params: vec![currency_type(first), currency_type(second)],
    }
}

pub fn time_metadata() -> StructTag {
    StructTag {
        address: CORE_CODE_ADDRESS,
        name: Identifier::new("CurrentTimestamp").expect("Valid module name."),
        module: Identifier::new("Time").expect("Valid module name."),
        type_params: vec![],
    }
}
