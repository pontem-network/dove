use anyhow::{ensure, Result};
use bech32::u5;
use lazy_static::lazy_static;

use crate::errors::OffsetsMap;
use regex::Regex;

pub static HRP: &str = "wallet";

lazy_static! {
    static ref BECH32_REGEX: Regex = Regex::new(
        r#"[\s=]+(["!#$%&'()*+,\-./0123456789:;<=>?@A-Z\[\\\]^_`a-z{|}~]{1,83}1[A-Z0-9a-z&&[^boi1]]{6,})"#,
    )
    .unwrap();
}

pub fn bech32_into_libra(address: &str) -> Result<String> {
    let (_, data_bytes) = bech32::decode(address)?;
    let data = bech32::convert_bits(&data_bytes, 5, 8, true)?;
    Ok(format!("{}00000000", hex::encode(&data)))
}

pub fn libra_into_bech32(libra_address: &str) -> Result<String> {
    ensure!(
        libra_address.starts_with("0x"),
        "Pass address with 0x prefix"
    );
    ensure!(libra_address.len() == 50, "Address should be of length 50");
    let data = hex::decode(&libra_address[2..42])?;
    let data = bech32::convert_bits(&data, 8, 5, true)?
        .into_iter()
        .map(u5::try_from_u8)
        .collect::<Result<Vec<_>, _>>()?;

    Ok(bech32::encode(&HRP, data)?)
}

pub fn replace_bech32_addresses(source: &str) -> (String, OffsetsMap) {
    let mut transformed_source = source.to_string();
    let mut offsets_map = OffsetsMap::new();
    let mut last_interval_pos = 0;
    let mut overall_offset = 0;

    for mat in BECH32_REGEX.captures_iter(source).into_iter() {
        let item = mat.get(1).unwrap();

        let address = item.as_str();
        if address.starts_with("0x") {
            // libra match, don't replace
            continue;
        }
        if let Ok(libra_address) = bech32_into_libra(address) {
            let end = item.end() + overall_offset;
            offsets_map.insert((last_interval_pos, end), overall_offset);
            last_interval_pos = end;

            let libra_address_s = format!("0x{}", libra_address);
            transformed_source = transformed_source.replace(address, &libra_address_s);

            let len_diff = libra_address_s.len() - address.len();
            overall_offset += len_diff;
        }
    }
    offsets_map.insert(
        (last_interval_pos, source.len() + overall_offset),
        overall_offset,
    );
    (transformed_source, offsets_map)
}
