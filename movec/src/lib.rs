//! Move compiler.

#![deny(missing_docs)]

#[macro_use]
extern crate anyhow;
extern crate log;

/// Move builder.
pub mod builder;
/// Movec commands handler.
pub mod cmd;
/// Dependencies loader.
pub mod dependence;
/// Movec configuration.
pub mod manifest;
