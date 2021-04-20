pub use anyhow::Result;
use move_core_types::account_address::AccountAddress;
use move_lang::{cfgir, FullyCompiledProgram, move_continue_up_to, Pass, PassResult};
use move_lang::compiled_unit::CompiledUnit;
use move_lang::errors::Errors;
pub use move_lang::name_pool::ConstPool;
use move_lang::shared::Address;

use parser::parse_program;

use crate::compiler::dialects::Dialect;
use crate::compiler::file::MoveFile;
use crate::compiler::parser::{parse_target, ParserArtifact, ParsingMeta};

pub mod address;
pub mod dialects;
pub mod error;
pub mod file;
pub mod location;
pub mod mut_string;
pub mod parser;
pub mod source_map;

pub type SourceDeps = Vec<MoveFile<'static, 'static>>;

pub type CheckerResult = Result<cfgir::ast::Program, Errors>;
pub type SourceWithDeps = (ParserArtifact, Option<SourceDeps>);

pub trait CompileFlow<A> {
    fn init(&mut self, _dialect: &dyn Dialect, _sender: Option<AccountAddress>) {}
    fn after_parse_target(&mut self, parser_artifact: ParserArtifact) -> Step<A, SourceWithDeps> {
        Step::Next((parser_artifact, None))
    }
    fn after_parse_program(
        &mut self,
        parser_artifact: ParserArtifact,
    ) -> Step<A, (ParserArtifact, Option<FullyCompiledProgram>)> {
        Step::Next((parser_artifact, None))
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
    sender: Option<AccountAddress>,
    mut flow: impl CompileFlow<A>,
) -> A {
    // Init compiler flow.
    flow.init(dialect, sender);

    // Parse target.
    let (ast, deps) = match flow.after_parse_target(parse_target(dialect, targets, sender)) {
        Step::Stop(artifact) => return artifact,
        Step::Next(target_with_deps) => target_with_deps,
    };

    // Parse program.
    let program = match deps {
        None => ast,
        Some(deps) => parse_program(dialect, ast, &deps, sender),
    };
    let (ast, precompiled) = match flow.after_parse_program(program) {
        Step::Stop(artifact) => return artifact,
        Step::Next((ast, precompiled)) => (ast, precompiled),
    };

    let ParserArtifact {
        meta,
        result: pprog_res,
    } = ast;
    let sender = sender.map(|addr| Address::new(addr.to_u8()));

    // Check program.
    let check_result = pprog_res
        .and_then(|pprog| {
            move_continue_up_to(
                precompiled.as_ref(),
                PassResult::Parser(sender, pprog),
                Pass::CFGIR,
            )
        })
        .map(|res| match res {
            PassResult::CFGIR(cfgir) => cfgir,
            _ => unreachable!(),
        });

    let (meta, check_result) = match flow.after_check(meta, check_result) {
        Step::Stop(artifact) => return artifact,
        Step::Next(res) => res,
    };

    // Translate program.
    let units = check_result
        .and_then(|check_result| {
            move_continue_up_to(
                precompiled.as_ref(),
                PassResult::CFGIR(check_result),
                Pass::Compilation,
            )
        })
        .map(|res| match res {
            PassResult::Compilation(units) => units,
            _ => unreachable!(),
        });

    flow.after_translate(meta, units)
}
