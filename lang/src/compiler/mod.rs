pub use anyhow::Result;
use move_lang::{cfgir, move_continue_up_to, Pass, PassResult, FullyCompiledProgram};
use move_lang::compiled_unit::CompiledUnit;
use move_lang::errors::Errors;
pub use move_lang::name_pool::ConstPool;

use parser::parse_program;
use crate::compiler::dialects::Dialect;
use crate::compiler::file::MoveFile;
use crate::compiler::parser::{ParsingMeta, ParserArtifact};
use move_core_types::account_address::AccountAddress;
use move_lang::shared::Address;

pub mod address;
pub mod dialects;
pub mod error;
pub mod file;
pub mod location;
pub mod mut_string;
pub mod parser;
pub mod source_map;

pub type CheckerResult = Result<cfgir::ast::Program, Errors>;
pub type SourceWithDeps = (
    ParserArtifact,
    Option<Vec<MoveFile<'static, 'static>>>,
    Option<FullyCompiledProgram>,
);

pub trait CompileFlow<A> {
    fn init(&mut self, _dialect: &dyn Dialect, _sender: Option<AccountAddress>) {}
    fn after_parse_target(&mut self, parser_artifact: ParserArtifact) -> Step<A, SourceWithDeps> {
        Step::Next((parser_artifact, None, None))
    }
    fn after_parse_program(
        &mut self,
        parser_artifact: ParserArtifact,
    ) -> Step<A, ParserArtifact> {
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
    sender: Option<AccountAddress>,
    mut flow: impl CompileFlow<A>,
) -> A {
    // Init compiler flow.
    flow.init(dialect, sender);
    // Parse target.
    todo!()
    //
    // let parser_result = match flow.after_parsing(parse_program(dialect, targets, deps, sender)) {
    //     Step::Stop(artifact) => return artifact,
    //     Step::Next(res) => res,
    // };
    // let ParserArtifact {
    //     meta,
    //     result: pprog_res,
    // } = parser_result;
    //
    // let sender = sender.map(|addr| Address::new(addr.to_u8()));
    //
    // let check_result = pprog_res
    //     .and_then(|pprog| {
    //         move_continue_up_to(None, PassResult::Parser(sender, pprog), Pass::CFGIR)
    //     })
    //     .map(|res| match res {
    //         PassResult::CFGIR(cfgir) => cfgir,
    //         _ => unreachable!(),
    //     });
    //
    // let (meta, check_result) = match flow.after_check(meta, check_result) {
    //     Step::Stop(artifact) => return artifact,
    //     Step::Next(res) => res,
    // };
    //
    // let units = check_result
    //     .and_then(|check_result| {
    //         move_continue_up_to(None, PassResult::CFGIR(check_result), Pass::Compilation)
    //     })
    //     .map(|res| match res {
    //         PassResult::Compilation(units) => units,
    //         _ => unreachable!(),
    //     });
    //
    // flow.after_translate(meta, units)
}
