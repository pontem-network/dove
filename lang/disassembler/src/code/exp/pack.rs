use crate::imports::Import;
use crate::types::FType;
use crate::code::exp::{ExpLoc, find_range, Exp, SourceRange};
use crate::{Encode, write_array};
use anyhow::Error;
use crate::code::translator::Context;
use crate::libra::file_format::*;
use crate::unit::UnitAccess;
use std::fmt::Write;

/// Pack field.
#[derive(Debug)]
pub struct PackField<'a> {
    /// Field name.
    pub name: &'a str,
    /// Field value.
    pub value: ExpLoc<'a>,
}

impl<'a> Encode for PackField<'a> {
    fn encode<W: Write>(&self, w: &mut W, _: usize) -> Result<(), Error> {
        w.write_str(self.name)?;
        w.write_str(": ")?;

        if self.value.as_ref().is_nop() {
            w.write_str("_")?;
        } else {
            self.value.encode(w, 0)?;
        }

        Ok(())
    }
}

/// Struct pack.
#[derive(Debug)]
pub struct Pack<'a> {
    /// Struct import.
    pub module: Option<Import<'a>>,
    /// Struct name.
    pub name: &'a str,
    /// Struct type parameters.
    pub type_params: Vec<FType<'a>>,
    /// Struct fields.
    pub fields: Vec<PackField<'a>>,
}

impl<'a> Pack<'a> {
    /// Create a new `Pack` expressions.
    pub fn exp(
        index: &StructDefinitionIndex,
        type_params: Option<&SignatureIndex>,
        ctx: &mut impl Context<'a>,
        unit: &'a impl UnitAccess,
    ) -> Exp<'a> {
        if let Some(def) = unit.struct_def(*index) {
            let struct_handler = unit.struct_handle(def.struct_handle);
            let module = unit.module_handle(struct_handler.module);

            let name = unit.identifier(struct_handler.name);

            let fields = ctx.pack_fields(&def);
            let type_params = ctx.extract_signature(type_params);

            Exp::Pack(Pack {
                module: ctx.module_import(module),
                name,
                type_params,
                fields,
            })
        } else {
            Exp::Error(Bytecode::Pack(*index))
        }
    }
}

impl<'a> SourceRange for Pack<'a> {
    fn source_range(&self) -> Option<(usize, usize)> {
        find_range(self.fields.iter().map(|f| &f.value))
    }
}

impl<'a> Encode for Pack<'a> {
    fn encode<W: Write>(&self, w: &mut W, _: usize) -> Result<(), Error> {
        if let Some(module) = &self.module {
            module.encode(w, 0)?;
            w.write_str("::")?;
        }
        w.write_str(self.name)?;
        if !self.type_params.is_empty() {
            write_array(w, "<", ", ", &self.type_params, ">")?;
        }

        write_array(w, " { ", ", ", &self.fields, " }")
    }
}
