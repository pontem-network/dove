#[allow(dead_code)]
/// Block of expressions in curly braces.
pub mod block;
/// Branching algorithms.
pub mod branching;
/// Cast.
pub mod cast;
/// Function call.
pub mod function;
/// Load literal or constant.
pub mod ld;
/// Load local variable.
pub mod loc;
/// Local variable assignment.
pub mod lt;
/// Build in operators.
pub mod operators;
/// Struct constructor.
pub mod pack;
/// Return statement.
pub mod ret;
/// Reference.
pub mod rf;
/// Struct destructor.
pub mod unpack;

use crate::Encode;
use vm::file_format::*;
use std::fmt::Write;
use anyhow::Error;
use crate::code::exp::operators::{BinaryOp, Abort, Not};
use crate::code::exp::ret::Ret;
use crate::code::exp::cast::Cast;
use crate::code::exp::ld::Ld;
use crate::code::exp::function::FnCall;
use crate::code::exp::loc::Loc;
use crate::code::exp::lt::Let;
use crate::code::exp::pack::Pack;
use crate::code::exp::unpack::Unpack;
use crate::code::exp::rf::{FieldRef, Ref, Deref, WriteRef};
use crate::code::exp::block::Block;
use itertools::Itertools;

/// Expression wrapper that adds bytecode location of this expression.
#[derive(Debug)]
pub struct ExpLoc<'a> {
    index: usize,
    exp: Box<Exp<'a>>,
}

impl<'a> ExpLoc<'a> {
    /// Create a new `ExpLoc`.
    pub fn new(index: usize, val: Exp<'a>) -> ExpLoc<'a> {
        ExpLoc {
            index,
            exp: Box::new(val),
        }
    }

    /// Returns expression start index in the bytecode.
    pub fn index(&self) -> usize {
        self.index
    }

    /// Returns index range of the expression.
    pub fn range(&self) -> (usize, usize) {
        if let Some((mut l, mut r)) = self.exp.source_range() {
            if self.index < l {
                l = self.index;
            }

            if self.index > r {
                r = self.index;
            }

            (l, r)
        } else {
            (self.index, self.index)
        }
    }

    /// Returns inner expression.
    pub fn val(self) -> Exp<'a> {
        *self.exp
    }
}

impl<'a> SourceRange for ExpLoc<'a> {
    fn source_range(&self) -> Option<(usize, usize)> {
        Some(self.range())
    }
}

impl<'a> SourceRange for &ExpLoc<'a> {
    fn source_range(&self) -> Option<(usize, usize)> {
        Some(self.range())
    }
}

impl<'a> SourceRange for (usize, usize) {
    fn source_range(&self) -> Option<(usize, usize)> {
        Some((self.0, self.1))
    }
}

impl<'a> SourceRange for Option<(usize, usize)> {
    fn source_range(&self) -> Option<(usize, usize)> {
        self.map(|(l, r)| (l, r))
    }
}

/// Range in the bytecode.
pub trait SourceRange {
    /// Returns index range.
    fn source_range(&self) -> Option<(usize, usize)>;
}

impl<'a> AsRef<Exp<'a>> for ExpLoc<'a> {
    fn as_ref(&self) -> &Exp<'a> {
        &self.exp
    }
}

impl<'a> AsMut<Exp<'a>> for ExpLoc<'a> {
    fn as_mut(&mut self) -> &mut Exp<'a> {
        self.exp.as_mut()
    }
}

impl<'a> Encode for ExpLoc<'a> {
    fn encode<W: Write>(&self, w: &mut W, indent: usize) -> Result<(), Error> {
        self.exp.encode(w, indent)
    }
}

