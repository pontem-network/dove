use std::collections::HashMap;

use anyhow::Error;
use move_core_types::account_address::AccountAddress;
use move_lang::parse_file;
use move_lang::errors::{FilesSourceText, output_errors};
use move_lang::parser::ast::{
    Definition, Script, Type, Type_, NameAccessChain_, LeadingNameAccess_, ModuleDefinition,
    ModuleMember, Visibility as AstVisibility,
};

use crate::compiler::dialects::Dialect;
use crate::compiler::preprocessor::BuilderPreprocessor;
use codespan_reporting::term::termcolor::{StandardStream, ColorChoice};
use move_core_types::identifier::Identifier;
use move_lang::callback::Interact;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct FuncMeta {
    pub name: Identifier,
    pub visibility: Visibility,
    pub type_parameters: Vec<String>,
    pub parameters: Vec<(String, String)>,
}

pub fn script_meta(
    script_path: &str,
    dialect: &dyn Dialect,
    sender: &str,
) -> Result<Vec<FuncMeta>, Error> {
    Ok(parse(script_path, dialect, sender)?
        .into_iter()
        .filter_map(|def| {
            if let Definition::Script(script) = def {
                make_script_meta(script).ok()
            } else {
                None
            }
        })
        .collect::<Vec<_>>())
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
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

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct ModuleMeta {
    pub address: AccountAddress,
    pub name: Identifier,
    pub funs: Vec<FuncMeta>,
}

pub fn module_meta(
    module_path: &str,
    dialect: &dyn Dialect,
    sender: &str,
) -> Result<Vec<ModuleMeta>, Error> {
    let mut modules = Vec::new();

    for def in parse(module_path, dialect, sender)? {
        match def {
            Definition::Module(module) => {
                modules.push(parse_module_definition(module, None)?);
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
                    modules.push(parse_module_definition(def, addr)?);
                }
            }
            Definition::Script(_) => {
                // no-op
            }
        }
    }
    Ok(modules)
}

fn parse(
    script_path: &str,
    dialect: &dyn Dialect,
    sender: &str,
) -> Result<Vec<Definition>, Error> {
    let mut preprocessor = BuilderPreprocessor::new(dialect, sender);
    let mut files: FilesSourceText = HashMap::new();
    let (defs, _, errors) = parse_file(&mut files,  preprocessor.static_str(script_path.to_string()), &mut preprocessor)?;
    if errors.is_empty() {
        Ok(defs)
    } else {
        let errors = preprocessor.transform(errors);
        let mut writer = StandardStream::stderr(ColorChoice::Auto);
        output_errors(&mut writer, files, errors);
        Err(anyhow!("Could not compile scripts '{}'.", script_path))
    }
}

fn make_script_meta(script: Script) -> Result<FuncMeta, Error> {
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
    Ok(FuncMeta {
        name: Identifier::new(func.name.0.value)?,
        visibility: Visibility::Script,
        type_parameters,
        parameters,
    })
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
        .into_iter()
        .filter_map(|member| match member {
            ModuleMember::Function(func) => {
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

                let visibility = match func.visibility {
                    AstVisibility::Public(_) => Visibility::Public,
                    AstVisibility::Script(_) => Visibility::Script,
                    AstVisibility::Friend(_) => Visibility::Friend,
                    AstVisibility::Internal => Visibility::Internal,
                };

                Some(FuncMeta {
                    name: Identifier::new(func.name.0.value).expect("Valid identifier"),
                    visibility,
                    type_parameters,
                    parameters,
                })
            }
            _ => None,
        })
        .collect();

    Ok(ModuleMeta {
        address,
        name: Identifier::new(name.0.value)?,
        funs,
    })
}

#[cfg(test)]
mod metadata_tests {
    use crate::compiler::metadata::{module_meta, ModuleMeta, FuncMeta, Visibility, script_meta};
    use crate::compiler::dialects::DialectName;
    use tempfile::NamedTempFile;
    use std::io::Write;
    use move_core_types::language_storage::CORE_CODE_ADDRESS;
    use move_core_types::identifier::Identifier;
    use move_core_types::account_address::AccountAddress;

