use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use anyhow::Error;
use codespan::Span;
use move_core_types::account_address::AccountAddress;
use move_lang::{parse_file, MatchedFileCommentMap};
use move_lang::errors::{FilesSourceText, output_errors};
use move_lang::parser::ast::{
    Definition, Script, Type, Type_, NameAccessChain_, LeadingNameAccess_, ModuleDefinition,
    ModuleMember, Visibility as AstVisibility, StructFields,
};

use crate::compiler::dialects::Dialect;
use crate::compiler::preprocessor::BuilderPreprocessor;
use codespan_reporting::term::termcolor::{StandardStream, ColorChoice};
use ir_to_bytecode_syntax::syntax::leak_str;
use move_core_types::identifier::Identifier;
use move_lang::shared::Identifier as Iden;

pub mod spanned;
use spanned::Spanned;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub enum Unit {
    Module(ModuleMeta),
    Script(FuncMeta),
}
impl Unit {
    /// Converts from Unit to Option<ModuleMeta>.
    pub fn module(&self) -> Option<&ModuleMeta> {
        match self {
            Unit::Module(module) => Some(module),
            _ => None,
        }
    }
    /// Converts from Unit to Option<FuncMeta>.
    pub fn script(&self) -> Option<&FuncMeta> {
        match self {
            Unit::Script(script) => Some(script),
            _ => None,
        }
    }
}
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub struct FuncMeta_ {
    pub name: Identifier,
    pub visibility: Visibility,
    pub type_parameters: Vec<String>,
    pub parameters: Vec<(String, String)>,
    pub doc: Option<String>,
}

pub type FuncMeta = Spanned<FuncMeta_>;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub struct StructMeta_ {
    /// struct Name { ... }
    pub name: Identifier,
    /// struct Name has ability1, ability2 { ... }
    pub abilities: Vec<String>,
    /// struct Name<Type1: copy + drop, Type2> { ... }
    pub type_parameters: Vec<(String, Vec<String>)>,
    /// struct Example {
    ///     /// doc for field
    ///     field1: type,
    ///     field2: u8,
    ///     field3: u64,
    ///     ...
    /// }
    pub fields: Vec<StructMetaField>,
    /// /// Doc text
    /// struct Example {
    ///     ...
    /// }
    pub doc: Option<String>,
}

pub type StructMeta = Spanned<StructMeta_>;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub struct StructMetaField_ {
    pub name: Identifier,
    pub type_field: String,
    pub doc: Option<String>,
}

pub type StructMetaField = Spanned<StructMetaField_>;

fn processing_docs(docs: &mut MatchedFileCommentMap) {
    docs.iter_mut()
        .for_each(|(_, doc)| *doc = doc.trim().to_string());
    docs.retain(|_, doc| !doc.is_empty());
}

