use lang::compiler::file;
use lang::compiler::file::MoveFile;
use lang::flow::builder::StaticResolver;
use lang::flow::checker::MoveChecker;

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

    pub fn check_file(&self, file: MoveFile) -> Option<FileDiagnostic> {
        match self.check_file_inner(file) {
            Ok(_) => None,
            Err(mut ds) => Some(ds.remove(0)),
        }
    }

    fn check_file_inner(&self, current_file: MoveFile) -> Result<(), Vec<FileDiagnostic>> {
        let deps = self
            .read_stdlib_files()
            .into_iter()
            .chain(
                self.db
                    .module_files()
                    .into_iter()
                    .map(|(name, text)| MoveFile::with_content(name, text)),
            )
            .filter(|file| file.name() != current_file.name())
            .collect::<Vec<MoveFile<'static, 'static>>>();

        let resolver = StaticResolver::new(deps);
        MoveChecker::new(
            self.db.config.dialect().as_ref(),
            Some(self.db.config.sender()),
            resolver,
        )
        .check(&[&current_file])
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

    fn read_stdlib_files(&self) -> Vec<MoveFile<'static, 'static>> {
        self.db
            .config
            .stdlib_folder
            .as_ref()
            .map(|folder| file::load_move_files(&[folder]).unwrap_or_default())
            .unwrap_or_default()
    }
}
