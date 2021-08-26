use std::fmt::Write;
use std::rc::Rc;

use anyhow::{anyhow, Result};
use lazy_static::lazy_static;
use move_core_types::account_address::AccountAddress;
use regex::Regex;

use compat::AddressType;

use crate::compiler::mut_string::{MutString, NewValue};
use crate::compiler::source_map::FileOffsetMap;

pub static HRP: &str = "wallet";

lazy_static! {
    static ref BECH32_REGEX: Regex = Regex::new(
        r#"[\s@=]+(["!#$%&'()*+,\-./0123456789:;<=>?@A-Z\[\\\]^_`a-z{|}~]{1,83}1[A-Z0-9a-z&&[^boi1]]{6,})"#,
    )
    .unwrap();
}

pub fn bech32_into_address(address: &str) -> Result<AccountAddress> {
    let (_, data_bytes) = bech32::decode(address)?;
    let data = bech32::convert_bits(&data_bytes, 5, 8, true)?;
    if data.len() != AddressType::Dfninance as usize {
        Err(anyhow!(
            "Invalid dfinance address length [{}]. Expected {} bytes.",
            address,
            AddressType::Dfninance as usize
        ))
    } else {
        let mut address_buff = [0u8; AccountAddress::LENGTH];
        address_buff[AccountAddress::LENGTH - AddressType::Dfninance as usize..]
            .copy_from_slice(&data);
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
pub fn diem_into_bech32(diem_address: &str) -> Result<String> {
    ensure!(
        diem_address.starts_with("0x"),
        "Pass address with 0x prefix"
    );
    let data = hex::decode(&diem_address[2..])?;
    let data = bech32::convert_bits(&data, 8, 5, true)?
        .into_iter()
        .map(bech32::u5::try_from_u8)
        .collect::<Result<Vec<_>, _>>()?;

    Ok(bech32::encode(HRP, data)?)
}

pub fn replace_bech32_addresses(
    source_text: &str,
    mut_str: &mut MutString,
    file_source_map: &mut FileOffsetMap,
) {
    for mat in BECH32_REGEX.captures_iter(source_text).into_iter() {
        let item = mat.get(1).unwrap();

        let orig_address = item.as_str();
        if orig_address.starts_with("0x") {
            continue;
        }
        if let Ok(diem_address) = bech32_into_diem(orig_address) {
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

#[cfg(test)]
mod tests {
    use move_core_types::account_address::AccountAddress;

    use crate::compiler::address::bech32::bech32_into_address;

    #[test]
    pub fn test_bech32_into_address() {
        assert_eq!(
            bech32_into_address("wallet1me0cdn52672y7feddy7tgcj6j4dkzq2su745vh").unwrap(),
            AccountAddress::from_hex(
                "000000000000000000000000DE5F86CE8AD7944F272D693CB4625A955B610150"
            )
            .unwrap()
        )
    }
}
