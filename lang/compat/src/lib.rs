use anyhow::{ensure, Result, bail};
use move_core_types::account_address::AccountAddress;

use crate::adapt::AddressAdaptation;
use dialect::Dialect;
use std::convert::TryFrom;

mod adapt;
mod context;
mod mutator;

pub const BASIS_LENGTH: usize = 32;

#[derive(Copy, Clone)]
pub enum AddressType {
    Dfninance = 20,
    Diem = 16,
}

impl TryFrom<&Dialect> for AddressType {
    type Error = anyhow::Error;
    fn try_from(value: &Dialect) -> std::prelude::rust_2015::Result<Self, Self::Error> {
        match value {
            Dialect::Diem => Ok(AddressType::Diem),
            Dialect::DFinance => Ok(AddressType::Dfninance),
            Dialect::Pont => bail!(r#"Dialect "Pont" is not supported"#),
        }
    }
}

pub fn adapt_to_basis(bytes: &mut Vec<u8>, address_type: AddressType) -> Result<()> {
    let adapt = AddressAdaptation::new(address_type as usize, BASIS_LENGTH);
    adapt.make(bytes)
}

pub fn adapt_from_basis(bytes: &mut Vec<u8>, address_type: AddressType) -> Result<()> {
    let adapt = AddressAdaptation::new(BASIS_LENGTH, address_type as usize);
    adapt.make(bytes)
}

pub fn adapt_address_to_target(address: AccountAddress, address_type: AddressType) -> Vec<u8> {
    let buffer = address.to_u8();
    match address_type {
        AddressType::Dfninance => buffer[12..].to_vec(),
        AddressType::Diem => buffer[16..].to_vec(),
    }
}

pub fn adapt_address_to_basis(
    address: &[u8],
    address_type: AddressType,
) -> Result<AccountAddress> {
    let buffer = match address_type {
        AddressType::Dfninance => {
            ensure!(
                address.len() == AddressType::Dfninance as usize,
                "Dfninance address must be 20 bytes long."
            );
            let mut buffer = [0; 32];
            buffer[12..].copy_from_slice(address);
            buffer
        }
        AddressType::Diem => {
            ensure!(
                address.len() == AddressType::Diem as usize,
                "Diem address must be 16 bytes long."
            );
            let mut buffer = [0; 32];
            buffer[16..].copy_from_slice(address);
            buffer
        }
    };

    Ok(AccountAddress::new(buffer))
}

#[cfg(test)]
mod test {
    use move_core_types::account_address::AccountAddress;

    use crate::{adapt_address_to_basis, adapt_address_to_target, AddressType};

    #[test]
    fn test_address_adaptation() {
        let address = AccountAddress::random();
        let dfi_address = adapt_address_to_target(address, AddressType::Dfninance);
        assert_eq!(dfi_address.len(), AddressType::Dfninance as usize);

        let diem_address = adapt_address_to_target(address, AddressType::Diem);
        assert_eq!(diem_address.len(), AddressType::Diem as usize);

        assert_eq!(
            adapt_address_to_target(
                adapt_address_to_basis(&dfi_address, AddressType::Dfninance).unwrap(),
                AddressType::Dfninance
            ),
            dfi_address
        );
        assert_eq!(&address.to_u8()[12..], &dfi_address[..]);

        assert_eq!(
            adapt_address_to_target(
                adapt_address_to_basis(&diem_address, AddressType::Diem).unwrap(),
                AddressType::Diem
            ),
            diem_address
        );
        assert_eq!(&address.to_u8()[16..], &diem_address[..]);
    }
}
