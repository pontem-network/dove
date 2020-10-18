pub mod address;
pub mod bech32;
pub mod dialects;
pub mod file;
pub mod parser;
pub mod source_map;
pub mod error;
pub mod location;

pub use anyhow::Result;
pub use move_lang::name_pool::ConstPool;
use move_lang::compiled_unit::CompiledUnit;
use move_lang::errors::Errors;
use parser::parse_program;
use crate::compiler::dialects::Dialect;
use crate::compiler::address::ProvidedAccountAddress;
use crate::compiler::file::MoveFile;
use move_lang::{check_program, cfgir, to_bytecode};
use crate::compiler::parser::{ParserArtifact, ParsingMeta};

pub type CheckerResult = Result<cfgir::ast::Program, Errors>;

pub trait CompileFlow<A> {
    fn init(&mut self, _dialect: &dyn Dialect, _sender: &Option<&ProvidedAccountAddress>) {}
    fn after_parsing(&mut self, parser_artifact: ParserArtifact) -> Step<A, ParserArtifact> {
        Step::Next(parser_artifact)
    }
    fn after_check(
        &mut self,
        meta: ParsingMeta,
        check_result: CheckerResult,
    ) -> Step<A, (ParsingMeta, CheckerResult)> {
        Step::Next((meta, check_result))
    }
    fn after_translate(
        &mut self,
        meta: ParsingMeta,
        translation_result: Result<Vec<CompiledUnit>, Errors>,
    ) -> A;
}

pub enum Step<A, N> {
    Stop(A),
    Next(N),
}

pub fn compile<A>(
    dialect: &dyn Dialect,
    targets: &[MoveFile],
    deps: &[MoveFile],
    sender: Option<&ProvidedAccountAddress>,
    mut flow: impl CompileFlow<A>,
) -> A {
    flow.init(dialect, &sender);
    let parser_result = match flow.after_parsing(parse_program(dialect, targets, deps, sender)) {
        Step::Stop(artifact) => return artifact,
        Step::Next(res) => res,
    };
    let ParserArtifact {
        meta,
        result: pprog_res,
    } = parser_result;

    let sender = sender.map(|addr| addr.as_address());
    let (meta, check_result) = match flow.after_check(meta, check_program(pprog_res, sender)) {
        Step::Stop(artifact) => return artifact,
        Step::Next(res) => res,
    };

    flow.after_translate(meta, check_result.and_then(to_bytecode::translate::program))
}
