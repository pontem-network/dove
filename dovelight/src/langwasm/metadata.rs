use std::collections::HashMap;
use anyhow::{Error, anyhow};
use lang::compiler::dialects::Dialect;
use lang::compiler::metadata::{FuncMeta, ModuleMeta, parse_module_definition, make_script_meta};
use lang::compiler::preprocessor::BuilderPreprocessor;
use move_lang::parser::ast::{Definition, LeadingNameAccess_};
use move_lang::errors::FilesSourceText;
use crate::move_langwasm::parse_file;
use move_lang::callback::Interact;
use move_core_types::account_address::AccountAddress;

pub fn script_meta_source(
    name: &str,
    source: &str,
    dialect: &dyn Dialect,
    sender: &str,
) -> Result<Vec<FuncMeta>, Error> {
    Ok(parse(name, source, dialect, sender)?
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

pub fn module_meta_source(
    name: &str,
    source: &str,
    dialect: &dyn Dialect,
    sender: &str,
) -> Result<Vec<ModuleMeta>, Error> {
    let mut modules = Vec::new();

    for def in parse(name, source, dialect, sender)? {
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
    name: &str,
    source: &str,
    dialect: &dyn Dialect,
    sender: &str,
) -> Result<Vec<Definition>, Error> {
    let mut preprocessor = BuilderPreprocessor::new(dialect, sender);
    let mut files: FilesSourceText = HashMap::new();
    let (defs, _, errors) = parse_file(
        &mut files,
        preprocessor.static_str(name.to_string()),
        source.to_string(),
        &mut preprocessor,
    )?;
    if errors.is_empty() {
        Ok(defs)
    } else {
        Err(anyhow!("Could not compile scripts '{}'.", name))
    }
}

#[cfg(test)]
mod metadata_tests {
    use lang::compiler::dialects::DialectName;
    use lang::compiler::metadata::{ModuleMeta, FuncMeta, Visibility};
    use crate::langwasm::metadata::{module_meta_source, script_meta_source};
    use move_core_types::identifier::Identifier;
    use move_core_types::language_storage::CORE_CODE_ADDRESS;
    use move_core_types::account_address::AccountAddress;

    #[test]
    fn test_module_meta_source() {
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
        let dialect = DialectName::Pont.get_dialect();

        let defs = module_meta_source("demo", source, dialect.as_ref(), "0x1").unwrap();

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
    fn test_script_meta_source() {
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

        let dialect = DialectName::Pont.get_dialect();

        let defs = script_meta_source("demo", source, dialect.as_ref(), "0x1").unwrap();
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