    #[test]
    fn test_module_meta() {
        let source = r"
address 0x1 {
module Empty {}

module FuncsVisability {
    fun f1() {}

    public fun f2() {}

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
}
        ";
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
                ModuleMeta {
                    address: CORE_CODE_ADDRESS,
                    name: Identifier::new("Empty").unwrap(),
                    funs: vec![],
                },
                ModuleMeta {
                    address: CORE_CODE_ADDRESS,
                    name: Identifier::new("FuncsVisability").unwrap(),
                    funs: vec![
                        FuncMeta {
                            name: Identifier::new("f1").unwrap(),
                            visibility: Visibility::Internal,
                            type_parameters: vec![],
                            parameters: vec![],
                        },
                        FuncMeta {
                            name: Identifier::new("f2").unwrap(),
                            visibility: Visibility::Public,
                            type_parameters: vec![],
                            parameters: vec![],
                        },
                        FuncMeta {
                            name: Identifier::new("f3").unwrap(),
                            visibility: Visibility::Script,
                            type_parameters: vec![],
                            parameters: vec![],
                        },
                        FuncMeta {
                            name: Identifier::new("f4").unwrap(),
                            visibility: Visibility::Friend,
                            type_parameters: vec![],
                            parameters: vec![],
                        },
                        FuncMeta {
                            name: Identifier::new("f5").unwrap(),
                            visibility: Visibility::Internal,
                            type_parameters: vec![],
                            parameters: vec![],
                        },
                        FuncMeta {
                            name: Identifier::new("f6").unwrap(),
                            visibility: Visibility::Public,
                            type_parameters: vec![],
                            parameters: vec![],
                        },
                    ],
                },
                ModuleMeta {
                    address: AccountAddress::from_hex_literal("0x2").unwrap(),
                    name: Identifier::new("FuncsTp").unwrap(),
                    funs: vec![
                        FuncMeta {
                            name: Identifier::new("f1").unwrap(),
                            visibility: Visibility::Script,
                            type_parameters: vec!["T".to_string(), "D".to_string()],
                            parameters: vec![],
                        },
                        FuncMeta {
                            name: Identifier::new("f2").unwrap(),
                            visibility: Visibility::Script,
                            type_parameters: vec![],
                            parameters: vec![],
                        },
                    ],
                },
                ModuleMeta {
                    address: AccountAddress::from_hex_literal("0x3").unwrap(),
                    name: Identifier::new("FuncsArgs").unwrap(),
                    funs: vec![
                        FuncMeta {
                            name: Identifier::new("f1").unwrap(),
                            visibility: Visibility::Script,
                            type_parameters: vec![],
                            parameters: vec![],
                        },
                        FuncMeta {
                            name: Identifier::new("f2").unwrap(),
                            visibility: Visibility::Script,
                            type_parameters: vec![],
                            parameters: vec![
                                ("_d".to_string(), "signer".to_string(),),
                                ("d".to_string(), "u8".to_string(),),
                            ],
                        },
                    ],
                },
            ]
        );
    }

    #[test]
    fn test_script_meta() {
        let source = r"
            script {
                fun main() {
                }
            }
            script {
                fun main_1(_d: signer) {
                }
            }
            script {
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
                FuncMeta {
                    name: Identifier::new("main").unwrap(),
                    visibility: Visibility::Script,
                    type_parameters: vec![],
                    parameters: vec![],
                },
                FuncMeta {
                    name: Identifier::new("main_1").unwrap(),
                    visibility: Visibility::Script,
                    type_parameters: vec![],
                    parameters: vec![("_d".to_string(), "signer".to_string(),)],
                },
                FuncMeta {
                    name: Identifier::new("main_2").unwrap(),
                    visibility: Visibility::Script,
                    type_parameters: vec!["T".to_string()],
                    parameters: vec![("_d".to_string(), "signer".to_string(),)],
                },
            ]
        );
    }
}
