use lsp_types::Diagnostic;
use move_lang::parser::ast::FileDefinition;

use crate::compiler::{check, parse_file};
use crate::ide::db::{AnalysisChange, FilePath, RootDatabase};

#[derive(Debug, Default)]
pub struct AnalysisHost {
    db: RootDatabase,
}

impl AnalysisHost {
    pub fn analysis(&self) -> Analysis {
        Analysis {
            db: self.db.clone(),
        }
    }

    pub fn db(&self) -> &RootDatabase {
        &self.db
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
    pub fn check_with_libra_compiler(&self, fpath: FilePath, text: &str) -> Vec<Diagnostic> {
        let main_file = match self.parse(fpath, text) {
            Ok(file) => file,
            Err(diags) => {
                return diags;
            }
        };

        // TODO: build dependency graph and only load required deps
        let dependencies: Vec<FileDefinition> = self
            .db
            .project_files_mapping
            .iter()
            .filter(|(file_fpath, _)| **file_fpath != fpath)
            .map(|(fpath, text)| self.parse(fpath, &text).unwrap())
            .collect();

        let errors =
            check::check_parsed_program(main_file, dependencies, Some(self.db.sender_address));
        match errors {
            Err(errors) => errors
                .into_iter()
                .map(|error| self.db.libra_error_into_diagnostic(error))
                .collect(),
            Ok(_) => vec![],
        }
    }

    pub fn parse(&self, fpath: FilePath, text: &str) -> Result<FileDefinition, Vec<Diagnostic>> {
        parse_file(fpath, text).map_err(|err| vec![self.db.libra_error_into_diagnostic(err)])
    }
}
