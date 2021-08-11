//! Move compiler.

#![deny(missing_docs)]

#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate log;

/// Dove cli interface.
pub mod cli;
/// Dove commands handler.
pub mod cmd;
/// Dove execution context.
pub mod context;
/// Docgen.
pub mod docs;
/// Move executor.
pub mod executor;
/// Dove modules index.
pub mod index;
/// Dove configuration.
pub mod manifest;
/// StdOut stream
pub mod stdout;
/// for tests
pub mod tests_helper;
/// Build|Run transaction. Used in "dove run" and "dove tx"
pub mod transaction;
