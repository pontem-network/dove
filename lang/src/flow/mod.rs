use crate::compiler::file::MoveFile;
use move_lang::errors::Error;
use move_lang::{FullyCompiledProgram, parser};
use move_lang::parser::ast::{Program, Definition};

pub use crate::compiler::CompileFlow;

pub mod checker;

pub trait DependencyResolver {
    fn resolve_source_deps(
        &mut self,
        ast: &[Definition],
    ) -> Result<Option<Vec<MoveFile<'static, 'static>>>, Error>;

    fn resolve_precompiled(
        &mut self,
        ast: &parser::ast::Program,
    ) -> Result<Option<FullyCompiledProgram>, Error>;
}

pub struct StaticResolver {
    deps: Vec<MoveFile<'static, 'static>>,
}

impl StaticResolver {
    pub fn new(deps: Vec<MoveFile<'static, 'static>>) -> StaticResolver {
        StaticResolver { deps }
    }
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
