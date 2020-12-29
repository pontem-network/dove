use rust_base58::base58::FromBase58;
use anyhow::{anyhow, ensure, Result};
use lazy_static::lazy_static;
use regex::Regex;
use crate::compiler::source_map::FileOffsetMap;

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

pub fn ss58_to_libra(ss58: &str) -> Result<String> {
    let bs58 = match ss58.from_base58() {
        Ok(bs58) => bs58,
        Err(_) => return Err(anyhow!("Wrong base58")),
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
    Ok(format!("0x{}", hex::encode_upper(addr)))
}

pub fn replace_ss58_addresses(source: &str, file_source_map: &mut FileOffsetMap) -> String {
    let mut transformed_source = source.to_string();

    for mat in SS58_REGEX.captures_iter(source).into_iter() {
        let item = mat
            .get(0)
            .expect("can't extract match from SS58 regex capture");

        let orig_address = item.as_str();
        if orig_address.starts_with("0x") {
            // libra match, don't replace
            continue;
        }
        if let Ok(libra_address) = ss58_to_libra(orig_address) {
            file_source_map.insert_address_layer(
                item.end(),
                orig_address.to_owned(),
                libra_address.clone(),
            );
            transformed_source = transformed_source.replace(orig_address, &libra_address);
        }
    }
    transformed_source
}

#[cfg(test)]
mod test {
    use crate::compiler::source_map::FileOffsetMap;
    use super::{PUB_KEY_LENGTH, replace_ss58_addresses, ss58_to_libra, ss58hash};

    #[test]
    fn test_ss58_to_libra() {
        let polka_address = "G7UkJAutjbQyZGRiP8z5bBSBPBJ66JbTKAkFDq3cANwENyX";
        let libra_address = ss58_to_libra(&polka_address).unwrap();

        assert_eq!(
            hex::decode(&libra_address[2..]).unwrap().len(),
            PUB_KEY_LENGTH
        );

        assert_eq!(
            "0x9C786090E2598AE884FF9D1F01D6A1A9BAF13A9E61F73633A8928F4D80BF7DFE",
            libra_address
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

        let res = replace_ss58_addresses(source, &mut FileOffsetMap::default());
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
            res
        );
    }
}
