mod algorithms;

use crate::disassembler::code::exp::Exp;
use crate::disassembler::code::translator::Context;
use libra::file_format::*;
use crate::disassembler::code::exp::block::Block;

/// Handles `BrTrue` instruction.
pub fn br_true<'a>(true_offset: usize, ctx: &mut impl Context<'a>) -> Exp<'a> {
    let next = ctx.opcode_by_relative_offset(1).clone();
    let false_offset_start = if let Bytecode::Branch(false_offset) = next {
        ctx.next_opcode();
        false_offset as usize
    } else {
        if ctx.opcode_offset() == ctx.end_offset() {
            return Exp::If(ctx.pop_exp(), Block::new(vec![], true), None);
        }

        let false_branch_len = true_offset - ctx.opcode_offset() - 1;
        let block = ctx.translate_block(false_branch_len);

        let false_branch = if block.is_empty() {
            None
        } else {
            Some(Block::new(block, true))
        };

        let block = ctx.translate_block(ctx.remaining_code().len());
        return Exp::If(ctx.pop_exp(), Block::new(block, true), false_branch);
    };

    let mut false_offset_end = None;

    let true_branch = if true_offset < ctx.opcode_offset() {
        vec![ctx.loc(Exp::Continue)]
    } else {
        let branch_len = false_offset_start - true_offset;
        let mut block = ctx.translate_block(branch_len as usize);
        if let Some(last) = block.last_mut() {
            match last.as_mut() {
                Exp::Ret(r) => {
                    r.explicit_keyword = true;
                }
                Exp::Continue => {
                    if let Some(offset) =
                        ctx.opcode_by_absolute_offset(ctx.opcode_offset()).offset()
                    {
                        if let Some(last) = ctx.last_exp() {
                            if last.range().0 == *offset as usize {
                                block.remove(block.len() - 1);
                                return Exp::While(ctx.pop_exp(), Block::new(block, true));
                            }
                        }
                    }
                }
                Exp::Nop => {
                    if let Bytecode::Branch(offset) =
                        ctx.opcode_by_absolute_offset(ctx.opcode_offset())
                    {
                        let offset = *offset as usize;
                        if offset > ctx.opcode_offset() {
                            false_offset_end = Some(offset);
                        }
                    } else {
                    }
                }
                _ => {}
            }
        }
        block
    };

    let false_branch = if let Some(false_offset_end) = false_offset_end {
        if (false_offset_start as usize) < ctx.opcode_offset() {
            Some(Block::new(vec![ctx.loc(Exp::Continue)], true))
        } else {
            let branch_len = false_offset_end - false_offset_start;
            if branch_len == 0 {
                None
            } else {
                let block = ctx.translate_block(branch_len as usize);
                if block.is_empty() {
                    None
                } else {
                    Some(Block::new(block, true))
                }
            }
        }
    } else {
        None
    };

    Exp::If(ctx.pop_exp(), Block::new(true_branch, true), false_branch)
}

/// Handles `BrFalse` instruction.
pub fn br_false<'a>(index: usize, _ctx: &mut impl Context<'a>) -> Exp<'a> {
    Exp::Error(Bytecode::BrFalse(index as u16))
}

/// Handles `Branch` instruction.
pub fn br<'a>(offset: usize, ctx: &mut impl Context<'a>) -> Exp<'a> {
    if offset > ctx.opcode_offset() {
        Exp::Nop
    } else {
        Exp::Continue
    }
}
