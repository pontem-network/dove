use anyhow::Error;
use regex::Regex;
use lang::compiler::metadata::FuncMeta;
use move_core_types::identifier::Identifier;
use move_core_types::account_address::AccountAddress;
use crate::tx::Context;
use crate::langwasm::metadata::{script_meta_source, module_meta_source};
use crate::compiler::source_map::SourceMap;

pub fn find_script(
    // Project Code
    source_map: &SourceMap,
    // Launch data
    conext: &Context,
    script_name: &Identifier,
    source_index: Option<String>,
) -> Result<Vec<(String, FuncMeta)>, Error> {
    let move_indexes = if let Some(index) = source_index {
        vec![index]
    } else {
        source_map.keys()
    };
    let move_indexes = find_by_regexp(
        &source_map,
        move_indexes,
        &format!(r#"fun[\s]+{}"#, script_name.as_str()),
    )?;
    let sender = conext.account_address.to_string();

    Ok(move_indexes
        .iter()
        .map(|index| {
            script_meta_source(
                index,
                source_map.get(index).unwrap_or_default(),
                conext.dialect.as_ref(),
                &sender,
            )
            .map(|function_metadata| (index, function_metadata))
        })
        .collect::<Result<Vec<(_, _)>, Error>>()?
        .into_iter()
        .flat_map(|(index, functions_meta)| {
            functions_meta
                .into_iter()
                .filter(|meta| &meta.name == script_name)
                .map(|meta| (index.to_owned(), meta))
                .collect::<Vec<_>>()
        })
        .collect())
}

pub fn find_module_function(
    // Project Code
    source_map: &SourceMap,
    // Launch data
    context: &Context,
    module_address: &AccountAddress,
    module_name: &Identifier,
    function_name: &Identifier,
    source_index: Option<&String>,
) -> Result<Vec<(String, FuncMeta)>, Error> {
    let script_only = context.cfg.script_func_only;
    let source_indexes = if let Some(index) = source_index {
        vec![index.to_string()]
    } else {
        source_map.keys()
    };
    let source_indexes = find_by_regexp(
        source_map,
        source_indexes,
        &format!(
            r#"module([\s]+|[\s]+[\dA-Za-z{{}}]+::){}[\s]+\{{"#,
            module_name
        ),
    )?;

    let sender = context.account_address.to_string();
    Ok(source_indexes
        .iter()
        .map(|index| {
            module_meta_source(
                index,
                source_map.get(index).unwrap_or_default(),
                context.dialect.as_ref(),
                &sender,
            )
            .map(|m| (index, m))
        })
        .collect::<Result<Vec<(_, _)>, Error>>()?
        .into_iter()
        .flat_map(|(index, modules_meta)| {
            modules_meta
                .into_iter()
                .filter(|module_meta| {
                    module_meta.address == *module_address && &module_meta.name == module_name
                })
                .flat_map(|module_meta| module_meta.funs)
                .filter(|function_meta| &function_meta.name == function_name)
                .filter(|function_meta| {
                    if script_only {
                        function_meta.visibility.is_script()
                    } else {
                        false
                    }
                })
                .map(|f| (index.to_owned(), f))
                .collect::<Vec<_>>()
        })
        .collect())
}

fn find_by_regexp(
    // Project Code
    source_map: &SourceMap,
    // In which indexes to search
    sources_indexes: Vec<String>,
    // Regex search condition
    regex: &str,
) -> Result<Vec<String>, Error> {
    let regexp = Regex::new(regex)?;
    Ok(sources_indexes
        .iter()
        .filter(|index| {
            if let Some(source) = source_map.get(&index) {
                return regexp.is_match(source);
            }
            false
        })
        .cloned()
        .collect())
}

#[cfg(test)]
mod test {
    use std::str::FromStr;
    use lang::compiler::dialects::DialectName;
    use move_core_types::account_address::AccountAddress;
    use move_core_types::identifier::Identifier;
    use lang::compiler::metadata::{FuncMeta, Visibility};
    use crate::compiler::source_map::SourceMap;
    use crate::tx::Context;
    use crate::tx::resolver::{find_script, find_module_function};
    use lang::tx::fn_call::Config;

    #[test]
    fn test_find_script() {
        let source_text = r#"{
                "source_map": {
                    "module.tmp0.move": "module 0x1::Tmp0 { public fun get():u8 { 0 } }",
                    "module.tmp1.move": "module 0x1::Tmp1 { public fun get():u8 { 1 } }",
                    "module.tmp2.move": "module 0x1::Tmp2 { public fun get():u8 { 2 } }",
                    "script.test0.move": "script { use 0x1::Tmp0; fun run_tmp_0() { let _result = Tmp0::get(); } }",
                    "script.test1.move": "script { use 0x1::Tmp1; fun run_tmp_1() { let _result = Tmp1::get(); } }",
                    "script.test2.move": "script { use 0x1::Tmp2; fun run_tmp_2() { let _result = Tmp2::get(); } }",
                    "script.test0_1.move": "script { use 0x1::Tmp0; fun run_tmp_0() { let _result = Tmp0::get(); } }",
                    "script.test12.move": "script { use 0x1::Tmp0; fun run_tmp_0() { let _result = Tmp0::get(); } }script { use 0x1::Tmp1; fun run_tmp_1() { let _result = Tmp1::get(); } }"
                }
            }"#;
        let source_map: SourceMap = serde_json::from_str(source_text).unwrap();
        let context = Context {
            chain_api: "localhost".to_string(),
            dialect: DialectName::from_str("diem").unwrap().get_dialect(),
            account_address: AccountAddress::from_hex_literal("0x1").unwrap(),
            cfg: Config::for_tx(),
        };

        let name = Identifier::new("run_tmp_0").unwrap();
        let mut result = find_script(&source_map, &context, &name, None).unwrap();
        result.sort_by(|a, b| a.0.cmp(&b.0));

        assert_eq!(
            result,
            vec![
                (
                    "script.test0.move".to_string(),
                    FuncMeta {
                        name: name.clone(),
                        visibility: Visibility::Script,
                        type_parameters: vec![],
                        parameters: vec![],
                    },
                ),
                (
                    "script.test0_1.move".to_string(),
                    FuncMeta {
                        name: name.clone(),
                        visibility: Visibility::Script,
                        type_parameters: vec![],
                        parameters: vec![],
                    },
                ),
                (
                    "script.test12.move".to_string(),
                    FuncMeta {
                        name: name.clone(),
                        visibility: Visibility::Script,
                        type_parameters: vec![],
                        parameters: vec![],
                    },
                ),
            ]
        );
    }

    #[test]
    fn test_find_module_function() {
        let source_text = r#"{
                "source_map": {
                    "module.tmp0.move": "module 0x1::Tmp0 { public fun get():u8 { 0 } }",
                    "module.tmp1.move": "module 0x1::Tmp1 { public fun get():u8 { 1 } }",
                    "module.tmp2.move": "module 0x1::Tmp2 { public(script) fun tmp_run() { let _v = Tmp2::get();  } public fun get():u8 { 2 } }",
                    "script.test12.move": "script { use 0x1::Tmp0; fun run_tmp_0() { let _result = Tmp0::get(); } } script { use 0x1::Tmp1; fun run_tmp_1() { let _result = Tmp1::get(); } }"
                }
            }"#;
        let source_map: SourceMap = serde_json::from_str(source_text).unwrap();
        let context = Context {
            chain_api: "localhost".to_string(),
            dialect: DialectName::from_str("diem").unwrap().get_dialect(),
            account_address: AccountAddress::from_hex_literal("0x1").unwrap(),
            cfg: Config::for_tx(),
        };

        let module_address = AccountAddress::from_hex_literal("0x1").unwrap();
        let module_name = Identifier::new("Tmp2").unwrap();
        let function_name = Identifier::new("tmp_run").unwrap();
        let mut result = find_module_function(
            &source_map,
            &context,
            &module_address,
            &module_name,
            &function_name,
            None,
        )
        .unwrap();
        result.sort_by(|a, b| a.0.cmp(&b.0));

        assert_eq!(
            result,
            vec![(
                "module.tmp2.move".to_string(),
                FuncMeta {
                    name: function_name.clone(),
                    visibility: Visibility::Script,
                    type_parameters: vec![],
                    parameters: vec![],
                },
            )]
        );
    }
}
