use crate::code::exp::{ExpLoc, Exp, SourceRange, find_range};
use crate::Encode;
use anyhow::Error;
use std::fmt::Write;
use crate::code::translator::Context;
use move_binary_format::file_format::*;
use crate::code::locals::Local;
use crate::unit::UnitAccess;

/// Field reference.
#[derive(Debug)]
pub struct FieldRef<'a> {
    is_mut: bool,
    field_name: &'a str,
    instance: ExpLoc<'a>,
}

impl<'a> FieldRef<'a> {
    /// Field reference.
    pub fn exp(
        index: &FieldHandleIndex,
        is_mut: bool,
        ctx: &mut impl Context<'a>,
        unit: &'a impl UnitAccess,
    ) -> Exp<'a> {
        if let Some((field, def)) = unit
            .field_handle(*index)
            .and_then(|field| unit.struct_def(field.owner).map(|def| (field, def)))
        {
            match &def.field_information {
                StructFieldInformation::Declared(fields) => {
                    if let Some(field) = fields.get(field.field as usize) {
                        let field_name = unit.identifier(field.name);
                        Exp::FieldRef(FieldRef {
                            is_mut,
                            field_name,
                            instance: ctx.pop_exp(),
                        })
                    } else {
                        Exp::Nop
                    }
                }
                StructFieldInformation::Native => Exp::Nop,
            }
        } else {
            ctx.err()
        }
    }
}

impl<'a> SourceRange for FieldRef<'a> {
    fn source_range(&self) -> Option<(usize, usize)> {
        self.instance.source_range()
    }
}

impl<'a> Encode for FieldRef<'a> {
    fn encode<W: Write>(&self, w: &mut W, _: usize) -> Result<(), Error> {
        match self.instance.as_ref() {
            Exp::Ref(_) => {
                //no-op
            }
            _ => {
                w.write_str("&")?;
                if self.is_mut {
                    w.write_str("mut ")?;
                }
            }
        }
        self.instance.encode(w, 0)?;
        w.write_str(".")?;
        w.write_str(self.field_name)?;
        Ok(())
    }
}

/// Reference.
#[derive(Debug)]
pub struct Ref<'a> {
    is_mut: bool,
    local: Local<'a>,
}

impl<'a> Ref<'a> {
    /// Create a new reference expression.
    pub fn exp(index: u8, is_mut: bool, ctx: &mut impl Context<'a>) -> Exp<'a> {
        let local = ctx.local_var(index);
        local.mark_as_used();

        Exp::Ref(Ref { is_mut, local })
    }
}

impl<'a> SourceRange for Ref<'a> {
    fn source_range(&self) -> Option<(usize, usize)> {
        None
    }
}

impl<'a> Encode for Ref<'a> {
    fn encode<W: Write>(&self, w: &mut W, _: usize) -> Result<(), Error> {
        w.write_str("&")?;
        if self.is_mut {
            w.write_str("mut ")?;
        }
        self.local.encode(w, 0)?;
        Ok(())
    }
}

///Dereference expressions.
#[derive(Debug)]
pub struct Deref<'a> {
    exp: ExpLoc<'a>,
}

impl<'a> Deref<'a> {
    /// Create a new `Deref` expressions.
    pub fn exp(ctx: &mut impl Context<'a>) -> Exp<'a> {
        Exp::Deref(Deref { exp: ctx.pop_exp() })
    }
}

impl<'a> SourceRange for Deref<'a> {
    fn source_range(&self) -> Option<(usize, usize)> {
        self.exp.source_range()
    }
}

impl<'a> Encode for Deref<'a> {
    fn encode<W: Write>(&self, w: &mut W, _: usize) -> Result<(), Error> {
        w.write_str("*")?;
        self.exp.encode(w, 0)?;
        Ok(())
    }
}

/// Write reference representation.
#[derive(Debug)]
pub struct WriteRef<'a> {
    val: ExpLoc<'a>,
    val_ref: ExpLoc<'a>,
}

impl<'a> WriteRef<'a> {
    /// Create a new `WriteRef` expressions.
    pub fn exp(ctx: &mut impl Context<'a>) -> Exp<'a> {
        let (val, val_ref) = ctx.pop2_exp();
        Exp::WriteRef(WriteRef { val, val_ref })
    }
}

impl<'a> SourceRange for WriteRef<'a> {
    fn source_range(&self) -> Option<(usize, usize)> {
        find_range(vec![&self.val, &self.val_ref])
    }
}

impl<'a> Encode for WriteRef<'a> {
    fn encode<W: Write>(&self, w: &mut W, _: usize) -> Result<(), Error> {
        w.write_str("*")?;
        self.val_ref.encode(w, 0)?;
        w.write_str(" = ")?;
        self.val.encode(w, 0)
    }
}
