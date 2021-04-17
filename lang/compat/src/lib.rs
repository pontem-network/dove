#[macro_use]
extern crate anyhow;

use anyhow::Result;
use crate::adapt::AddressAdaptation;

mod adapt;
mod context;
mod mutator;

pub const BASIS_LENGTH: usize = 32;

pub enum SourceType {
    Dfninance = 20,
    Diem = 16,
}

pub fn adapt_to_basis(bytes: &mut Vec<u8>, source_type: SourceType) -> Result<()> {
    let adapt = AddressAdaptation::new(source_type as usize, BASIS_LENGTH);
    adapt.make(bytes)
}

pub fn adapt_from_basis(bytes: &mut Vec<u8>, source_type: SourceType) -> Result<()> {
    let adapt = AddressAdaptation::new(BASIS_LENGTH, source_type as usize);
    adapt.make(bytes)
}
