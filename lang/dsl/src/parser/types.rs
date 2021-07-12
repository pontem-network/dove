use move_lang::parser::ast::{Use, ModuleAccess, Type};
use move_lang::shared::{Address, Name};
use move_ir_types::location::{Spanned, Loc};
use move_core_types::language_storage::TypeTag;

#[derive(Debug, PartialEq)]
pub enum Value_ {
    Var(String),
    Address(Address),
    Bool(bool),
    Num(u128),
    Struct(Struct),
    Bytes(Vec<u8>),
    Vec(Vec<Value>),
}

pub type Value = Spanned<Value_>;

#[derive(Debug, PartialEq)]
pub struct Struct {
    pub fields: Vec<(Name, Value)>,
}

#[derive(Debug, PartialEq)]
pub struct Var {
    pub name: Name,
    pub value: Value,
}

#[derive(Debug, PartialEq)]
pub struct ResourceStore {
    pub address: Address,
    pub tp: Type,
    pub value: Option<Value>,
}

#[derive(Debug, PartialEq)]
pub struct Call {
    pub name: ModuleAccess,
    pub t_params: Option<Vec<Spanned<TypeTag>>>,
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
