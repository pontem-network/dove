use crate::code::translator::Context;
use crate::code::exp::{ExpLoc, Exp, SourceRange, find_range};
use crate::Encode;
use anyhow::Error;
use std::fmt::Write;

/// Binary operation.
#[derive(Debug)]
pub struct BinaryOp<'a> {
    /// Left operand.
    pub left: ExpLoc<'a>,
    /// Operator.
    pub sign: Op,
    /// Right operand.
    pub right: ExpLoc<'a>,
}

impl<'a> BinaryOp<'a> {
    /// Create a new `BinaryOp` expressions.
    pub fn exp(sign: Op, ctx: &mut impl Context<'a>) -> Exp<'a> {
        let (left, right) = ctx.pop2_exp();
        fn basket(exp: ExpLoc) -> ExpLoc {
            let index = exp.index();
            match exp.as_ref() {
                Exp::BinaryOp(_) => ExpLoc::new(index, Exp::Basket(exp)),
                _ => exp,
            }
        }

        Exp::BinaryOp(BinaryOp {
            left: basket(left),
            sign,
            right: basket(right),
        })
    }
}

impl<'a> SourceRange for BinaryOp<'a> {
    fn source_range(&self) -> Option<(usize, usize)> {
        find_range(vec![&self.left, &self.right])
    }
}

impl<'a> Encode for BinaryOp<'a> {
    fn encode<W: Write>(&self, w: &mut W, _: usize) -> Result<(), Error> {
        self.left.encode(w, 0)?;
        self.sign.encode(w, 0)?;
        self.right.encode(w, 0)
    }
}

/// Binary operation.
#[derive(Debug)]
pub enum Op {
    /// +
    Add,
    /// -
    Sub,
    /// *
    Mul,
    /// %
    Mod,
    /// /
    Div,
    /// |
    BitOr,
    /// &
    BitAnd,
    /// ^
    Xor,
    /// ||
    Or,
    /// &&
    And,
    /// ==
    Eq,
    /// !=
    Neq,
    /// <
    Lt,
    /// >
    Gt,
    /// <=
    Le,
    /// >=
    Ge,
    /// >>
    Shl,
    /// <<
    Shr,
}

impl Encode for Op {
    fn encode<W: Write>(&self, w: &mut W, _: usize) -> Result<(), Error> {
        w.write_str(match self {
            Op::Add => " + ",
            Op::Sub => " - ",
            Op::Mul => " * ",
            Op::Mod => " % ",
            Op::Div => " / ",
            Op::BitOr => " | ",
            Op::BitAnd => " & ",
            Op::Xor => " ^ ",
            Op::Or => " || ",
            Op::And => " && ",
            Op::Eq => " == ",
            Op::Neq => " != ",
            Op::Lt => " < ",
            Op::Gt => " > ",
            Op::Le => " <= ",
            Op::Ge => " >= ",
            Op::Shl => " << ",
            Op::Shr => " >> ",
        })?;
        Ok(())
    }
}

/// Nop.
pub fn nop<'a>() -> Exp<'a> {
    Exp::Nop
}

/// Pop stack.
pub fn pop<'a>() -> Exp<'a> {
    Exp::Nop
}

/// Abort expression.
#[derive(Debug)]
pub struct Abort<'a> {
    exp: ExpLoc<'a>,
}

impl<'a> Abort<'a> {
    /// Create a new `Abort` expressions.
    pub fn exp(ctx: &mut impl Context<'a>) -> Exp<'a> {
        Exp::Abort(Abort { exp: ctx.pop_exp() })
    }

    /// Returns Abort with the given expression.
    pub fn mock(exp: ExpLoc<'static>) -> Exp<'static> {
        Exp::Abort(Abort { exp })
    }
}

impl<'a> SourceRange for Abort<'a> {
    fn source_range(&self) -> Option<(usize, usize)> {
        self.exp.source_range()
    }
}

impl<'a> Encode for Abort<'a> {
    fn encode<W: Write>(&self, w: &mut W, _: usize) -> Result<(), Error> {
        w.write_str("abort ")?;
        self.exp.encode(w, 0)
    }
}

/// Logical negation.
#[derive(Debug)]
pub struct Not<'a> {
    exp: ExpLoc<'a>,
}

impl<'a> Not<'a> {
    /// Create a new `Not` expressions.
    pub fn exp(ctx: &mut impl Context<'a>) -> Exp<'a> {
        Exp::Not(Not { exp: ctx.pop_exp() })
    }
}

impl<'a> SourceRange for Not<'a> {
    fn source_range(&self) -> Option<(usize, usize)> {
        self.exp.source_range()
    }
}

impl<'a> Encode for Not<'a> {
    fn encode<W: Write>(&self, w: &mut W, _: usize) -> Result<(), Error> {
        w.write_str("!")?;
        self.exp.encode(w, 0)
    }
}
