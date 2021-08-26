use crate::structs::StructDef;
use anyhow::Error;
use crate::{Encode, INDENT, Config};
use std::fmt::Write;
use crate::generics::Generics;
use crate::imports::Imports;
use crate::functions::FunctionsDef;
use crate::unit::{UnitAccess};
use move_core_types::account_address::AccountAddress;

/// Module representation.
pub struct Module<'a> {
    address: AccountAddress,
    name: String,
    structs: Vec<StructDef<'a>>,
    functions: Vec<FunctionsDef<'a>>,
    imports: &'a Imports<'a>,
}

impl<'a> Module<'a> {
    /// Creates a new module.
    pub fn new(
        unit: &'a impl UnitAccess,
        imports: &'a Imports<'a>,
        generics: &'a Generics,
        config: &'a Config,
    ) -> Module<'a> {
        let structs = unit
            .struct_defs()
            .iter()
            .map(|def| StructDef::new(def, unit, generics, imports, config))
            .collect();

        let functions = unit
            .function_defs()
            .iter()
            .map(|def| FunctionsDef::new(def, unit, generics, imports, config))
            .collect();

        let id = unit.self_id();
        Module {
            address: *id.address(),
            name: id.name().as_str().to_owned(),
            structs,
            functions,
            imports,
        }
    }
}

impl<'a> Encode for Module<'a> {
    fn encode<W: Write>(&self, w: &mut W, _indent: usize) -> Result<(), Error> {
        writeln!(w, "module 0x{}::{} {{", self.address, self.name)?;

        self.imports.encode(w, INDENT)?;
        writeln!(w)?;

        for struct_def in &self.structs {
            struct_def.encode(w, INDENT)?;
            writeln!(w, "\n")?;
        }

        for function in &self.functions {
            function.encode(w, INDENT)?;
            writeln!(w, "\n")?;
        }

        writeln!(w, "}}")?;
        Ok(())
    }
}
