use crate::generics::Generic;
use crate::Encode;
use std::fmt::Write;
use anyhow::Error;
use crate::imports::{Import, Imports};
use vm::file_format::*;
use crate::unit::UnitAccess;

/// Extract type signature.
pub fn extract_type_signature<'a>(
    unit: &'a impl UnitAccess,
    signature: &'a SignatureToken,
    imports: &'a Imports,
    type_params: &[Generic],
) -> FType<'a> {
    match signature {
        SignatureToken::U8 => FType::Primitive("u8"),
        SignatureToken::Bool => FType::Primitive("bool"),
        SignatureToken::U64 => FType::Primitive("u64"),
        SignatureToken::U128 => FType::Primitive("u128"),
        SignatureToken::Address => FType::Primitive("address"),
        SignatureToken::Signer => FType::Primitive("signer"),

        SignatureToken::Vector(sign) => FType::Vec(Box::new(extract_type_signature(
            unit,
            sign.as_ref(),
            imports,
            type_params,
        ))),
        SignatureToken::Struct(struct_index) => {
            FType::Struct(extract_struct_name(unit, struct_index, imports))
        }
        SignatureToken::StructInstantiation(struct_index, typed) => FType::StructInst(
            extract_struct_name(unit, struct_index, imports),
            typed
                .iter()
                .map(|t| extract_type_signature(unit, t, imports, type_params))
                .collect::<Vec<_>>(),
        ),
        SignatureToken::Reference(sign) => FType::Ref(Box::new(extract_type_signature(
            unit,
            sign.as_ref(),
            imports,
            type_params,
        ))),
        SignatureToken::MutableReference(sign) => FType::RefMut(Box::new(
            extract_type_signature(unit, sign.as_ref(), imports, type_params),
        )),
        SignatureToken::TypeParameter(index) => {
            FType::Generic(type_params[*index as usize].clone())
        }
    }
}

/// Extract struct name.
pub fn extract_struct_name<'a>(
    unit: &'a impl UnitAccess,
    struct_index: &'a StructHandleIndex,
    imports: &'a Imports,
) -> FullStructName<'a> {
    let handler = unit.struct_handle(*struct_index);

    let module_handler = unit.module_handle(handler.module);
    let module_name = unit.identifier(module_handler.name);
    let address = unit.address(module_handler.address);
    let type_name = unit.identifier(handler.name);

    imports
        .get_import(address, module_name)
        .map(|import| FullStructName {
            name: type_name,
            import: Some(import),
        })
        .unwrap_or_else(|| FullStructName {
            name: type_name,
            import: None,
        })
}

/// Type.
#[derive(Debug)]
pub enum FType<'a> {
    /// Generic type.
    Generic(Generic),
    /// Primitive type.
    Primitive(&'static str),
    /// Reference type.
    Ref(Box<FType<'a>>),
    /// Mutable reference type.
    RefMut(Box<FType<'a>>),
    /// Vector type.
    Vec(Box<FType<'a>>),
    /// Struct type.
    Struct(FullStructName<'a>),
    /// Struct instantiation instance.
    StructInst(FullStructName<'a>, Vec<FType<'a>>),
}

impl<'a> Encode for FType<'a> {
    fn encode<W: Write>(&self, w: &mut W, indent: usize) -> Result<(), Error> {
        match self {
            FType::Primitive(name) => {
                w.write_str(name)?;
            }
            FType::Generic(type_param) => {
                type_param.as_name().encode(w, indent)?;
            }
            FType::Ref(t) => {
                w.write_str("&")?;
                t.encode(w, indent)?;
            }
            FType::RefMut(t) => {
                w.write_str("&mut ")?;
                t.encode(w, indent)?;
            }
            FType::Vec(t) => {
                w.write_str("vector<")?;
                t.encode(w, indent)?;
                w.write_str(">")?;
            }
            FType::Struct(name) => {
                name.encode(w, indent)?;
            }
            FType::StructInst(name, generics) => {
                name.encode(w, indent)?;
                if !generics.is_empty() {
                    write!(w, "<")?;
                    for (index, generic) in generics.iter().enumerate() {
                        generic.encode(w, 0)?;
                        if index != generics.len() - 1 {
                            w.write_str(", ")?;
                        }
                    }
                    write!(w, ">")?;
                }
            }
        }

        Ok(())
    }
}

/// Full structure name.
#[derive(Debug)]
pub struct FullStructName<'a> {
    name: &'a str,
    import: Option<Import<'a>>,
}

impl<'a> Encode for FullStructName<'a> {
    fn encode<W: Write>(&self, w: &mut W, indent: usize) -> Result<(), Error> {
        if let Some(import) = &self.import {
            import.encode(w, indent)?;
            w.write_str("::")?;
        }
        w.write_str(self.name)?;
        Ok(())
    }
}
