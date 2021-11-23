//! Move compiler.
#![deny(missing_docs)]

/// Dove build version
pub const DOVE_VERSION: &str = git_hash::crate_version!();
/// Dove short hash of the commit
pub const DOVE_HASH: &str = git_hash::git_hash_short_as_str!();

/// Version and Tag (v###) for PONT STDLIB
pub const PONT_STDLIB_VERSION: &str = "v0.3.0";
/// GIT URL for PONT STDLIB
pub const PONT_STDLIB_URL: &str = "https://github.com/pontem-network/move-stdlib";

/// DIEM version|branch
pub const DIEM_VERSION: &str = git_hash::dependency_branch_from_cargo_lock!("move-cli");
/// DIEM short hash of the commit
pub const DIEM_HASH: &str = git_hash::dependency_git_short_hash_from_cargo_lock!("move-cli");

#[macro_use]
extern crate anyhow;

/// Dove cli interface.
pub mod cli;
/// Dove commands handler.
pub mod cmd;
/// Dove execution context.
pub mod context;
/// Export Dove.toml => Move.toml
pub mod export;
/// StdOut stream
pub mod stdout;
/// Transactions.
pub mod tx;
