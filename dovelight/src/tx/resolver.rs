use anyhow::Error;
use regex::Regex;
use lang::compiler::metadata::FuncMeta;
use move_core_types::identifier::Identifier;
use move_core_types::account_address::AccountAddress;
use crate::tx::ProjectData;
use crate::langwasm::metadata::{script_meta_source, module_meta_source};

pub fn find_script(
    project_data: &ProjectData,
    script_name: &Identifier,
    source_index: Option<String>,
) -> Result<Vec<(String, FuncMeta)>, Error> {
    let move_indexes = if let Some(index) = source_index {
        vec![index]
    } else {
        project_data.source_map.keys()
    };
    let move_indexes = find_by_regexp(
        &project_data,
        move_indexes,
        &format!(r#"fun[\s]+{}"#, script_name.as_str()),
    )?;
    let sender = &project_data.account_address;

    Ok(move_indexes
        .iter()
        .filter_map(|index| {
            script_meta_source(
                index,
                project_data.source_map.get(index).unwrap_or_default(),
                project_data.dialect.as_ref(),
                &sender.to_string(),
            )
            .ok()
            .map(|function_metadata| (index, function_metadata))
        })
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
    project_data: &ProjectData,
    module_address: &AccountAddress,
    module_name: &Identifier,
    function_name: &Identifier,
    source_index: Option<&String>,
) -> Result<Vec<(String, FuncMeta)>, Error> {
    let script_only = project_data.cfg.script_func_only;
    let move_indexes = if let Some(index) = source_index {
        vec![index.to_string()]
    } else {
        project_data.source_map.keys()
    };
    let move_indexes = find_by_regexp(
        &project_data,
        move_indexes,
        &format!(
            r#"module([\s]+|[\s]+[\dA-Za-z{{}}]+::){}[\s]+\{{"#,
            module_name
        ),
    )?;

    let sender = &project_data.account_address;

    Ok(move_indexes
        .iter()
        .filter_map(|index| {
            module_meta_source(
                index,
                project_data.source_map.get(index).unwrap_or_default(),
                project_data.dialect.as_ref(),
                &sender.to_string(),
            )
            .ok()
            .map(|m| (index, m))
        })
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
    project_data: &ProjectData,
    move_files: Vec<String>,
    regex: &str,
) -> Result<Vec<String>, Error> {
    let regexp = Regex::new(regex)?;
    Ok(move_files
        .iter()
        .filter(|index| {
            if let Some(source) = project_data.source_map.get(&index) {
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
    use crate::tx::ProjectData;
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
        let project_data = ProjectData {
            chain_api: "localhost".to_string(),
            source_map,
            dialect: DialectName::from_str("diem").unwrap().get_dialect(),
            account_address: AccountAddress::from_hex_literal("0x1").unwrap(),
            cfg: Config::for_tx(),
        };

        let name = Identifier::new("run_tmp_0").unwrap();
        let mut result = find_script(&project_data, &name, None).unwrap();
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
                    "module.tmp2.move": "module 0x1::Tmp2 { public(script) fun tmp_run() { let _v = Tmp2::get();  }; public fun get():u8 { 2 } }",
                    "script.test12.move": "script { use 0x1::Tmp0; fun run_tmp_0() { let _result = Tmp0::get(); } }script { use 0x1::Tmp1; fun run_tmp_1() { let _result = Tmp1::get(); } }"
                }
            }"#;
        let source_map: SourceMap = serde_json::from_str(source_text).unwrap();
        let project_data = ProjectData {
            chain_api: "localhost".to_string(),
            source_map,
            dialect: DialectName::from_str("diem").unwrap().get_dialect(),
            account_address: AccountAddress::from_hex_literal("0x1").unwrap(),
            cfg: Config::for_tx(),
        };

        let module_address = AccountAddress::from_hex_literal("0x1").unwrap();
        let module_name = Identifier::new("Tmp2").unwrap();
        let function_name = Identifier::new("tmp_run").unwrap();
        let mut result = find_module_function(
            &project_data,
            &module_address,
            &module_name,
            &function_name,
            None,
        )
        .unwrap();
        println!("{:?}", result);
        result.sort_by(|a, b| a.0.cmp(&b.0));

        assert_eq!(
            result,
            vec![
                (
                    "script.test0.move".to_string(),
                    FuncMeta {
                        name: function_name.clone(),
                        visibility: Visibility::Script,
                        type_parameters: vec![],
                        parameters: vec![],
                    },
                ),
                (
                    "script.test0_1.move".to_string(),
                    FuncMeta {
                        name: function_name.clone(),
                        visibility: Visibility::Script,
                        type_parameters: vec![],
                        parameters: vec![],
                    },
                ),
                (
                    "script.test12.move".to_string(),
                    FuncMeta {
                        name: function_name.clone(),
                        visibility: Visibility::Script,
                        type_parameters: vec![],
                        parameters: vec![],
                    },
                ),
            ]
        );
    }
}
