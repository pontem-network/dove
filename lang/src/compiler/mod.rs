pub use anyhow::Result;
use diem::move_lang::{cfgir, move_continue_up_to, Pass, PassResult};
use diem::move_lang::compiled_unit::CompiledUnit;
use diem::move_lang::errors::Errors;
pub use diem::move_lang::name_pool::ConstPool;

use parser::parse_program;

use crate::compiler::address::ProvidedAccountAddress;
use crate::compiler::dialects::Dialect;
use crate::compiler::file::MoveFile;
use crate::compiler::parser::{ParserArtifact, ParsingMeta};

pub mod address;
pub mod bech32;
pub mod dialects;
pub mod error;
pub mod file;
pub mod location;
pub mod parser;
pub mod source_map;
pub mod ss58;

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

    let check_result = pprog_res
        .and_then(|pprog| move_continue_up_to(PassResult::Parser(sender, pprog), Pass::CFGIR))
        .map(|res| match res {
            PassResult::CFGIR(cfgir) => cfgir,
            _ => unreachable!(),
        });

    let (meta, check_result) = match flow.after_check(meta, check_result) {
        Step::Stop(artifact) => return artifact,
        Step::Next(res) => res,
    };

    let units = check_result
        .and_then(|check_result| {
            move_continue_up_to(PassResult::CFGIR(check_result), Pass::Compilation)
        })
        .map(|res| match res {
            PassResult::Compilation(units) => units,
            _ => unreachable!(),
        });

    flow.after_translate(meta, units)
}
