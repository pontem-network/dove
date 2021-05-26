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
/// Dove modules index.
pub mod index;
/// Dove configuration.
pub mod manifest;
