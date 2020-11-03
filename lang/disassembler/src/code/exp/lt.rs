use crate::code::locals::Local;
use crate::code::exp::{ExpLoc, Exp, SourceRange};
use crate::code::translator::Context;
use crate::Encode;
use anyhow::Error;
use std::fmt::Write;

/// Assign local variable expression.
#[derive(Debug)]
pub struct Let<'a> {
    local: Local<'a>,
    exp: ExpLoc<'a>,
}

impl<'a> Let<'a> {
    /// Create a new `Let` expressions.
    pub fn exp(index: u8, ctx: &mut impl Context<'a>) -> Exp<'a> {
        let local = ctx.local_var(index);

        if let Some(exp) = ctx.last_exp() {
            let exp = match exp.as_ref() {
                Exp::Let(_) => ExpLoc::new(ctx.opcode_offset(), Exp::Nop),
                _ => ctx.pop_exp(),
            };
            Exp::Let(Let { local, exp })
        } else {
            Exp::Let(Let {
                local,
                exp: ExpLoc::new(ctx.opcode_offset(), Exp::Nop),
            })
        }
    }
}

impl<'a> SourceRange for Let<'a> {
    fn source_range(&self) -> Option<(usize, usize)> {
        self.exp.source_range()
    }
}

impl<'a> Encode for Let<'a> {
    fn encode<W: Write>(&self, w: &mut W, _: usize) -> Result<(), Error> {
        self.local.write_name(w)?;
        if !self.exp.as_ref().is_nop() {
            w.write_str(" = ")?;
            self.exp.encode(w, 0)?;
        }
        Ok(())
    }
}
