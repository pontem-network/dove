use libra::file_format::*;
use crate::disassembler::imports::Imports;
use crate::disassembler::functions::Param;
use crate::disassembler::{Encode, INDENT};
use anyhow::Error;
use crate::disassembler::generics::Generic;
use crate::disassembler::code::translator::{Translator};
use crate::disassembler::code::locals::{Locals, Local};
use crate::disassembler::code::iter::BytecodeIterator;
use crate::disassembler::code::exp::block::Block;
use crate::disassembler::unit::UnitAccess;
use std::fmt::Write;

/// Function body representation.
pub struct Body<'a> {
    block: Block<'a>,
    locals: Locals<'a>,
}

impl<'a> Body<'a> {
    /// Create a new Body.
    pub fn new<'b>(
        code: &'a CodeUnit,
        ret_len: usize,
        unit: &'a impl UnitAccess,
        params: &'b [Param<'a>],
        imports: &'a Imports,
        type_params: &'b [Generic],
    ) -> Body<'a> {
        let locals = Locals::new(
            params,
            unit,
            imports,
            type_params,
            unit.signature(code.locals),
        );
        let mut iter = BytecodeIterator::new(&code.code);
        let mut translator = Translator::new(
            &mut iter,
            ret_len,
            code.code.len(),
            &locals,
            unit,
            imports,
            type_params,
        );
        translator.translate();

        Body {
            block: Block::new(translator.expressions(), false),
            locals,
        }
    }

    /// Returns body with abort instruction.
    pub fn mock() -> Body<'static> {
        Body {
            block: Block::mock(),
            locals: Locals::mock(),
        }
    }
}

impl<'a> Encode for Body<'a> {
    fn encode<W: Write>(&self, w: &mut W, indent: usize) -> Result<(), Error> {
        let mut new_line = false;
        for local in &self.locals.inner {
            match local {
                Local::Var(var) => {
                    new_line = true;
                    writeln!(w)?;
                    write!(
                        w,
                        "{s:width$}let ",
                        s = "",
                        width = (indent as usize) + INDENT
                    )?;
                    var.encode(w, 0)?;
                    w.write_str(";")?;
                }
                Local::Param(_) => {
                    //no-op
                }
            }
        }

        if new_line {
            writeln!(w)?;
        }

        self.block.encode(w, indent)
    }
}
