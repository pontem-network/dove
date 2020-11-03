use crate::code::exp::{Exp, ExpLoc};
use crate::imports::{Imports, Import};
use crate::generics::Generic;
use crate::types::{extract_type_signature, FType};

use crate::libra::file_format::*;

use crate::code::locals::{Locals, Local};
use crate::code::iter::BytecodeIterator;
use crate::code::exp::operators::{BinaryOp, Op, pop, nop, Abort, Not};
use crate::code::exp::ret::Ret;
use crate::code::exp::cast::{CastType, Cast};
use crate::code::exp::ld::Ld;
use crate::code::exp::function::{FnCall, BuildIn};
use crate::code::exp::loc::{Loc, LocAccess};
use crate::code::exp::lt::Let;
use crate::code::exp::rf::{FieldRef, Ref, Deref, WriteRef};
use crate::code::exp::pack::{PackField, Pack};
use crate::code::exp::unpack::Unpack;
use crate::code::exp::branching::{br_true, br_false, br};
use crate::unit::UnitAccess;

/// Transaction context.
/// Provides functions for bytecode transactions.
pub trait Context<'a> {
    /// Removes the last element from a expression list and returns it, or [`Exp::Non`] if it
    /// is empty.
    fn pop_exp(&mut self) -> ExpLoc<'a>;

    /// Returns reference to the last element of a expression list or [`None`] if it
    /// is empty.
    fn last_exp(&self) -> Option<&ExpLoc<'a>>;

    /// Removes the two last elements from a expression list and returns it, or [`Exp::Non`] if it
    /// is empty.
    fn pop2_exp(&mut self) -> (ExpLoc<'a>, ExpLoc<'a>);

    /// Removes the `exp_count` last elements from a expression list and returns it.
    fn pop_exp_vec(&mut self, exp_count: usize) -> Vec<ExpLoc<'a>>;

    /// Returns module Import by its handle reference.
    fn module_import(&self, module: &ModuleHandle) -> Option<Import<'a>>;

    /// Extracts signature by its index.
    fn extract_signature(&self, type_params: Option<&SignatureIndex>) -> Vec<FType<'a>>;

    /// Returns local variable by its index.
    fn local_var(&self, index: u8) -> Local<'a>;

    /// Returns current bytecode offset.
    fn opcode_offset(&self) -> usize;

    /// Returns struct fields by its definition.
    fn pack_fields(&mut self, def: &StructDefinition) -> Vec<PackField<'a>>;

    /// Translates next `block_size` bytecode instructions and returns it.
    fn translate_block(&mut self, block_size: usize) -> Vec<ExpLoc<'a>>;

    /// Returns next bytecode instruction and updates bytecode iterator state.
    fn next_opcode(&mut self) -> Option<&Bytecode>;

    /// Wraps the given expression at the current location.
    fn loc(&self, exp: Exp<'a>) -> ExpLoc<'a>;

    /// Returns the bytecode instruction by relative index.
    fn opcode_by_relative_offset(&self, offset: isize) -> &Bytecode;

    /// Returns the bytecode instruction by absolute index.
    fn opcode_by_absolute_offset(&self, offset: usize) -> &Bytecode;

    /// Returns the last bytecode offset of the current context.
    fn end_offset(&self) -> usize;

    /// Returns remaining bytecode instructions.
    fn remaining_code(&self) -> &[Bytecode];

    /// Returns error expression.
    fn err(&self) -> Exp<'a>;
}

/// Bytecode translator.
pub struct Translator<'a, 'b, 'c, A>
where
    A: UnitAccess,
{
    expressions: Vec<ExpLoc<'a>>,
    locals: &'b Locals<'a>,
    unit: &'a A,
    imports: &'a Imports<'a>,
    type_params: &'b [Generic],
    opcode_iter: &'c mut BytecodeIterator<'a>,
    end_offset: usize,
    ret_len: usize,
}