pub fn script_meta(
    script_path: &str,
    dialect: &dyn Dialect,
    sender: &str,
) -> Result<Vec<FuncMeta>, Error> {
    parse(script_path, dialect, sender).map(|list| {
        list.iter()
            .filter_map(|unit| unit.script())
            .cloned()
            .collect::<Vec<FuncMeta>>()
    })
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub enum Visibility {
    Public,
    Script,
    Friend,
    Internal,
}

impl Visibility {
    pub fn is_script(&self) -> bool {
        matches!(self, Visibility::Script)
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub struct ModuleMeta_ {
    pub address: AccountAddress,
    pub name: Identifier,
    pub funs: Vec<FuncMeta>,
    pub structs: Vec<StructMeta>,
    pub doc: Option<String>,
}

pub type ModuleMeta = Spanned<ModuleMeta_>;

pub fn module_meta(
    module_path: &str,
    dialect: &dyn Dialect,
    sender: &str,
) -> Result<Vec<ModuleMeta>, Error> {
    parse(module_path, dialect, sender).map(|list| {
        list.iter()
            .filter_map(|unit| unit.module())
            .cloned()
            .collect()
    })
}

/// Get metadata from move file
pub fn parse(script_path: &str, dialect: &dyn Dialect, sender: &str) -> Result<Vec<Unit>, Error> {
    let mut preprocessor = BuilderPreprocessor::new(dialect, sender);
    let mut files: FilesSourceText = HashMap::new();
    let (defs, mut docs, errors) =
        parse_file(&mut files, leak_str(script_path), &mut preprocessor)?;

    if !errors.is_empty() {
        let errors = preprocessor.transform(errors);
        let mut writer = StandardStream::stderr(ColorChoice::Auto);
        output_errors(&mut writer, files, errors);
        anyhow::bail!("Could not compile scripts '{}'.", script_path);
    }

    let mut list_unit = Vec::new();
    processing_docs(&mut docs);

    for def in defs {
        match def {
            Definition::Module(module) => {
                list_unit.push(Unit::Module(parse_module_definition(module, &docs, None)?));
            }
            Definition::Address(def) => {
                let addr = match def.addr.value {
                    LeadingNameAccess_::AnonymousAddress(addr) => {
                        Some(AccountAddress::new(addr.into_bytes()))
                    }
                    LeadingNameAccess_::Name(_) => def
                        .addr_value
                        .map(|addr| AccountAddress::new(addr.value.into_bytes())),
                };
                for def in def.modules {
                    list_unit.push(Unit::Module(parse_module_definition(def, &docs, addr)?));
                }
            }
            Definition::Script(script) => {
                list_unit.push(Unit::Script(make_script_meta(script, &docs)?))
            }
        }
    }

    Ok(list_unit)
}

fn make_script_meta(script: Script, docs: &MatchedFileCommentMap) -> Result<FuncMeta, Error> {
    let func = script.function;

    let type_parameters = func
        .signature
        .type_parameters
        .into_iter()
        .map(|tp| tp.0.value)
        .collect();
    let parameters = func
        .signature
        .parameters
        .into_iter()
        .map(|(var, tp)| (var.0.value, extract_type_name(tp)))
        .collect();
    let span: Span = func.loc.span.into();
    Ok(Spanned::new(
        func.loc.into(),
        FuncMeta_ {
            name: Identifier::new(func.name.0.value)?,
            visibility: Visibility::Script,
            type_parameters,
            parameters,
            doc: docs.get(&span.start()).cloned(),
        },
    ))
}

fn extract_type_name(tp: Type) -> String {
    match tp.value {
        Type_::Apply(name, types) => {
            let mut tp = match name.value {
                NameAccessChain_::One(name) => name.value,
                NameAccessChain_::Two(access, name) => {
                    format!("{}::{}", access.value, name.value)
                }
                NameAccessChain_::Three(access, name) => {
                    let (address, m_name) = access.value;
                    format!("{}::{}::{}", address, m_name, name.value)
                }
            };
            if !types.is_empty() {
                tp.push('<');
                tp.push_str(
                    &types
                        .into_iter()
                        .map(extract_type_name)
                        .collect::<Vec<_>>()
                        .join(", "),
                );
                tp.push('>');
            }
            tp
        }
        Type_::Ref(is_mut, tp) => {
            if is_mut {
                format!("&mut {}", extract_type_name(*tp))
            } else {
                format!("&{}", extract_type_name(*tp))
            }
        }
        Type_::Fun(types, tp) => {
            format!(
                "({}):{}",
                types
                    .into_iter()
                    .map(extract_type_name)
                    .collect::<Vec<_>>()
                    .join(", "),
                extract_type_name(*tp)
            )
        }
        Type_::Unit => "()".to_owned(),
        Type_::Multiple(types) => {
            format!(
                "({})",
                types
                    .into_iter()
                    .map(extract_type_name)
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        }
    }
}

fn parse_module_definition(
    module: ModuleDefinition,
    docs: &MatchedFileCommentMap,
    adds: Option<AccountAddress>,
) -> Result<ModuleMeta, Error> {
    let ModuleDefinition {
        address,
        name,
        members,
        ..
    } = module;

    let address = address.and_then(|addr| {
        match addr.value {
            LeadingNameAccess_::AnonymousAddress(addr) => Some(AccountAddress::new(addr.into_bytes())),
            LeadingNameAccess_::Name(_) => adds,
        }
    }).or(adds)
        .ok_or_else(|| anyhow!("Failed to parse module definition. The module {} does not contain an address definition.", name.0.value))?;

    let funs = members
        .iter()
        .filter_map(|member| match member {
            ModuleMember::Function(func) => {
                let type_parameters = func
                    .signature
                    .type_parameters
                    .iter()
                    .map(|tp| tp.0.value.to_owned())
                    .collect();
                let parameters = func
                    .signature
                    .parameters
                    .iter()
                    .map(|(var, tp)| (var.0.value.to_owned(), extract_type_name(tp.to_owned())))
                    .collect();

                let visibility = match func.visibility {
                    AstVisibility::Public(_) => Visibility::Public,
                    AstVisibility::Script(_) => Visibility::Script,
                    AstVisibility::Friend(_) => Visibility::Friend,
                    AstVisibility::Internal => Visibility::Internal,
                };

                let span: Span = func.loc.span.into();
                Some(Spanned::new(
                    func.loc.into(),
                    FuncMeta_ {
                        name: Identifier::new(func.name.0.value.to_owned())
                            .expect("Valid identifier"),
                        visibility,
                        type_parameters,
                        parameters,
                        doc: docs.get(&span.start()).cloned(),
                    },
                ))
            }
            _ => None,
        })
        .collect();

    let structs = members
        .into_iter()
        .filter_map(|member| match member {
            ModuleMember::Struct(struc) => {
                let abilities = struc
                    .abilities
                    .iter()
                    .map(|ab| ab.value.to_string())
                    .collect();

                let fields = match struc.fields {
                    StructFields::Defined(fields) => fields
                        .iter()
                        .map(|(name, tp)| {
                            let span: Span = name.loc().span.into();
                            Spanned::new(
                                name.loc().into(),
                                StructMetaField_ {
                                    name: Identifier::new(name.to_string())
                                        .expect("Valid identifier"),
                                    type_field: extract_type_name(tp.clone()),
                                    doc: docs.get(&span.start()).cloned(),
                                },
                            )
                        })
                        .collect(),
                    _ => Vec::new(),
                };
                let type_parameters = struc
                    .type_parameters
                    .iter()
                    .map(|(name, ab)| {
                        let ab = ab.iter().map(|ab| ab.to_string()).collect();
                        (name.to_string(), ab)
                    })
                    .collect();

                let span: Span = struc.loc.span.into();
                Some(Spanned::new(
                    struc.loc.into(),
                    StructMeta_ {
                        name: Identifier::new(struc.name.0.value).expect("Valid identifier"),
                        abilities,
                        type_parameters,
                        fields,
                        doc: docs.get(&span.start()).cloned(),
                    },
                ))
            }
            _ => None,
        })
        .collect();

    let span: Span = module.loc.span.into();
    Ok(Spanned::new(
        module.loc.into(),
        ModuleMeta_ {
            address,
            name: Identifier::new(name.0.value)?,
            funs,
            structs,
            doc: docs.get(&span.start()).cloned(),
        },
    ))
}

#[cfg(test)]
mod metadata_tests {
    use crate::compiler::metadata::{
        module_meta, ModuleMeta_, FuncMeta_, Visibility, script_meta, StructMeta_,
        StructMetaField_, StructMetaField, Spanned,
    };
    use crate::compiler::dialects::DialectName;
    use tempfile::NamedTempFile;
    use std::io::Write;
    use move_core_types::language_storage::CORE_CODE_ADDRESS;
    use move_core_types::identifier::Identifier;
    use move_core_types::account_address::AccountAddress;
    use move_ir_types::location::SpanDef;
    use crate::compiler::metadata::spanned::Loc;

    fn spanned_wrap<T>(value: T) -> Spanned<T> {
        Spanned::new(Loc::new("none".to_string(), SpanDef::default()), value)
    }
    fn create_field(name: &str, tp: &str, doc: Option<&str>) -> StructMetaField {
        spanned_wrap(StructMetaField_ {
            name: Identifier::new(name).unwrap(),
            type_field: tp.to_string(),
            doc: doc.map(|d| d.to_string()),
        })
    }

    trait ToSpannded {
        fn to_spanned(self) -> Spanned<Self>
        where
            Self: Sized,
        {
            spanned_wrap(self)
        }
    }

    impl ToSpannded for FuncMeta_ {}
    impl ToSpannded for ModuleMeta_ {}
    impl ToSpannded for StructMeta_ {}

    #[test]
    fn test_module_meta() {
        let source = r"
            address 0x1 {
                module Empty {}

                /// doc for module
                module StructsModule{
                    struct Empty {}
                    /// doc for stucture
                    struct Example {
                        /// doc for field
                        field1: u8,
                        field2: u64,
                        field3: address,
                        field4: bool,
                        field5: Empty
                    }
                    struct Example2<T: copy + drop> has copy, drop {
                        field1: bool,
                        field2: T
                    }
                }
                module FuncsVisability {

                    struct MyStruct {
                        field1: bool,
                    }
                    /// doc for function
                    fun f1() {}
                    /*
                    not doc type comment
                    */
                    public fun f2() {}
                    // not doc type comment
                    public(script) fun f3() {}

                    public(friend) fun f4() {}
                    native fun f5();
                    native public fun f6();
                }
            }

            module 0x2::FuncsTp {
                public(script) fun f1<T, D>() {}
                public(script) fun f2() {}
            }

            module 0x3::FuncsArgs {
                public(script) fun f1() {}
                public(script) fun f2(_d: signer, d: u8) {}
            }";
        let mut module = NamedTempFile::new().unwrap();
        module.write_all(source.as_bytes()).unwrap();

        let dialect = DialectName::Pont.get_dialect();

        let defs = module_meta(
            module.path().to_string_lossy().as_ref(),
            dialect.as_ref(),
            "0x1",
        )
        .unwrap();

        assert_eq!(
            defs,
            vec![
                ModuleMeta_ {
                    address: CORE_CODE_ADDRESS,
                    name: Identifier::new("Empty").unwrap(),
                    funs: vec![],
                    structs: vec![],
                    doc: None
                }
                .to_spanned(),
                ModuleMeta_ {
                    address: CORE_CODE_ADDRESS,
                    name: Identifier::new("StructsModule").unwrap(),
                    funs: vec![],
                    structs: vec![
                        StructMeta_ {
                            name: Identifier::new("Empty").unwrap(),
                            abilities: vec![],
                            type_parameters: vec![],
                            fields: vec![],
                            doc: None
                        }
                        .to_spanned(),
                        StructMeta_ {
                            name: Identifier::new("Example").unwrap(),
                            abilities: vec![],
                            type_parameters: vec![],
                            fields: vec![
                                create_field("field1", "u8", Some("doc for field")),
                                create_field("field2", "u64", None),
                                create_field("field3", "address", None),
                                create_field("field4", "bool", None),
                                create_field("field5", "Empty", None)
                            ],
                            doc: Some("doc for stucture".to_string())
                        }
                        .to_spanned(),
                        StructMeta_ {
                            name: Identifier::new("Example2").unwrap(),
                            abilities: vec!["copy".to_string(), "drop".to_string()],
                            type_parameters: vec![(
                                "T".to_string(),
                                vec!["copy".to_string(), "drop".to_string()]
                            )],
                            fields: vec![
                                create_field("field1", "bool", None),
                                create_field("field2", "T", None),
                            ],
                            doc: None
                        }
                        .to_spanned()
                    ],
                    doc: Some("doc for module".to_string())
                }
                .to_spanned(),
                ModuleMeta_ {
                    address: CORE_CODE_ADDRESS,
                    name: Identifier::new("FuncsVisability").unwrap(),
                    funs: vec![
                        FuncMeta_ {
                            name: Identifier::new("f1").unwrap(),
                            visibility: Visibility::Internal,
                            type_parameters: vec![],
                            parameters: vec![],
                            doc: Some("doc for function".to_string())
                        }
                        .to_spanned(),
                        FuncMeta_ {
                            name: Identifier::new("f2").unwrap(),
                            visibility: Visibility::Public,
                            type_parameters: vec![],
                            parameters: vec![],
                            doc: None
                        }
                        .to_spanned(),
                        FuncMeta_ {
                            name: Identifier::new("f3").unwrap(),
                            visibility: Visibility::Script,
                            type_parameters: vec![],
                            parameters: vec![],
                            doc: None
                        }
                        .to_spanned(),
                        FuncMeta_ {
                            name: Identifier::new("f4").unwrap(),
                            visibility: Visibility::Friend,
                            type_parameters: vec![],
                            parameters: vec![],
                            doc: None
                        }
                        .to_spanned(),
                        FuncMeta_ {
                            name: Identifier::new("f5").unwrap(),
                            visibility: Visibility::Internal,
                            type_parameters: vec![],
                            parameters: vec![],
                            doc: None
                        }
                        .to_spanned(),
                        FuncMeta_ {
                            name: Identifier::new("f6").unwrap(),
                            visibility: Visibility::Public,
                            type_parameters: vec![],
                            parameters: vec![],
                            doc: None
                        }
                        .to_spanned(),
                    ],
                    structs: vec![StructMeta_ {
                        name: Identifier::new("MyStruct").unwrap(),
                        abilities: vec![],
                        type_parameters: vec![],
                        fields: vec![create_field("field1", "bool", None)],
                        doc: None
                    }
                    .to_spanned()],
                    doc: None
                }
                .to_spanned(),
                ModuleMeta_ {
                    address: AccountAddress::from_hex_literal("0x2").unwrap(),
                    name: Identifier::new("FuncsTp").unwrap(),
                    funs: vec![
                        FuncMeta_ {
                            name: Identifier::new("f1").unwrap(),
                            visibility: Visibility::Script,
                            type_parameters: vec!["T".to_string(), "D".to_string()],
                            parameters: vec![],
                            doc: None
                        }
                        .to_spanned(),
                        FuncMeta_ {
                            name: Identifier::new("f2").unwrap(),
                            visibility: Visibility::Script,
                            type_parameters: vec![],
                            parameters: vec![],
                            doc: None
                        }
                        .to_spanned(),
                    ],
                    structs: vec![],
                    doc: None
                }
                .to_spanned(),
                ModuleMeta_ {
                    address: AccountAddress::from_hex_literal("0x3").unwrap(),
                    name: Identifier::new("FuncsArgs").unwrap(),
                    funs: vec![
                        FuncMeta_ {
                            name: Identifier::new("f1").unwrap(),
                            visibility: Visibility::Script,
                            type_parameters: vec![],
                            parameters: vec![],
                            doc: None
                        }
                        .to_spanned(),
                        FuncMeta_ {
                            name: Identifier::new("f2").unwrap(),
                            visibility: Visibility::Script,
                            type_parameters: vec![],
                            parameters: vec![
                                ("_d".to_string(), "signer".to_string(),),
                                ("d".to_string(), "u8".to_string(),),
                            ],
                            doc: None
                        }
                        .to_spanned(),
                    ],
                    structs: vec![],
                    doc: None
                }
                .to_spanned(),
            ]
        );
    }

    #[test]
    fn test_script_meta() {
        let source = r"
            script {
                /// doc for function
                fun main() {
                }
            }
            script {
                // not doc type comment
                fun main_1(_d: signer) {
                }
            }
            script {
                /*
                not doc type comment
                */
                fun main_2<T>(_d: signer) {
                }
            }
        ";

        let mut module = NamedTempFile::new().unwrap();
        module.write_all(source.as_bytes()).unwrap();

        let dialect = DialectName::Pont.get_dialect();

        let defs = script_meta(
            module.path().to_string_lossy().as_ref(),
            dialect.as_ref(),
            "0x1",
        )
        .unwrap();
        assert_eq!(
            defs,
            vec![
                FuncMeta_ {
                    name: Identifier::new("main").unwrap(),
                    visibility: Visibility::Script,
                    type_parameters: vec![],
                    parameters: vec![],
                    doc: Some("doc for function".to_string())
                }
                .to_spanned(),
                FuncMeta_ {
                    name: Identifier::new("main_1").unwrap(),
                    visibility: Visibility::Script,
                    type_parameters: vec![],
                    parameters: vec![("_d".to_string(), "signer".to_string(),)],
                    doc: None
                }
                .to_spanned(),
                FuncMeta_ {
                    name: Identifier::new("main_2").unwrap(),
                    visibility: Visibility::Script,
                    type_parameters: vec!["T".to_string()],
                    parameters: vec![("_d".to_string(), "signer".to_string(),)],
                    doc: None
                }
                .to_spanned(),
            ]
        );
    }
}
