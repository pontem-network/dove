use crate::compiler::dialects::Dialect;
use crate::compiler::address::ProvidedAccountAddress;
use crate::compiler::file::MoveFile;
use crate::compiler::{CompileFlow, Step, compile, CheckerResult};
use crate::compiler::parser::{ParsingMeta, ParserArtifact};
use diem::move_lang::compiled_unit::CompiledUnit;
use diem::move_lang::errors::Errors;

pub struct MoveChecker<'a> {
    dialect: &'a dyn Dialect,
    sender: Option<&'a ProvidedAccountAddress>,
}

impl<'a> MoveChecker<'a> {
    pub fn new(
        dialect: &'a dyn Dialect,
        sender: Option<&'a ProvidedAccountAddress>,
    ) -> MoveChecker<'a> {
        MoveChecker { dialect, sender }
    }

    pub fn check(self, targets: &[MoveFile], deps: &[MoveFile]) -> Result<(), Errors> {
        compile(self.dialect, targets, deps, self.sender, self)
    }
}

impl<'a> CompileFlow<Result<(), Errors>> for MoveChecker<'a> {
    fn after_parsing(
        &mut self,
        parser_artifact: ParserArtifact,
    ) -> Step<Result<(), Errors>, ParserArtifact> {
        if parser_artifact.result.is_err() {
            let ParserArtifact { meta, result } = parser_artifact;
            Step::Stop(
                result
                    .map(|_| ())
                    .map_err(|errors| meta.offsets_map.transform(errors)),
            )
        } else {
            Step::Next(parser_artifact)
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
