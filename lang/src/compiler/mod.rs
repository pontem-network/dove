use std::collections::HashSet;

pub use anyhow::Result;
use codespan_reporting::diagnostic::{Diagnostic, Label};
use itertools::Itertools;
use move_core_types::account_address::AccountAddress;
use move_lang::{cfgir, FullyCompiledProgram, move_continue_up_to, Pass, PassResult};
use move_lang::compiled_unit::CompiledUnit;
use move_lang::errors::Errors;
use move_lang::shared::Address;
use move_model::model::GlobalEnv;
use move_model::run_spec_checker;

use parser::parse_program;

use crate::compiler::dialects::Dialect;
use crate::compiler::file::MoveFile;
use crate::compiler::parser::{Comments, parse_target, ParserArtifact, ParsingMeta};

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
        env: Option<GlobalEnv>,
        translation_result: Result<Vec<CompiledUnit>, Errors>,
    ) -> A;
}

pub enum Step<A, N> {
    Stop(A),
    Next(N),
}

pub fn compile<A>(
    dialect: &dyn Dialect,
    targets: &[&MoveFile],
    sender: Option<AccountAddress>,
    mut flow: impl CompileFlow<A>,
    create_env: bool,
) -> A {
    // Init compiler flow.
    flow.init(dialect, sender);

    let mut env = if create_env {
        Some(GlobalEnv::new())
    } else {
        None
    };

    // Parse target.
    let (ast, deps) =
        match flow.after_parse_target(parse_target(dialect, targets, sender, create_env)) {
            Step::Stop(artifact) => return artifact,
            Step::Next(target_with_deps) => target_with_deps,
        };

    let source_keys = ast
        .meta
        .source_map
        .keys()
        .map(|key| key.to_owned())
        .collect::<HashSet<_>>();

    // Parse program.
    let program = match deps {
        None => ast,
        Some(deps) => parse_program(dialect, ast, &deps, sender, create_env),
    };

    if let Some(env) = env.as_mut() {
        for fname in program.meta.source_map.keys().sorted() {
            let fsrc = &program.meta.source_map[fname];
            env.add_source(fname, fsrc, !source_keys.contains(fname));
        }

        for (fname, documentation) in &program.meta.comments {
            if let Comments::MatchedCommentMap(matched) = documentation {
                let file_id = env.get_file_id(fname).expect("file name defined");
                env.add_documentation(file_id, matched.to_owned());
            }
        }
    }

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
                Pass::Expansion,
            )
        })
        .and_then(|pass_res| match pass_res {
            PassResult::Expansion(eprog, eerrors) => {
                Ok((eprog.clone(), PassResult::Expansion(eprog, eerrors)))
            }
            _ => unreachable!(),
        })
        .and_then(|(eprog, pass_res)| {
            move_continue_up_to(precompiled.as_ref(), pass_res, Pass::CFGIR)
                .map(|pass| (eprog, pass))
        })
        .map(|(eprog, res)| match res {
            PassResult::CFGIR(cfgir) => (eprog, cfgir),
            _ => unreachable!(),
        });

    let (eprog, check_result) = match check_result {
        Ok((eprog, cfgir)) => (Some(eprog), Ok(cfgir)),
        Err(errs) => (None, Err(errs)),
    };

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

    if let Some(env) = env.as_mut() {
        match &units {
            Ok(units) => {
                if let Some(eprog) = eprog {
                    run_spec_checker(env, units.clone(), eprog);
                }
            }
            Err(errs) => {
                add_move_lang_errors(env, meta.offsets_map.transform(errs.to_owned()));
            }
        }
    }

    flow.after_translate(meta, env, units)
}

fn add_move_lang_errors(env: &mut GlobalEnv, errors: Errors) {
    let mk_label = |env: &mut GlobalEnv, err: (move_ir_types::location::Loc, String)| {
        let loc = env.to_loc(&err.0);
        Label::new(loc.file_id(), loc.span(), err.1)
    };
    for mut error in errors {
        let primary = error.remove(0);
        let diag = Diagnostic::new_error("", mk_label(env, primary))
            .with_secondary_labels(error.into_iter().map(|e| mk_label(env, e)));
        env.add_diag(diag);
    }
}
