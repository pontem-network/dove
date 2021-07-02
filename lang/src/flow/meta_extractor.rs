use crate::compiler::{CompileFlow, Step, compile, SourceDeps};
use anyhow::Error;
use crate::compiler::parser::{ParsingMeta, ParserArtifact};
use move_lang::errors::Errors;
use crate::compiler::error::CompilerError;
use crate::compiler::dialects::Dialect;
use crate::compiler::file::MoveFile;
use move_lang::parser::ast::{Script, Type, Type_, ModuleAccess_, Definition};
use move_lang::compiled_unit::CompiledUnit;
use move_core_types::account_address::AccountAddress;
use move_model::model::GlobalEnv;

pub struct ScriptMetadata;

impl ScriptMetadata {
    pub fn extract(
        dialect: &dyn Dialect,
        sender: Option<AccountAddress>,
        scripts: &[&MoveFile],
    ) -> Result<Vec<Meta>, Error> {
        compile(dialect, scripts, sender, ScriptMetadata, false)
    }
}

impl CompileFlow<Result<Vec<Meta>, Error>> for ScriptMetadata {
    fn after_parse_target(
        &mut self,
        parser_artifact: ParserArtifact,
    ) -> Step<Result<Vec<Meta>, Error>, (ParserArtifact, Option<SourceDeps>)> {
        let result = parser_artifact.result;
        let source_map = parser_artifact.meta.source_map;
        let offsets_map = parser_artifact.meta.offsets_map;
        Step::Stop(
            result
                .map_err(|err| {
                    CompilerError {
                        source_map,
                        errors: offsets_map.transform(err),
                    }
                    .into()
                })
                .map(|prog| {
                    prog.source_definitions
                        .into_iter()
                        .filter_map(|def| {
                            if let Definition::Script(script) = def {
                                Some(make_script_meta(script))
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>()
                }),
        )
    }

    fn after_translate(
        &mut self,
        _: ParsingMeta,
        _: Option<GlobalEnv>,
        _: Result<Vec<CompiledUnit>, Errors>,
    ) -> Result<Vec<Meta>, Error> {
        Ok(vec![])
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

#[derive(Debug)]
pub struct Meta {
    pub name: String,
    pub type_parameters: Vec<String>,
    pub parameters: Vec<(String, String)>,
}