impl<'a, 'b, 'c, A> Translator<'a, 'b, 'c, A>
where
    A: UnitAccess,
{
    /// Creates a new translator.
    pub fn new(
        opcode_iter: &'c mut BytecodeIterator<'a>,
        ret_len: usize,
        opcodes_count: usize,
        locals: &'b Locals<'a>,
        unit: &'a A,
        imports: &'a Imports<'a>,
        type_params: &'b [Generic],
    ) -> Translator<'a, 'b, 'c, A> {
        let start_offset = opcode_iter.index();
        Translator {
            opcode_iter,
            expressions: vec![],
            locals,
            unit,
            imports,
            type_params,
            ret_len,
            end_offset: start_offset + opcodes_count,
        }
    }

    /// Translates bytecode instructions.
    pub fn translate(&mut self) {
        loop {
            if self.end_offset > self.opcode_iter.index() {
                if let Some(opcode) = self.opcode_iter.next() {
                    let exp = self.next_exp(opcode);
                    self.expressions
                        .push(ExpLoc::new(self.opcode_iter.index(), exp));
                } else {
                    break;
                }
            } else {
                break;
            }
        }
    }

    fn next_exp(&mut self, opcode: &Bytecode) -> Exp<'a> {
        match opcode {
            Bytecode::Pop => pop(),
            Bytecode::Not => Not::exp(self),
            Bytecode::Abort => Abort::exp(self),
            Bytecode::Add => BinaryOp::exp(Op::Add, self),
            Bytecode::Sub => BinaryOp::exp(Op::Sub, self),
            Bytecode::Mul => BinaryOp::exp(Op::Mul, self),
            Bytecode::Mod => BinaryOp::exp(Op::Mod, self),
            Bytecode::Div => BinaryOp::exp(Op::Div, self),
            Bytecode::BitOr => BinaryOp::exp(Op::BitOr, self),
            Bytecode::BitAnd => BinaryOp::exp(Op::BitAnd, self),
            Bytecode::Xor => BinaryOp::exp(Op::Xor, self),
            Bytecode::Or => BinaryOp::exp(Op::Or, self),
            Bytecode::And => BinaryOp::exp(Op::And, self),
            Bytecode::Eq => BinaryOp::exp(Op::Eq, self),
            Bytecode::Neq => BinaryOp::exp(Op::Neq, self),
            Bytecode::Lt => BinaryOp::exp(Op::Lt, self),
            Bytecode::Gt => BinaryOp::exp(Op::Gt, self),
            Bytecode::Le => BinaryOp::exp(Op::Le, self),
            Bytecode::Ge => BinaryOp::exp(Op::Ge, self),
            Bytecode::Shl => BinaryOp::exp(Op::Shl, self),
            Bytecode::Shr => BinaryOp::exp(Op::Shr, self),
            Bytecode::Nop => nop(),
            Bytecode::Ret => Ret::exp(self.ret_len, self),
            Bytecode::CastU8 => Cast::exp(CastType::U8, self),
            Bytecode::CastU64 => Cast::exp(CastType::U64, self),
            Bytecode::CastU128 => Cast::exp(CastType::U128, self),
            Bytecode::LdU8(val) => Ld::u8(*val),
            Bytecode::LdU64(val) => Ld::u64(*val),
            Bytecode::LdU128(val) => Ld::u128(*val),
            Bytecode::LdConst(index) => Ld::ld_const(*index, self.unit),
            Bytecode::LdTrue => Ld::bool(true),
            Bytecode::LdFalse => Ld::bool(false),
            Bytecode::Call(index) => FnCall::plain(index, None, self, self.unit),
            Bytecode::CallGeneric(index) => {
                let inst = self.unit.function_instantiation(*index);
                FnCall::plain(&inst.handle, Some(&inst.type_parameters), self, self.unit)
            }
            Bytecode::Exists(index) => {
                FnCall::build_in(BuildIn::Exists, index, None, self, self.unit)
            }
            Bytecode::ExistsGeneric(index) => self.build_in(*index, BuildIn::Exists, opcode),
            Bytecode::MoveFrom(index) => {
                FnCall::build_in(BuildIn::MoveFrom, index, None, self, self.unit)
            }
            Bytecode::MoveFromGeneric(index) => self.build_in(*index, BuildIn::MoveFrom, opcode),
            Bytecode::MoveTo(index) => {
                FnCall::build_in(BuildIn::MoveTo, index, None, self, self.unit)
            }
            Bytecode::MoveToGeneric(index) => self.build_in(*index, BuildIn::MoveTo, opcode),
            Bytecode::ImmBorrowGlobal(index) => {
                FnCall::build_in(BuildIn::BorrowGlobal, index, None, self, self.unit)
            }
            Bytecode::ImmBorrowGlobalGeneric(index) => {
                self.build_in(*index, BuildIn::BorrowGlobal, opcode)
            }
            Bytecode::MutBorrowGlobal(index) => {
                FnCall::build_in(BuildIn::BorrowGlobalMut, index, None, self, self.unit)
            }
            Bytecode::MutBorrowGlobalGeneric(index) => {
                self.build_in(*index, BuildIn::BorrowGlobalMut, opcode)
            }
            Bytecode::CopyLoc(index) => Loc::exp(false, LocAccess::Copy, *index, self),
            Bytecode::MoveLoc(index) => Loc::exp(false, LocAccess::Move, *index, self),
            Bytecode::StLoc(index) => Let::exp(*index, self),
            Bytecode::Pack(index) => Pack::exp(index, None, self, self.unit),
            Bytecode::PackGeneric(index) => {
                if let Some(inst) = self.unit.struct_def_instantiation(*index) {
                    Pack::exp(&inst.def, Some(&inst.type_parameters), self, self.unit)
                } else {
                    Exp::Error(opcode.clone())
                }
            }
            Bytecode::Unpack(def) => Unpack::exp(def, None, self, self.unit),
            Bytecode::UnpackGeneric(index) => {
                if let Some(inst) = self.unit.struct_def_instantiation(*index) {
                    Unpack::exp(&inst.def, Some(&inst.type_parameters), self, self.unit)
                } else {
                    Exp::Error(opcode.clone())
                }
            }
            Bytecode::MutBorrowField(index) => FieldRef::exp(index, true, self, self.unit),
            Bytecode::MutBorrowFieldGeneric(index) => {
                if let Some(field_index) = self.unit.field_instantiation(*index) {
                    FieldRef::exp(&field_index.handle, true, self, self.unit)
                } else {
                    Exp::Error(opcode.clone())
                }
            }
            Bytecode::ImmBorrowField(index) => FieldRef::exp(index, false, self, self.unit),
            Bytecode::ImmBorrowFieldGeneric(index) => {
                if let Some(field_index) = self.unit.field_instantiation(*index) {
                    FieldRef::exp(&field_index.handle, false, self, self.unit)
                } else {
                    Exp::Error(opcode.clone())
                }
            }
            Bytecode::FreezeRef => self.pop_exp().val(),
            Bytecode::MutBorrowLoc(index) => Ref::exp(*index, true, self),
            Bytecode::ImmBorrowLoc(index) => Ref::exp(*index, false, self),
            Bytecode::ReadRef => Deref::exp(self),
            Bytecode::WriteRef => WriteRef::exp(self),

            Bytecode::BrTrue(true_offset) => br_true(*true_offset as usize, self),
            Bytecode::BrFalse(offset) => br_false(*offset as usize, self),
            Bytecode::Branch(offset) => br(*offset as usize, self),
        }
    }

    /// Returns transaction results.
    pub fn expressions(self) -> Vec<ExpLoc<'a>> {
        self.expressions
    }

    fn build_in(
        &mut self,
        index: StructDefInstantiationIndex,
        kind: BuildIn,
        opcode: &Bytecode,
    ) -> Exp<'a> {
        if let Some(def) = self.unit.struct_def_instantiation(index) {
            FnCall::build_in(kind, &def.def, Some(&def.type_parameters), self, self.unit)
        } else {
            Exp::Error(opcode.clone())
        }
    }

    #[allow(dead_code)]
    fn take_by_offset(&mut self, offset: usize) -> Vec<ExpLoc<'a>> {
        let mut buffer = Vec::new();
        while let Some(exp) = self.expressions.last() {
            if exp.index() >= offset {
                if let Some(exp) = self.expressions.pop() {
                    buffer.insert(0, exp);
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        buffer
    }
}

impl<'a, 'b, 'c, A> Context<'a> for Translator<'a, 'b, 'c, A>
where
    A: UnitAccess,
{
    fn pop_exp(&mut self) -> ExpLoc<'a> {
        self.expressions
            .pop()
            .unwrap_or_else(|| ExpLoc::new(0, Exp::Nop))
    }

    fn last_exp(&self) -> Option<&ExpLoc<'a>> {
        self.expressions.last()
    }

    fn pop2_exp(&mut self) -> (ExpLoc<'a>, ExpLoc<'a>) {
        let second = self.pop_exp();
        let first = self.pop_exp();
        (first, second)
    }

    fn pop_exp_vec(&mut self, exp_count: usize) -> Vec<ExpLoc<'a>> {
        let len = self.expressions.len();
        if exp_count > len {
            let mut res = self.expressions.split_off(0);
            for _ in 0..exp_count - len {
                res.push(self.loc(Exp::Nop))
            }
            res
        } else {
            self.expressions
                .split_off(self.expressions.len() - exp_count)
                .into_iter()
                .collect()
        }
    }

    fn module_import(&self, module: &ModuleHandle) -> Option<Import<'a>> {
        let module_name = self.unit.identifier(module.name);
        let module_address = self.unit.address(module.address);
        self.imports.get_import(module_address, module_name)
    }

    fn extract_signature(&self, type_params: Option<&SignatureIndex>) -> Vec<FType<'a>> {
        type_params
            .map(|index| {
                self.unit
                    .signature(*index)
                    .0
                    .iter()
                    .map(|t| extract_type_signature(self.unit, t, self.imports, self.type_params))
                    .collect::<Vec<_>>()
            })
            .unwrap_or_else(Vec::new)
    }

    fn local_var(&self, index: u8) -> Local<'a> {
        self.locals.get(index as usize)
    }

    fn opcode_offset(&self) -> usize {
        self.opcode_iter.index()
    }

    fn pack_fields(&mut self, def: &StructDefinition) -> Vec<PackField<'a>> {
        match &def.field_information {
            StructFieldInformation::Native => vec![],
            StructFieldInformation::Declared(fields) => self
                .pop_exp_vec(fields.len())
                .into_iter()
                .zip(fields)
                .map(|(exp, def)| PackField {
                    name: self.unit.identifier(def.name),
                    value: exp,
                })
                .collect(),
        }
    }

    fn translate_block(&mut self, block_size: usize) -> Vec<ExpLoc<'a>> {
        let mut translator = Translator::new(
            self.opcode_iter,
            self.ret_len,
            block_size,
            self.locals,
            self.unit,
            self.imports,
            self.type_params,
        );
        translator.translate();
        translator.expressions
    }

    fn next_opcode(&mut self) -> Option<&Bytecode> {
        self.opcode_iter.next()
    }

    fn loc(&self, exp: Exp<'a>) -> ExpLoc<'a> {
        ExpLoc::new(self.opcode_offset(), exp)
    }

    fn opcode_by_relative_offset(&self, offset: isize) -> &Bytecode {
        self.opcode_iter.by_relative(offset)
    }

    fn opcode_by_absolute_offset(&self, offset: usize) -> &Bytecode {
        self.opcode_iter.absolute(offset)
    }

    fn end_offset(&self) -> usize {
        self.end_offset
    }

    fn remaining_code(&self) -> &[Bytecode] {
        self.opcode_iter.remaining_code()
    }

    fn err(&self) -> Exp<'a> {
        Exp::Error(self.opcode_iter.by_relative(0).clone())
    }
}
