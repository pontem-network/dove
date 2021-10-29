use rust_base58::base58::FromBase58;
use rust_base58::ToBase58;
use anyhow::{anyhow, ensure, Result};
use lazy_static::lazy_static;
use regex::Regex;
use crate::compiler::source_map::FileOffsetMap;
use move_core_types::account_address::AccountAddress;
use crate::compiler::mut_string::{MutString, NewValue};
use std::rc::Rc;

const SS58_PREFIX: &[u8] = b"SS58PRE";
const PUB_KEY_LENGTH: usize = 32;

lazy_static! {
    static ref SS58_REGEX: Regex = Regex::new(r#"[1-9A-HJ-NP-Za-km-z]{40,}"#,).unwrap();
}

fn ss58hash(data: &[u8]) -> blake2_rfc::blake2b::Blake2bResult {
    let mut context = blake2_rfc::blake2b::Blake2b::new(64);
    context.update(SS58_PREFIX);
    context.update(data);
    context.finalize()
}

pub fn ss58_to_address(ss58: &str) -> Result<AccountAddress> {
    let bs58 = match ss58.from_base58() {
        Ok(bs58) => bs58,
        Err(err) => return Err(anyhow!("Wrong base58:{}", err)),
    };
    ensure!(
        bs58.len() == PUB_KEY_LENGTH + 3,
        format!("Address length must be equal {} bytes", PUB_KEY_LENGTH + 3)
    );
    if bs58[PUB_KEY_LENGTH + 1..PUB_KEY_LENGTH + 3]
        != ss58hash(&bs58[0..PUB_KEY_LENGTH + 1]).as_bytes()[0..2]
    {
        return Err(anyhow!("Wrong address checksum"));
    }
    let mut addr = [0; PUB_KEY_LENGTH];
    addr.copy_from_slice(&bs58[1..PUB_KEY_LENGTH + 1]);
    Ok(AccountAddress::new(addr))
}

pub fn ss58_to_diem(ss58: &str) -> Result<String> {
    Ok(format!("{:#X}", ss58_to_address(ss58)?))
}

pub fn replace_ss58_addresses(
    source_text: &str,
    mut_str: &mut MutString,
    file_source_map: &mut FileOffsetMap,
) {
    for mat in SS58_REGEX.captures_iter(source_text).into_iter() {
        let item = mat
            .get(0)
            .expect("can't extract match from SS58 regex capture");

        let orig_address = item.as_str();
        if orig_address.starts_with("0x") {
            // diem match, don't replace
            continue;
        }
        if let Ok(diem_address) = ss58_to_diem(orig_address) {
            let diem_address = Rc::new(diem_address);
            mut_str.make_patch(item.start(), item.end(), NewValue::Rc(diem_address.clone()));

            file_source_map.insert_address_layer(
                item.end(),
                orig_address.to_owned(),
                diem_address,
            );
        }
    }
}

/// Convert address to ss58
/// 0xD43593C715FDD31C61141ABD04A99FD6822C8558854CCDE39A5684E7A56DA27D => 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
pub fn address_to_ss58(account: &AccountAddress) -> String {
    let mut ss58_address = [0; 35];
    ss58_address[0] = 42;
    ss58_address[1..33].copy_from_slice(&account.to_u8());
    let hash = ss58hash(&ss58_address[0..33]);
    ss58_address[33..35].copy_from_slice(&hash.as_bytes()[0..2]);
    ss58_address.to_base58()
}

#[cfg(test)]
mod test {
    use crate::compiler::source_map::FileOffsetMap;
    use crate::compiler::mut_string::MutString;
    use super::{
        PUB_KEY_LENGTH, replace_ss58_addresses, ss58_to_diem, ss58hash, ss58_to_address,
        address_to_ss58,
    };

    #[test]
    fn test_ss58_to_diem() {
        let polka_address = "G7UkJAutjbQyZGRiP8z5bBSBPBJ66JbTKAkFDq3cANwENyX";
        let diem_address = ss58_to_diem(polka_address).unwrap();

        assert_eq!(
            hex::decode(&diem_address[2..]).unwrap().len(),
            PUB_KEY_LENGTH
        );

        assert_eq!(
            "0x9C786090E2598AE884FF9D1F01D6A1A9BAF13A9E61F73633A8928F4D80BF7DFE",
            diem_address
        );
    }

    #[test]
    fn test_ss58hash() {
        let msg = b"hello, world!";
        let hash = ss58hash(msg).as_bytes().to_vec();

        assert_eq!("656facfcf4f90cce9ec9b65c9185ea75346507c67e25133f5809b442487468a674973f9167193e86bee0c706f6766f7edf638ed3e21ad12c2908ea62924af4d7", hex::encode(hash));
    }

    #[test]
    fn test_replace_ss58_addresses() {
        let source = r"
            script {
                use 0x01::Event;
                use 1exaAg2VJRQbyUBAeXcktChCAqjVP9TUxF3zo23R2T6EGdE::Math;

                fun main(account: &signer, a: u64, b: u64) {
                    let sum = Math::add(a, b);
                    Event::emit(account, sum);
                }
            }
        ";

        let mut mut_str = MutString::new(source);
        replace_ss58_addresses(source, &mut mut_str, &mut FileOffsetMap::default());
        assert_eq!(
            r"
            script {
                use 0x01::Event;
                use 0x1CF326C5AAA5AF9F0E2791E66310FE8F044FAADAF12567EAA0976959D1F7731F::Math;

                fun main(account: &signer, a: u64, b: u64) {
                    let sum = Math::add(a, b);
                    Event::emit(account, sum);
                }
            }
        ",
            mut_str.freeze()
        );
    }

    #[test]
    fn test_to_ss58check_with_version() {
        let ss58 = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY";
        let adr = ss58_to_address(ss58).unwrap();

        assert_eq!(ss58, &address_to_ss58(&adr));
    }
}
