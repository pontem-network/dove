use move_lang::callback::Interact;
use crate::compiler::dependency::DependencySource;
use move_lang::parser::ast::Definition;
use crate::compiler::intern_table::InternTable;
use move_core_types::language_storage::ModuleId;
use lang::compiler::preprocessor::BuilderPreprocessor;
use lang::compiler::dialects::Dialect;
use std::borrow::Cow;
use anyhow::Error;
use crate::compiler::source_map::SourceMap;

pub struct CompilerInteract<'a> {
    source_map: SourceMap,
    intern_table: InternTable,
    preprocessor: BuilderPreprocessor<'a>,
}

impl<'a> CompilerInteract<'a> {
    pub fn new(dialect: &'a dyn Dialect, sender: &'a str, source_map: SourceMap) -> CompilerInteract<'a> {
        CompilerInteract {
            source_map,
            intern_table: Default::default(),
            preprocessor: BuilderPreprocessor::new(dialect, sender),
        }
    }
}

impl<'a> Interact for CompilerInteract<'a> {
    fn is_native_fs(&self) -> bool {
        false
    }

    fn static_str(&mut self, val: String) -> &'static str {
        self.intern_table.push(val)
    }

    fn file_access(&mut self, name: &'static str) -> Result<String, Error> {
        let file = self.source_map.get(name).ok_or_else(|| anyhow::anyhow!("File {} not found", name))?;
        Ok(self.preprocessor.preprocess(name, Cow::Borrowed(file)).into_owned())
    }

    fn preprocess<'b>(&mut self, name: &'static str, source: Cow<'b, str>) -> Cow<'b, str> {
        self.preprocessor.preprocess(name, source)
    }

    fn analyze_ast(&mut self, _: &[Definition]) {}

    fn required_dependencies(&mut self) -> Vec<ModuleId> {
        vec![]
    }
}