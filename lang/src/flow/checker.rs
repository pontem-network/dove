use move_core_types::account_address::AccountAddress;
use move_lang::compiled_unit::CompiledUnit;
use move_lang::errors::Errors;
use move_lang::FullyCompiledProgram;
use crate::compiler::{CheckerResult, compile, CompileFlow, Step};
use crate::compiler::dialects::Dialect;
use crate::compiler::file::MoveFile;
use crate::compiler::parser::{ParserArtifact, ParsingMeta};
use crate::flow::DependencyResolver;

pub struct MoveChecker<'a, R: DependencyResolver> {
    dialect: &'a dyn Dialect,
    sender: Option<AccountAddress>,
    resolver: R,
}

impl<'a, R: DependencyResolver> MoveChecker<'a, R> {
    pub fn new(
        dialect: &'a dyn Dialect,
        sender: Option<AccountAddress>,
        resolver: R,
    ) -> MoveChecker<'a, R> {
        MoveChecker {
            dialect,
            sender,
            resolver,
        }
    }

    pub fn check(self, targets: &[MoveFile]) -> Result<(), Errors> {
        compile(self.dialect, targets, self.sender, self)
    }
}

impl<'a, R: DependencyResolver> CompileFlow<Result<(), Errors>> for MoveChecker<'a, R> {
    fn after_parse_target(
        &mut self,
        parser_artifact: ParserArtifact,
    ) -> Step<Result<(), Errors>, (ParserArtifact, Option<Vec<MoveFile<'static, 'static>>>)> {
        let ParserArtifact { meta, result } = parser_artifact;
        match result {
            Ok(ast) => match self.resolver.resolve_source_deps(&ast.source_definitions) {
                Ok(deps) => Step::Next((
                    ParserArtifact {
                        meta,
                        result: Ok(ast),
                    },
                    deps,
                )),
                Err(error) => Step::Stop(Err(vec![error])),
            },
            Err(errors) => Step::Stop(Err(meta.offsets_map.transform(errors))),
        }
    }

    fn after_parse_program(
        &mut self,
        parser_artifact: ParserArtifact,
    ) -> Step<Result<(), Errors>, (ParserArtifact, Option<FullyCompiledProgram>)> {
        let ParserArtifact { meta, result } = parser_artifact;
        match result {
            Ok(ast) => match self.resolver.resolve_precompiled(&ast) {
                Ok(deps) => Step::Next((
                    ParserArtifact {
                        meta,
                        result: Ok(ast),
                    },
                    deps,
                )),
                Err(error) => Step::Stop(Err(vec![error])),
            },
            Err(errors) => Step::Stop(Err(meta.offsets_map.transform(errors))),
        }
    }

    fn after_check(
        &mut self,
        meta: ParsingMeta,
        check_result: CheckerResult,
    ) -> Step<Result<(), Errors>, (ParsingMeta, CheckerResult)> {
        Step::Stop(
            check_result
                .map(|_| ())
                .map_err(|errors| meta.offsets_map.transform(errors)),
        )
    }

    fn after_translate(
        &mut self,
        _: ParsingMeta,
        _: Result<Vec<CompiledUnit>, Errors>,
    ) -> Result<(), Errors> {
        Ok(())
    }
}
