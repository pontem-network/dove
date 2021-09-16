use move_lang::shared::Flags;

use lang::compiler::{check, file};
use std::path::PathBuf;
use crate::inner::db::{FileDiagnostic, RootDatabase};

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

    pub fn check_file(&self, file: String) -> Option<FileDiagnostic> {
        match self.check_file_inner(file) {
            Ok(_) => None,
            Err(mut ds) => Some(ds.remove(0)),
        }
    }

    fn check_file_inner(&self, current_file: String) -> Result<(), Vec<FileDiagnostic>> {
        let deps = self
            .read_stdlib_files()
            .into_iter()
            .map(|path| path.to_string_lossy().to_string())
            .chain(self.db.module_files().into_iter())
            .filter(|file| file != &current_file)
            .collect::<Vec<String>>();

        check(
            &[current_file],
            &deps,
            self.db.config.dialect().as_ref(),
            self.db.config.sender(),
            None,
            Flags::empty(),
        )
        .unwrap()
        .1
        .map_err(|errors| {
            errors
                .into_iter()
                .map(|err| match self.db.make_diagnostic(err.clone()) {
                    Ok(d) => d,
                    Err(error) => panic!(
                        "While converting {:#?} into Diagnostic, error occurred: {:?}",
                        err,
                        error.to_string()
                    ),
                })
                .collect()
        })
    }

    fn read_stdlib_files(&self) -> Vec<PathBuf> {
        self.db
            .config
            .stdlib_folder
            .as_ref()
            .map(|folder| file::find_move_files(&[folder]).collect())
            .unwrap_or_default()
    }
}
