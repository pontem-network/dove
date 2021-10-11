use anyhow::Error;
use regex::Regex;
use lang::compiler::metadata::FuncMeta;
use move_core_types::identifier::Identifier;
use crate::tx::ProjectData;
use crate::langwasm::metadata::script_meta_source;

pub fn find_script(
    project_data: &ProjectData,
    name: &Identifier,
    file: Option<String>,
) -> Result<Vec<(String, FuncMeta)>, Error> {
    let move_files = if let Some(file) = file {
        vec![file]
    } else {
        project_data.source_map.keys()
    };
    let move_files = find_by_regexp(
        &project_data,
        move_files,
        &format!(r#"fun[\s]+{}"#, name.as_str()),
    )?;
    let sender = &project_data.address;

    Ok(move_files
        .iter()
        .filter_map(|p| {
            script_meta_source(
                p,
                project_data.source_map.get(p).unwrap_or_default(),
                project_data.dialect.as_ref(),
                &sender.to_string(),
            )
            .ok()
            .map(|f| (p, f))
        })
        .flat_map(|(p, m)| {
            m.into_iter()
                .filter(|m| &m.name == name)
                .map(|m| (p.to_owned(), m))
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
    use crate::tx::resolver::find_script;

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
            source_map,
            dialect: DialectName::from_str("diem").unwrap().get_dialect(),
            address: AccountAddress::from_hex_literal("0x1").unwrap(),
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
}
