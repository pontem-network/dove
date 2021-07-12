use std::collections::HashMap;

use anyhow::Error;
use move_core_types::account_address::AccountAddress;
use move_lang::{find_move_filenames, leak_str, parse_file};
use move_lang::errors::{Errors, FilesSourceText, output_errors};
use move_lang::parser::ast::{Definition, ModuleAccess_, Script, Type, Type_};

use crate::compiler::dialects::Dialect;
use crate::compiler::preprocessor::BuilderPreprocessor;
use codespan_reporting::term::termcolor::{StandardStream, ColorChoice};

#[derive(Debug)]
pub struct Meta {
    pub name: String,
    pub type_parameters: Vec<String>,
    pub parameters: Vec<(String, String)>,
}

pub fn script_metadata(
    targets: &[String],
    dialect: &dyn Dialect,
    sender: Option<AccountAddress>,
) -> Result<Vec<Meta>, Error> {
    let targets = find_move_filenames(targets, true)?
        .iter()
        .map(|s| leak_str(s))
        .collect::<Vec<&'static str>>();

    let mut preprocessor = BuilderPreprocessor::new(dialect, sender);

    let mut files: FilesSourceText = HashMap::new();
    let mut errors: Errors = Vec::new();
    let mut source_definitions = Vec::new();

    for fname in targets {
        let (defs, _, mut es) = parse_file(&mut files, fname, &mut preprocessor)?;
        source_definitions.extend(defs);
        errors.append(&mut es);
    }
    let errors = preprocessor.into_offset_map().transform(errors);

    if errors.is_empty() {
        Ok(source_definitions
            .into_iter()
            .filter_map(|def| {
                if let Definition::Script(script) = def {
                    Some(make_script_meta(script))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>())
    } else {
        let mut writer = StandardStream::stderr(ColorChoice::Auto);
        output_errors(&mut writer, files, errors);
        Err(anyhow!("could not compile scripts."))
    }
}

fn make_script_meta(script: Script) -> Meta {
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
    Meta {
        name: func.name.0.value,
        type_parameters,
        parameters,
    }
}

fn extract_type_name(tp: Type) -> String {
    match tp.value {
        Type_::Apply(name, types) => {
            let mut tp = match name.value {
                ModuleAccess_::Name(name) => name.value,
                ModuleAccess_::ModuleAccess(module, name) => {
                    format!("{}::{}", module.0.value, name.value)
                }
                ModuleAccess_::QualifiedModuleAccess(module, name) => {
                    let (address, m_name) = module.value;
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
