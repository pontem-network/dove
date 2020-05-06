use lsp_types::CompletionItem;
use move_lang::parser::ast::Definition;

use crate::change::AnalysisChange;
use crate::db::{FileDiagnostic, FilePath, RootDatabase};
use crate::utils::io;
use crate::{compiler, completion};

#[derive(Debug, Default)]
pub struct AnalysisHost {
    db: RootDatabase,
}

impl AnalysisHost {
    pub fn db(&self) -> &RootDatabase {
        &self.db
    }

    pub fn analysis(&self) -> Analysis {
        Analysis::new(self.db.clone())
    }

    pub fn apply_change(&mut self, change: AnalysisChange) {
        self.db.apply_change(change);
    }
}

#[derive(Debug)]
pub struct Analysis {
    db: RootDatabase,
}

impl Analysis {
    pub fn new(db: RootDatabase) -> Analysis {
        Analysis { db }
    }

    pub fn db(&self) -> &RootDatabase {
        &self.db
    }

    pub fn parse(&self, fpath: FilePath, text: &str) -> Result<Vec<Definition>, FileDiagnostic> {
        compiler::parse_file(fpath, text).map_err(|err| self.db.libra_error_into_diagnostic(err))
    }

    pub fn completions(&self) -> Vec<CompletionItem> {
        let mut completions = vec![];
        completions.extend(completion::get_keywords());
        completions.extend(completion::get_builtins());
        completions
    }

    pub fn check_with_libra_compiler(&self, fpath: FilePath, text: &str) -> Vec<FileDiagnostic> {
        match self.check_with_libra_compiler_inner(fpath, text) {
            Ok(_) => vec![],
            Err(ds) => ds,
        }
    }

    #[inline]
    fn check_with_libra_compiler_inner(
        &self,
        current_fpath: FilePath,
        current_text: &str,
    ) -> Result<(), Vec<FileDiagnostic>> {
        let current_file_defs = self
            .parse(current_fpath, current_text)
            .map_err(|d| vec![d])?;
        let mut deps = vec![];
        for (fpath, source_text) in self
            .read_stdlib_files()
            .into_iter()
            .chain(self.db.module_files().into_iter())
        {
            if fpath != current_fpath {
                let defs = self.parse(fpath, &source_text).map_err(|d| vec![d])?;
                deps.extend(defs);
            }
        }
        compiler::check_parsed_program(current_file_defs, deps, self.db().sender_address()).map_err(
            |errors| {
                errors
                    .into_iter()
                    .map(|err| self.db.libra_error_into_diagnostic(err))
                    .collect()
            },
        )
    }

    fn read_stdlib_files(&self) -> Vec<(FilePath, String)> {
        self.db
            .config
            .stdlib_folder
            .as_ref()
            .map(|folder| io::read_move_files(folder.as_path()))
            .unwrap_or_else(|| vec![])
    }
}
