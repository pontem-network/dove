use std::collections::HashSet;
use std::rc::Rc;

use anyhow::{Error, Result};

use lang::compiler::file::MoveFile;

use crate::context::{Context, get_context, load_manifest};
use std::path::PathBuf;

/// Project builder.
pub mod build;
/// Project dependencies loader.
pub mod clean;
/// Create transaction.
pub mod ct;
/// Dependencies fetcher.
pub mod fetch;
/// Project initializer.
pub mod init;
/// Project metadata.
pub mod metadata;
/// Project creator.
pub mod new;
/// Script executor.
pub mod run;
/// Test runner.
pub mod test;

/// Move command.
pub trait Cmd {
    /// Returns project context.
    /// This function must be overridden if the command is used with a custom context.
    fn context(&self, project_dir: PathBuf) -> Result<Context> {
        let manifest = load_manifest(&project_dir)?;
        get_context(project_dir, manifest)
    }

    /// Apply command with given context.
    fn apply(self, ctx: Context) -> Result<()>
    where
        Self: std::marker::Sized;

    /// Functions create execution context and apply command with it.
    fn execute(self, project_dir: PathBuf) -> Result<()>
    where
        Self: std::marker::Sized,
    {
        let context = self.context(project_dir)?;
        self.apply(context)
    }
}

/// Load dependencies by set of path.
pub fn load_dependencies(
    path_set: HashSet<Rc<str>>,
) -> Result<Vec<MoveFile<'static, 'static>>, Error> {
    path_set
        .iter()
        .map(|path| path.as_ref())
        .map(MoveFile::load)
        .collect()
}
