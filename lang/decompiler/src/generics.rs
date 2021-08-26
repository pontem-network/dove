use std::collections::HashSet;
use std::fmt::Write;
use std::rc::Rc;

use anyhow::Error;
use move_binary_format::file_format::*;

use crate::{Encode, write_array};
use crate::unit::UnitAccess;

const GENERICS_PREFIX: [&str; 22] = [
    "T", "G", "V", "A", "B", "C", "D", "F", "H", "J", "K", "L", "M", "N", "P", "Q", "R", "S",
    "W", "X", "Y", "Z",
];

/// Generics template.
#[derive(Clone, Debug)]
pub struct Generics(Rc<GenericPrefix>);

/// Generics prefix.
#[derive(Debug)]
pub enum GenericPrefix {
    /// Simple generic prefix.
    /// Prefix from generic prefix table.
    SimplePrefix(&'static str),
    /// Random generic prefix.
    Generated(u16),
}

impl Generics {
    /// Create a new generics.
    pub fn new(unit: &impl UnitAccess) -> Generics {
        let identifiers: HashSet<&str> = unit.identifiers().iter().map(|i| i.as_str()).collect();

        let generic = if let Some(prefix) = GENERICS_PREFIX
            .iter()
            .find(|prefix| !identifiers.contains(*prefix))
        {
            GenericPrefix::SimplePrefix(*prefix)
        } else {
            GenericPrefix::Generated(rand::random())
        };

        Generics(Rc::new(generic))
    }

    /// Create generic.
    pub fn create_generic(&self, index: usize, abilities: AbilitySet) -> Generic {
        Generic {
            prefix: self.clone(),
            index,
            abilities,
        }
    }
}

/// Generic representation.
#[derive(Clone, Debug)]
pub struct Generic {
    prefix: Generics,
    index: usize,
    abilities: AbilitySet,
}

impl Generic {
    ///Returns generic name.
    pub fn as_name(&self) -> GenericName {
        GenericName(self)
    }
}

impl Encode for Generics {
    fn encode<W: Write>(&self, w: &mut W, _indent: usize) -> Result<(), Error> {
        match self.0.as_ref() {
            GenericPrefix::SimplePrefix(p) => w.write_str(p)?,
            GenericPrefix::Generated(p) => write!(w, "TYPE{}", p)?,
        }
        Ok(())
    }
}

impl Encode for Generic {
    fn encode<W: Write>(&self, w: &mut W, indent: usize) -> Result<(), Error> {
        self.prefix.encode(w, indent)?;

        if self.index != 0 {
            write!(w, "{}", self.index)?;
        }

        let abi = self.abilities;

        if abi != AbilitySet::EMPTY {
            w.write_str(":")?;

            let mut is_first = true;

            if abi.has_copy() {
                write!(w, " copy")?;
                is_first = false;
            }

            if abi.has_drop() {
                if !is_first {
                    w.write_str(" +")?;
                }
                write!(w, " drop")?;
                is_first = false;
            }
            if abi.has_key() {
                if !is_first {
                    w.write_str(" +")?;
                }
                write!(w, " key")?;
                is_first = false;
            }
            if abi.has_store() {
                if !is_first {
                    w.write_str(" +")?;
                }
                write!(w, " store")?;
            }
        }
        Ok(())
    }
}

/// Generic name.
pub struct GenericName<'a>(&'a Generic);

impl<'a> Encode for GenericName<'a> {
    fn encode<W: Write>(&self, w: &mut W, indent: usize) -> Result<(), Error> {
        self.0.prefix.encode(w, indent)?;

        if self.0.index != 0 {
            write!(w, "{}", self.0.index)?;
        }

        Ok(())
    }
}

/// Extract type parameters.
pub fn extract_type_params(params: &[AbilitySet], generics: &Generics) -> Vec<Generic> {
    params
        .iter()
        .enumerate()
        .map(|(i, k)| generics.create_generic(i, *k))
        .collect()
}

/// Write type parameters to writer.
pub fn write_type_parameters<W: Write>(w: &mut W, type_params: &[Generic]) -> Result<(), Error> {
    if !type_params.is_empty() {
        write_array(w, "<", ", ", type_params, ">")?;
    }
    Ok(())
}
