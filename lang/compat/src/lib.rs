#[macro_use]
extern crate anyhow;

use anyhow::Result;
use crate::adapt::AddressAdaptation;

mod adapt;
mod context;
mod mutator;

pub const BASIS_LENGTH: usize = 32;

pub enum AddressType {
    Dfninance = 20,
    Diem = 16,
}

pub fn adapt_to_basis(bytes: &mut Vec<u8>, address_type: AddressType) -> Result<()> {
    let adapt = AddressAdaptation::new(address_type as usize, BASIS_LENGTH);
    adapt.make(bytes)
}

pub fn adapt_from_basis(bytes: &mut Vec<u8>, address_type: AddressType) -> Result<()> {
    let adapt = AddressAdaptation::new(BASIS_LENGTH, address_type as usize);
    adapt.make(bytes)
}
