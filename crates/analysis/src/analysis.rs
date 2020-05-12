use lsp_types::CompletionItem;

use crate::change::AnalysisChange;
use crate::completion;
use crate::db::{FileDiagnostic, RootDatabase};
use crate::utils::io;
use dialects::{dfinance, FilePath};

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

    pub fn parse(
        &self,
        fpath: FilePath,
        text: &str,
    ) -> Result<Vec<dfinance::types::Definition>, Vec<FileDiagnostic>> {
        dialects::dfinance::parse_file(fpath, text).map_err(|err| {
            err.into_iter()
                .filter_map(|err| match self.db.libra_error_into_diagnostic(err) {
                    Ok(d) => Some(d),
                    Err(err) => {
                        log::error!("{}", err);
                        None
                    }
                })
                .collect()
        })
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
        let current_file_defs = self.parse(current_fpath, current_text)?;
        let mut deps = vec![];
        for (fpath, source_text) in self
            .read_stdlib_files()
            .into_iter()
            .chain(self.db.module_files().into_iter())
        {
            if fpath != current_fpath {
                let defs = self.parse(fpath, &source_text)?;
                deps.extend(defs);
            }
        }

        dfinance::check_parsed_program(current_file_defs, deps, self.db().sender_address())
            .map_err(|errors| {
                errors
                    .into_iter()
                    .filter_map(|err| match self.db.libra_error_into_diagnostic(err) {
                        Ok(d) => Some(d),
                        Err(err) => {
                            log::error!("{}", err);
                            None
                        }
                    })
                    .collect()
            })
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