/// Move expression.
#[derive(Debug)]
pub enum Exp<'a> {
    /// Abort. (abort)
    Abort(Abort<'a>),
    /// Load literal or constant. (5)
    Ld(Ld),
    /// Disassembler error.
    Error(Bytecode),
    /// Local variable.
    Local(Loc<'a>),
    /// Cast types. (as)
    Cast(Cast<'a>),
    /// Binary operation.
    BinaryOp(BinaryOp<'a>),
    /// Expression in parentheses.
    Basket(ExpLoc<'a>),
    /// Logical negation.
    Not(Not<'a>),
    /// Function call.
    FnCall(FnCall<'a>),
    /// Local variable assignment.
    Let(Let<'a>),
    /// Struct constructor.
    Pack(Pack<'a>),
    /// Struct destructor.
    Unpack(Unpack<'a>),
    /// Return.
    Ret(Ret<'a>),
    /// Structures field access.
    FieldRef(FieldRef<'a>),
    /// Reference.
    Ref(Ref<'a>),
    /// Dereference.
    Deref(Deref<'a>),
    /// Assign reference.
    WriteRef(WriteRef<'a>),
    /// Infinite Loop.
    #[allow(dead_code)]
    Loop(Block<'a>),
    /// While loop.
    While(ExpLoc<'a>, Block<'a>),
    /// If else expression.
    If(ExpLoc<'a>, Block<'a>, Option<Block<'a>>),
    /// Break.
    Break,
    /// Continue.
    Continue,
    /// Nothing.
    Nop,
}

impl<'a> Exp<'a> {
    /// Returns `true` if the current expression is `Exp::Nop`.
    pub fn is_nop(&self) -> bool {
        matches!(self, Exp::Nop)
    }

    /// Returns bytecode range of the curent expression.
    pub fn source_range(&self) -> Option<(usize, usize)> {
        match self {
            Exp::Abort(a) => a.source_range(),
            Exp::Error(_) => None,
            Exp::Ld(ld) => ld.source_range(),
            Exp::Local(l) => l.source_range(),
            Exp::Cast(cast) => cast.source_range(),
            Exp::FnCall(f_call) => f_call.source_range(),
            Exp::BinaryOp(exp) => exp.source_range(),
            Exp::Basket(e) => e.source_range(),
            Exp::Not(e) => e.source_range(),
            Exp::Let(lt) => lt.source_range(),
            Exp::Pack(p) => p.source_range(),
            Exp::Unpack(u) => u.source_range(),
            Exp::Ret(r) => r.source_range(),
            Exp::FieldRef(rf) => rf.source_range(),
            Exp::Ref(r) => r.source_range(),
            Exp::Deref(drf) => drf.source_range(),
            Exp::WriteRef(wr) => wr.source_range(),
            Exp::Loop(b) => b.source_range(),
            Exp::While(e, b) => find_range(vec![e.source_range(), b.source_range()]),
            Exp::If(e, t, f) => find_range(vec![
                e.source_range(),
                t.source_range(),
                f.as_ref().and_then(|f| f.source_range()),
            ]),
            Exp::Break => None,
            Exp::Continue => None,
            Exp::Nop => None,
        }
    }
}

impl<'a> Encode for Exp<'a> {
    fn encode<W: Write>(&self, w: &mut W, indent: usize) -> Result<(), Error> {
        if indent != 0 {
            write!(w, "{s:width$}", s = "", width = indent as usize)?;
        }

        match self {
            Exp::Abort(a) => a.encode(w, indent)?,
            Exp::Local(loc) => loc.encode(w, indent)?,
            Exp::Cast(cast) => cast.encode(w, indent)?,
            Exp::FnCall(call) => call.encode(w, indent)?,
            Exp::Ret(ret) => ret.encode(w, indent)?,
            Exp::Nop => {
                // no-op
            }
            Exp::Error(b) => {
                write!(w, "Err [opcode: {:?}]", b)?;
            }
            Exp::BinaryOp(op) => op.encode(w, indent)?,
            Exp::Basket(inner) => {
                w.write_str("(")?;
                inner.encode(w, indent)?;
                w.write_str(")")?;
            }
            Exp::Not(not) => not.encode(w, indent)?,
            Exp::Let(lt) => lt.encode(w, indent)?,
            Exp::Pack(pack) => pack.encode(w, indent)?,
            Exp::Unpack(unpack) => unpack.encode(w, indent)?,
            Exp::FieldRef(rf) => rf.encode(w, indent)?,
            Exp::Loop(block) => {
                w.write_str("loop ")?;
                block.encode(w, indent)?;
            }
            Exp::If(condition, true_branch, false_branch) => {
                w.write_str("if (")?;
                condition.encode(w, 0)?;
                w.write_str(") ")?;
                true_branch.encode(w, indent)?;
                if let Some(false_branch) = false_branch {
                    w.write_str(" else ")?;
                    false_branch.encode(w, indent)?;
                }
            }
            Exp::Break => {
                w.write_str("break")?;
            }
            Exp::While(condition, body) => {
                w.write_str("while (")?;
                condition.encode(w, 0)?;
                w.write_str(") ")?;
                body.encode(w, indent)?;
            }
            Exp::Ref(rf) => rf.encode(w, indent)?,
            Exp::Deref(drf) => drf.encode(w, indent)?,
            Exp::WriteRef(wr) => wr.encode(w, indent)?,
            Exp::Continue => {
                w.write_str("continue")?;
            }
            Exp::Ld(ld) => ld.encode(w, indent)?,
        }
        Ok(())
    }
}

/// Returns bytecode range of the given expressions.
pub fn find_range<T, S>(range_list: T) -> Option<(usize, usize)>
where
    T: IntoIterator<Item = S>,
    S: SourceRange,
{
    let sorted_index_list = range_list
        .into_iter()
        .map(|p| p.source_range())
        .filter_map(|p| p)
        .flat_map(|p| vec![p.0, p.1])
        .sorted()
        .collect::<Vec<_>>();
    sorted_index_list
        .first()
        .and_then(|f| sorted_index_list.last().map(|l| (*f, *l)))
}
