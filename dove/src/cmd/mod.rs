use anyhow::Result;

use crate::context::Context;

/// Create transaction.
pub mod call;
/// Project dependencies loader.
pub mod clean;
/// Project builder.
pub mod deploy;
/// Script executor.
pub mod run;

pub trait Cmd {
    /// Apply command with given context.
    fn apply(&mut self, ctx: &mut Context) -> Result<()>;
}
