use move_core_types::account_address::AccountAddress;
use move_lang::shared::Address;

pub mod addresses;
pub mod bech32;
pub mod errors;
pub mod results;

#[derive(Debug, Clone)]
pub struct ProvidedAccountAddress {
    pub original: String,
    pub normalized_original: String,
    lowered: String,
}

impl ProvidedAccountAddress {
    pub fn new(original: String, normalized: String, lowered: String) -> ProvidedAccountAddress {
        ProvidedAccountAddress {
            original,
            normalized_original: normalized,
            lowered,
        }
    }

    pub fn lowered(&self) -> String {
        let lowered_bits = self.lowered[2..].to_owned();
        format!("0x{:0>40}", lowered_bits)
    }

    pub fn as_address(&self) -> Address {
        Address::new(self.as_account_address().into())
    }

    pub fn as_account_address(&self) -> AccountAddress {
        AccountAddress::from_hex_literal(&self.lowered).unwrap()
    }
}

impl Default for ProvidedAccountAddress {
    fn default() -> Self {
        ProvidedAccountAddress {
            original: "0x0".to_string(),
            normalized_original: "0x00000000000000000000000000000000".to_string(),
            lowered: "0x0000000000000000000000000000000000000000".to_string(),
        }
    }
}
