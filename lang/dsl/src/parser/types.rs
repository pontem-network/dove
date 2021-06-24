use move_core_types::language_storage::StructTag;
use move_lang::parser::ast::{Use, ModuleAccess, Type};
use move_lang::shared::{Address, Name};
use move_ir_types::location::{Spanned, Loc};

#[derive(Debug, PartialEq)]
pub enum Value_ {
    Var(String),
    Address(Address),
    Bool(bool),
    Num(u128),
    Struct(Struct),
    Bytes(Vec<u8>),
    VecNum(Vec<u128>),
    VecAddr(Vec<Address>),
    VecStruct(Vec<Struct>),
    EmptyVector(),
}

pub type Value = Spanned<Value_>;

#[derive(Debug, PartialEq)]
pub struct Struct {
    fields: Vec<Value>,
}

#[derive(Debug, PartialEq)]
pub struct Var {
    pub name: Name,
    pub value: Value,
}

#[derive(Debug, PartialEq)]
pub struct ResourceStore {
    pub address: Address,
    pub resource_tag: StructTag,
    pub value: Value,
}

#[derive(Debug, PartialEq)]
pub struct Call {
    pub name: ModuleAccess,
    pub t_params: Option<Vec<Type>>,
    pub params: Vec<Value>,
}

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Use(Use),
    Var(Var),
    Call(Call),
    Store(ResourceStore),
}

#[derive(Debug, PartialEq)]
pub struct Ast {
    pub loc: Loc,
    pub instructions: Vec<(Loc, Instruction)>,
}