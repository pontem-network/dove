use lsp_types::CompletionItem;
use move_lang::parser::ast::FileDefinition;

use crate::compiler::{check, parse_file};
use crate::completion;

use crate::ide::db::{AnalysisChange, FileDiagnostic, FilePath, RootDatabase};
use crate::utils::io;

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

    pub fn parse(&self, fpath: FilePath, text: &str) -> Result<FileDefinition, FileDiagnostic> {
        parse_file(fpath, text).map_err(|err| self.db.libra_error_into_diagnostic(err))
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
        fpath: FilePath,
        text: &str,
    ) -> Result<(), Vec<FileDiagnostic>> {
        let main_file = self.parse(fpath, text).map_err(|d| vec![d])?;

        let mut dependencies = self.parsed_stdlib_files();
        for (existing_mod_fpath, existing_mod_text) in self.db.module_files().iter() {
            if existing_mod_fpath != &fpath {
                let parsed = self
                    .parse(existing_mod_fpath, existing_mod_text)
                    .map_err(|d| vec![d])?;
                if matches!(parsed, FileDefinition::Modules(_)) {
                    dependencies.push(parsed);
                }
            }
        }
        let check_res =
            check::check_parsed_program(main_file, dependencies, Some(self.db().sender_address()));
        check_res.map_err(|errors| {
            errors
                .into_iter()
                .map(|err| self.db.libra_error_into_diagnostic(err))
                .collect()
        })
    }

    fn parsed_stdlib_files(&self) -> Vec<FileDefinition> {
        match &self.db.config.stdlib_folder {
            Some(folder) => {
                let mut parsed_mods = vec![];
                for (fpath, text) in io::get_module_files(folder.as_path()) {
                    let parsed = self.parse(fpath, &text).unwrap();
                    parsed_mods.push(parsed);
                }
                parsed_mods
            }
            None => vec![],
        }
    }
}
