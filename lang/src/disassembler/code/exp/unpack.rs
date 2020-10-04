use crate::disassembler::imports::Import;
use crate::disassembler::types::FType;
use crate::disassembler::code::exp::pack::PackField;
use crate::disassembler::code::exp::{ExpLoc, SourceRange, find_range, Exp};
use crate::disassembler::{Encode, write_array};
use anyhow::Error;
use std::fmt::Write;
use crate::disassembler::code::translator::Context;
use libra::file_format::*;
use crate::disassembler::unit::UnitAccess;

/// Unpack expressions.
#[derive(Debug)]
pub struct Unpack<'a> {
    /// Struct import.
    pub module: Option<Import<'a>>,
    /// Struct name.
    pub name: &'a str,
    /// Struct type parameters.
    pub type_params: Vec<FType<'a>>,
    /// Struct fields.
    pub fields: Vec<PackField<'a>>,
    /// Struct instance.
    pub source: ExpLoc<'a>,
}

impl<'a> Unpack<'a> {
    /// Creates a new `Unpack` expressions;
    pub fn exp(
        index: &StructDefinitionIndex,
        type_params: Option<&SignatureIndex>,
        ctx: &mut impl Context<'a>,
        unit: &'a impl UnitAccess,
    ) -> Exp<'a> {
        if let Some(def) = unit.struct_def(*index) {
            let struct_handler = unit.struct_handle(def.struct_handle);
            let module_handle = unit.module_handle(struct_handler.module);

            let name = unit.identifier(struct_handler.name);

            let type_params = ctx.extract_signature(type_params);

            let fields = match &def.field_information {
                StructFieldInformation::Native => vec![],
                StructFieldInformation::Declared(fields) => {
                    fields.iter().map(|f| unit.identifier(f.name)).collect()
                }
            };

            let forwards_exp = fields.len();

            let mut expressions = ctx
                .translate_block(forwards_exp)
                .into_iter()
                .rev()
                .collect::<Vec<_>>();

            for _ in 0..forwards_exp - expressions.len() {
                expressions.push(ExpLoc::new(ctx.opcode_offset(), Exp::Nop));
            }

            let fields = fields
                .into_iter()
                .zip(expressions)
                .map(|(name, exp)| PackField { name, value: exp })
                .collect();

            Exp::Unpack(Unpack {
                module: ctx.module_import(module_handle),
                name,
                type_params,
                fields,
                source: ctx.pop_exp(),
            })
        } else {
            ctx.err()
        }
    }
}

impl<'a> SourceRange for Unpack<'a> {
    fn source_range(&self) -> Option<(usize, usize)> {
        find_range(self.fields.iter().map(|f| &f.value))
    }
}

impl<'a> Encode for Unpack<'a> {
    fn encode<W: Write>(&self, w: &mut W, _: usize) -> Result<(), Error> {
        if let Some(module) = &self.module {
            module.encode(w, 0)?;
            w.write_str("::")?;
        }
        w.write_str(self.name)?;
        if !self.type_params.is_empty() {
            write_array(w, "<", ", ", &self.type_params, ">")?;
        }

        write_array(w, " { ", ", ", &self.fields, " }")?;
        w.write_str(" = ")?;
        self.source.encode(w, 0)
    }
}
