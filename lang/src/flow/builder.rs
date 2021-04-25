use move_core_types::account_address::AccountAddress;
use move_lang::FullyCompiledProgram;
use move_lang::compiled_unit::CompiledUnit;
use move_lang::errors::{Errors, FilesSourceText, Error};

use crate::compiler::{compile, CompileFlow, Step, SourceDeps};
use crate::compiler::dialects::Dialect;
use crate::compiler::file::MoveFile;
use crate::compiler::parser::{ParserArtifact, ParsingMeta};
use crate::flow::DependencyResolver;
use move_lang::parser::ast::{Program, Definition};

pub struct Artifacts {
    pub files: FilesSourceText,
    pub prog: Result<Vec<CompiledUnit>, Errors>,
}

pub struct MoveBuilder<'a, R: DependencyResolver> {
    dialect: &'a dyn Dialect,
    sender: Option<AccountAddress>,
    resolver: R,
}

impl<'a, R: DependencyResolver> MoveBuilder<'a, R> {
    pub fn new(
        dialect: &'a dyn Dialect,
        sender: Option<AccountAddress>,
        resolver: R,
    ) -> MoveBuilder<'a, R> {
        MoveBuilder {
            dialect,
            sender,
            resolver,
        }
    }

    pub fn build(self, targets: &[MoveFile]) -> Artifacts {
        compile(self.dialect, targets, self.sender, self)
    }
}

impl<'a, R: DependencyResolver> CompileFlow<Artifacts> for MoveBuilder<'a, R> {
    fn after_parse_target(
        &mut self,
        parser_artifact: ParserArtifact,
    ) -> Step<Artifacts, (ParserArtifact, Option<SourceDeps>)> {
        if let Ok(ast) = parser_artifact.result.as_ref() {
            match self.resolver.resolve_source_deps(&ast.source_definitions) {
                Ok(deps) => Step::Next((parser_artifact, deps)),
                Err(error) => Step::Stop(Artifacts {
                    files: parser_artifact.meta.source_map,
                    prog: Err(vec![error]),
                }),
            }
        } else {
            Step::Next((parser_artifact, None))
        }
    }

    fn after_parse_program(
        &mut self,
        parser_artifact: ParserArtifact,
    ) -> Step<Artifacts, (ParserArtifact, Option<FullyCompiledProgram>)> {
        if let Ok(ast) = parser_artifact.result.as_ref() {
            match self.resolver.resolve_precompiled(ast) {
                Ok(precompiled) => Step::Next((parser_artifact, precompiled)),
                Err(error) => Step::Stop(Artifacts {
                    files: parser_artifact.meta.source_map,
                    prog: Err(vec![error]),
                }),
            }
        } else {
            Step::Next((parser_artifact, None))
        }
    }

    fn after_translate(
        &mut self,
        meta: ParsingMeta,
        result: Result<Vec<CompiledUnit>, Errors>,
    ) -> Artifacts {
        let prog = result.map_err(|errors| meta.offsets_map.transform(errors));
        Artifacts {
            files: meta.source_map,
            prog,
        }
    }
}

pub struct StaticResolver {
    pub deps: Vec<MoveFile<'static, 'static>>,
}

impl DependencyResolver for StaticResolver {
    fn resolve_source_deps(
        &mut self,
        _: &[Definition],
    ) -> Result<Option<Vec<MoveFile<'static, 'static>>>, Error> {
        Ok(Some(self.deps.clone()))
    }

    fn resolve_precompiled(
        &mut self,
        _: &Program,
    ) -> Result<Option<FullyCompiledProgram>, Error> {
        Ok(None)
    }
}
