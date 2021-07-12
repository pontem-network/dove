use move_lang::parser::ast::Definition;
use crate::compiler::file::MoveFile;
use move_lang::errors::Error;
use move_lang::{FullyCompiledProgram, parser};

pub use crate::compiler::CompileFlow;

pub mod builder;
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
