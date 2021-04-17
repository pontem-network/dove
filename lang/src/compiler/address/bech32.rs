use std::fmt::Write;

use anyhow::Result;
use lazy_static::lazy_static;
use move_core_types::account_address::AccountAddress;
use regex::Regex;

use crate::compiler::source_map::FileOffsetMap;

pub static HRP: &str = "wallet";

lazy_static! {
    static ref BECH32_REGEX: Regex = Regex::new(
        r#"[\s=]+(["!#$%&'()*+,\-./0123456789:;<=>?@A-Z\[\\\]^_`a-z{|}~]{1,83}1[A-Z0-9a-z&&[^boi1]]{6,})"#,
    )
    .unwrap();
}

pub fn bech32_into_address(address: &str) -> Result<AccountAddress> {
    let (_, data_bytes) = bech32::decode(address)?;
    let data = bech32::convert_bits(&data_bytes, 5, 8, true)?;

    if data.len() == AccountAddress::LENGTH {
        Ok(AccountAddress::from_bytes(data)?)
    } else if data.len() > AccountAddress::LENGTH {
        let prefix_to_skip = AccountAddress::LENGTH - data.len();
        Ok(AccountAddress::from_bytes(&data[prefix_to_skip..])?)
    } else {
        let mut address_buff = [0u8; AccountAddress::LENGTH];
        let pudding = data.len() - AccountAddress::LENGTH;
        address_buff[pudding..].copy_from_slice(&data);
        Ok(AccountAddress::new(address_buff))
    }
}

pub fn bech32_into_diem(address: &str) -> Result<String> {
    let (_, data_bytes) = bech32::decode(address)?;
    let data = bech32::convert_bits(&data_bytes, 5, 8, true)?;

    let mut addr = String::with_capacity(data.len() * 2);
    addr.push_str("0x");
    for byte in &data {
        write!(addr, "{:02X}", byte)?;
    }
    Ok(addr)
}

#[cfg(test)]
pub fn libra_into_bech32(diem_address: &str) -> Result<String> {
    ensure!(
        diem_address.starts_with("0x"),
        "Pass address with 0x prefix"
    );
    let data = hex::decode(&diem_address[2..])?;
    let data = bech32::convert_bits(&data, 8, 5, true)?
        .into_iter()
        .map(bech32::u5::try_from_u8)
        .collect::<Result<Vec<_>, _>>()?;

    Ok(bech32::encode(&HRP, data)?)
}

// TODO optimization required.
pub fn replace_bech32_addresses(source: String, file_source_map: &mut FileOffsetMap) -> String {
    let mut transformed_source: Option<String> = None;

    for mat in BECH32_REGEX.captures_iter(&source).into_iter() {
        let item = mat.get(1).unwrap();

        let orig_address = item.as_str();
        if orig_address.starts_with("0x") {
            continue;
        }
        if let Ok(diem_address) = bech32_into_diem(orig_address) {
            transformed_source = Some(match transformed_source {
                Some(source) => source.replace(orig_address, &diem_address),
                None => source.replace(orig_address, &diem_address),
            });

            file_source_map.insert_address_layer(
                item.end(),
                orig_address.to_owned(),
                diem_address,
            );
        }
    }
    transformed_source.unwrap_or_else(|| source)
}
