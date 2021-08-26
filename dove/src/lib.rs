//! Move compiler.

#![deny(missing_docs)]

/// Dove build version
pub const DOVE_VERSION: &str = git_hash::crate_version!();
/// Dove short hash of the commit
pub const DOVE_HASH: &str = git_hash::git_hash_short_as_str!();

/// Version and Tag (v###) for PONT STDLIB
pub const PONT_STDLIB_VERSION: &str = "v0.2.1";
/// GIT URL for PONT STDLIB
pub const PONT_STDLIB_URL: &str = "https://github.com/pontem-network/move-stdlib";

/// DIEM version|branch
pub const DIEM_VERSION: &str = git_hash::dependency_branch_from_cargo_lock!("move-stdlib");
/// DIEM short hash of the commit
pub const DIEM_HASH: &str = git_hash::dependency_git_short_hash_from_cargo_lock!("move-stdlib");

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
