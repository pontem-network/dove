use crate::disassembler::Encode;
use std::fmt::Write;
use anyhow::Error;
use crate::disassembler::code::locals::Local;
use crate::disassembler::code::exp::{Exp, SourceRange};
use crate::disassembler::code::translator::Context;

/// Local variable assignment.
#[derive(Debug)]
pub struct Loc<'a> {
    explicit_keyword: bool,
    access: LocAccess,
    local: Local<'a>,
}

impl<'a> Loc<'a> {
    /// Create a new loc expression.
    pub fn exp(
        explicit_keyword: bool,
        access: LocAccess,
        index: u8,
        ctx: &mut impl Context<'a>,
    ) -> Exp<'a> {
        let local = ctx.local_var(index);
        local.mark_as_used();

        Exp::Local(Loc {
            explicit_keyword,
            access,
            local,
        })
    }
}

impl<'a> SourceRange for Loc<'a> {
    fn source_range(&self) -> Option<(usize, usize)> {
        None
    }
}

impl<'a> Encode for Loc<'a> {
    fn encode<W: Write>(&self, w: &mut W, _: usize) -> Result<(), Error> {
        if self.explicit_keyword {
            w.write_str("(")?;
            self.access.encode(w, 0)?;
            w.write_str(" ")?;
        }

        self.local.write_name(w)?;

        if self.explicit_keyword {
            w.write_str(")")?;
        }
        Ok(())
    }
}

/// Access type.
#[derive(Debug)]
pub enum LocAccess {
    /// Move local.
    Move,
    /// Copy local.
    Copy,
}

impl Encode for LocAccess {
    fn encode<W: Write>(&self, w: &mut W, _indent: usize) -> Result<(), Error> {
        match self {
            LocAccess::Move => {
                w.write_str("move")?;
            }
            LocAccess::Copy => {
                w.write_str("copy")?;
            }
        }
        Ok(())
    }
}
