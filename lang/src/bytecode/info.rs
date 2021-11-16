use anyhow::Error;
use move_binary_format::access::{ModuleAccess, ScriptAccess};
use move_binary_format::CompiledModule;
use move_binary_format::file_format::{Ability, AbilitySet, empty_module, SignatureToken, StructHandleIndex, Visibility};
use move_core_types::account_address::AccountAddress;
use crate::bytecode::accessor::Bytecode;

#[derive(Debug)]
pub struct BytecodeInfo {
    bytecode: Bytecode,
}

impl From<Bytecode> for BytecodeInfo {
    fn from(bytecode: Bytecode) -> Self {
        BytecodeInfo {
            bytecode
        }
    }
}

impl BytecodeInfo {
    pub fn serialize(&self, binary: &mut Vec<u8>) -> Result<(), Error> {
        match &self.bytecode {
            Bytecode::Script(_, script, _) => script.serialize(binary),
            Bytecode::Module(module) => module.serialize(binary),
        }
    }

    pub fn is_module(&self) -> bool {
        match &self.bytecode {
            Bytecode::Script(_, _, _) => false,
            Bytecode::Module(_) => true,
        }
    }

    pub fn find_script_function(&self, name: &str) -> Option<Script> {
        match &self.bytecode {
            Bytecode::Script(name, script, module) => {
                let type_parameters = script.type_parameters
                    .iter()
                    .map(TypeAbilities::from)
                    .collect();

                let parameters = script.signature_at(script.parameters)
                    .0
                    .iter()
                    .map(|p| make_type(p, &module))
                    .collect();

                Some(Script {
                    name: name.as_str(),
                    parameters,
                    type_parameters,
                    returns: vec![],
                })
            }
            Bytecode::Module(module) => module
                .function_defs()
                .iter()
                .filter(|def| def.visibility == Visibility::Script)
                .find(|def| {
                    let handle = module.function_handle_at(def.function);
                    module.identifier_at(handle.name).as_str() == name
                })
                .map(|def| {
                    let handle = module.function_handle_at(def.function);
                    let parameters = module
                        .signature_at(handle.parameters)
                        .0
                        .iter()
                        .map(|p| make_type(p, module))
                        .collect();

                    let type_parameters = handle
                        .type_parameters
                        .iter()
                        .map(TypeAbilities::from)
                        .collect();
                    let return_ = &module.signature_at(handle.return_).0;

                    Script {
                        name: module.identifier_at(handle.name).as_str(),
                        parameters,
                        type_parameters,
                        returns: return_.iter().map(|st| make_type(st, module)).collect(),
                    }
                }),
        }
    }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct Script<'a> {
    pub name: &'a str,
    pub parameters: Vec<Type<'a>>,
    pub type_parameters: Vec<TypeAbilities>,
    pub returns: Vec<Type<'a>>,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum Type<'a> {
    Bool,
    U8,
    U64,
    U128,
    Address,
    Signer,
    Vector(Box<Type<'a>>),
    Struct(StructDef<'a>),
    Reference(Box<Type<'a>>),
    MutableReference(Box<Type<'a>>),
    TypeParameter(u16),
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct StructDef<'a> {
    pub address: &'a AccountAddress,
    pub module_name: &'a str,
    pub name: &'a str,
    pub type_parameters: Vec<Type<'a>>,
}

fn make_type<'a>(tok: &'a SignatureToken, module: &'a CompiledModule) -> Type<'a> {
    match tok {
        SignatureToken::Bool => Type::Bool,
        SignatureToken::U8 => Type::U8,
        SignatureToken::U64 => Type::U64,
        SignatureToken::U128 => Type::U128,
        SignatureToken::Address => Type::Address,
        SignatureToken::Signer => Type::Signer,
        SignatureToken::Vector(tp) => Type::Vector(Box::new(make_type(tp, module))),
        SignatureToken::Struct(idx) => Type::Struct(make_struct_def(*idx, &[], module)),
        SignatureToken::StructInstantiation(idx, tps) => {
            Type::Struct(make_struct_def(*idx, tps, module))
        }
        SignatureToken::Reference(rf) => Type::Reference(Box::new(make_type(rf, module))),
        SignatureToken::MutableReference(tp) => {
            Type::MutableReference(Box::new(make_type(tp, module)))
        }
        SignatureToken::TypeParameter(val) => Type::TypeParameter(*val),
    }
}

fn make_struct_def<'a>(
    idx: StructHandleIndex,
    tps: &'a [SignatureToken],
    module: &'a CompiledModule,
) -> StructDef<'a> {
    let struct_handle = module.struct_handle_at(idx);
    let struct_module_handle = module.module_handle_at(struct_handle.module);

    StructDef {
        address: module.address_identifier_at(struct_module_handle.address),
        name: module.identifier_at(struct_handle.name).as_str(),
        type_parameters: tps.iter().map(|tok| make_type(tok, module)).collect(),
        module_name: module.identifier_at(struct_module_handle.name).as_str(),
    }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct TypeAbilities {
    pub abilities: Vec<TypeAbility>,
}

impl From<&AbilitySet> for TypeAbilities {
    fn from(val: &AbilitySet) -> Self {
        TypeAbilities {
            abilities: val
                .into_iter()
                .map(|a| match a {
                    Ability::Copy => TypeAbility::Copy,
                    Ability::Drop => TypeAbility::Drop,
                    Ability::Store => TypeAbility::Store,
                    Ability::Key => TypeAbility::Key,
                })
                .collect(),
        }
    }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum TypeAbility {
    Copy,
    Drop,
    Store,
    Key,
}
