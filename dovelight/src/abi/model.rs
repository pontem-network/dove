use move_binary_format::access::ModuleAccess;
use move_binary_format::file_format::{
    Ability, AbilitySet, SignatureToken, StructFieldInformation, StructHandleIndex, Visibility,
};
use move_binary_format::CompiledModule;
use move_core_types::identifier::Identifier;
use move_core_types::language_storage::ModuleId;
use serde::{Deserialize, Serialize};
use move_core_types::account_address::AccountAddress;

#[derive(Debug, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq)]
pub struct ModuleAbi {
    pub id: ModuleId,
    pub friends: Vec<Friend>,
    pub structs: Vec<Struct>,
    pub funcs: Vec<Func>,
}

#[derive(Debug, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq)]
pub struct Struct {
    pub name: Identifier,
    pub type_parameters: Vec<TypeAbilities>,
    pub abilities: TypeAbilities,
    pub fields: Vec<Field>,
}

#[derive(Debug, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq)]
pub struct Field {
    pub name: Identifier,
    pub tp: Type,
}

#[derive(Debug, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq)]
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

#[derive(Debug, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq)]
pub enum TypeAbility {
    Copy,
    Drop,
    Store,
    Key,
}

#[derive(Debug, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq)]
pub enum Type {
    Bool,
    U8,
    U64,
    U128,
    Address,
    Signer,
    Vector(Box<Type>),
    Struct(StructDef),
    Reference(Box<Type>),
    MutableReference(Box<Type>),
    TypeParameter(u16),
}

#[derive(Debug, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq)]
pub struct StructDef {
    pub id: ModuleId,
    pub name: Identifier,
    pub type_parameters: Vec<Type>,
}

#[derive(Debug, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq)]
pub struct Func {
    pub name: Identifier,
    pub visibility: FuncVisibility,
    pub type_parameters: Vec<TypeAbilities>,
    pub parameters: Vec<Type>,
    pub returns: Vec<Type>,
}

#[derive(Debug, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq)]
pub enum FuncVisibility {
    Public,
    Script,
    Friend,
}

impl From<&Visibility> for FuncVisibility {
    fn from(val: &Visibility) -> Self {
        match val {
            Visibility::Private => {
                // not possible.
                FuncVisibility::Public
            }
            Visibility::Public => FuncVisibility::Public,
            Visibility::Script => FuncVisibility::Script,
            Visibility::Friend => FuncVisibility::Friend,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq)]
pub struct Friend {
    pub address: AccountAddress,
    pub name: Identifier,
}

impl From<CompiledModule> for ModuleAbi {
    fn from(module: CompiledModule) -> Self {
        ModuleAbi {
            id: module.self_id(),
            friends: make_friend_abi(&module),
            structs: make_structs_abi(&module),
            funcs: make_func_abi(&module),
        }
    }
}

fn make_structs_abi(module: &CompiledModule) -> Vec<Struct> {
    module
        .struct_defs()
        .iter()
        .map(|sdef| {
            let handle = module.struct_handle_at(sdef.struct_handle);
            let type_parameters = handle
                .type_parameters
                .iter()
                .map(TypeAbilities::from)
                .collect();

            let fields = match &sdef.field_information {
                StructFieldInformation::Native => vec![],
                StructFieldInformation::Declared(defs) => defs
                    .iter()
                    .map(|field| Field {
                        name: module.identifier_at(field.name).to_owned(),
                        tp: make_type(&field.signature.0, module),
                    })
                    .collect(),
            };

            Struct {
                name: module.identifier_at(handle.name).to_owned(),
                type_parameters,
                abilities: TypeAbilities::from(&handle.abilities),
                fields,
            }
        })
        .collect()
}

fn make_type(tok: &SignatureToken, module: &CompiledModule) -> Type {
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

fn make_struct_def(
    idx: StructHandleIndex,
    tps: &[SignatureToken],
    module: &CompiledModule,
) -> StructDef {
    let struct_handle = module.struct_handle_at(idx);
    let struct_module_handle = module.module_handle_at(struct_handle.module);
    let id = module.module_id_for_handle(struct_module_handle);

    StructDef {
        id,
        name: module.identifier_at(struct_handle.name).to_owned(),
        type_parameters: tps.iter().map(|tok| make_type(tok, module)).collect(),
    }
}

fn make_func_abi(module: &CompiledModule) -> Vec<Func> {
    module
        .function_defs()
        .iter()
        .filter(|def| match def.visibility {
            Visibility::Public | Visibility::Script | Visibility::Friend => true,
            Visibility::Private => false,
        })
        .map(|def| {
            let handle = module.function_handle_at(def.function);
            let parameters = &module.signature_at(handle.parameters).0;
            let return_ = &module.signature_at(handle.return_).0;
            Func {
                name: module.identifier_at(handle.name).to_owned(),
                visibility: FuncVisibility::from(&def.visibility),
                type_parameters: handle
                    .type_parameters
                    .iter()
                    .map(TypeAbilities::from)
                    .collect(),
                parameters: parameters.iter().map(|st| make_type(st, module)).collect(),
                returns: return_.iter().map(|st| make_type(st, module)).collect(),
            }
        })
        .collect()
}

fn make_friend_abi(module: &CompiledModule) -> Vec<Friend> {
    module
        .friend_decls()
        .iter()
        .map(|decl| Friend {
            address: *module.address_identifier_at(decl.address),
            name: module.identifier_at(decl.name).to_owned(),
        })
        .collect()
}
