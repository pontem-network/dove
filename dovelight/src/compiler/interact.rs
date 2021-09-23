use move_lang::callback::Interact;
use move_lang::parser::ast::Definition;
use crate::compiler::intern_table::InternTable;
use lang::compiler::preprocessor::BuilderPreprocessor;
use lang::compiler::dialects::Dialect;
use std::borrow::Cow;
use anyhow::Error;
use crate::compiler::source_map::SourceMap;
use crate::deps::extractor::ImportsExtractor;
use core::mem;
use crate::deps::{DependencyLoader, Store};
use crate::deps::resolver::DependencyResolver;
use move_lang::errors::{Errors, FilesSourceText};

const PREFIX: &str = "dep_";

pub struct CompilerInteract<'a, L: DependencyLoader, S: Store> {
    source_map: SourceMap,
    intern_table: InternTable,
    preprocessor: BuilderPreprocessor<'a>,
    dependency_extractor: ImportsExtractor,
    dependence_resolver: DependencyResolver<'a, L, S>,
}

impl<'a, L: DependencyLoader, S: Store> CompilerInteract<'a, L, S> {
    pub fn new(
        dialect: &'a dyn Dialect,
        sender: &'a str,
        source_map: SourceMap,
        dependence_resolver: DependencyResolver<'a, L, S>,
    ) -> CompilerInteract<'a, L, S> {
        CompilerInteract {
            source_map,
            intern_table: Default::default(),
            preprocessor: BuilderPreprocessor::new(dialect, sender),
            dependency_extractor: Default::default(),
            dependence_resolver,
        }
    }

    pub fn sources(&mut self) -> FilesSourceText {
        self.preprocessor.into_source()
    }

    pub fn transform(&self, errors: Errors) -> Errors {
        self.preprocessor.transform(errors)
    }
}

impl<'a, L: DependencyLoader, S: Store> Interact for CompilerInteract<'a, L, S> {
    fn is_native_fs(&self) -> bool {
        false
    }

    fn static_str(&mut self, val: String) -> &'static str {
        self.intern_table.push(val)
    }

    fn file_access(&mut self, name: &'static str) -> Result<String, Error> {
        if name.starts_with(PREFIX) {
            self.dependence_resolver
                .load_interface(&name[PREFIX.len()..])
                .map(|(_, content)| content)
        } else {
            let file = self
                .source_map
                .get(name)
                .ok_or_else(|| anyhow::anyhow!("File {} not found", name))?;
            Ok(self
                .preprocessor
                .preprocess(name, Cow::Borrowed(file))
                .into_owned())
        }
    }

    fn preprocess<'b>(&mut self, name: &'static str, source: Cow<'b, str>) -> Cow<'b, str> {
        self.preprocessor.preprocess(name, source)
    }

    fn analyze_ast(&mut self, def: &[Definition]) {
        self.dependency_extractor.extract(def);
    }

    fn required_dependencies(&mut self) -> Result<Vec<String>, Error> {
        let extractor = mem::take(&mut self.dependency_extractor);
        let usages = extractor.finish();
        self.dependence_resolver.load_tree(usages)
    }
}
