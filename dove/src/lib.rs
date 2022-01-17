//! Move compiler.
#![deny(missing_docs)]

/// Dove build version
pub const DOVE_VERSION: &str = git_hash::crate_version!();
/// Dove short hash of the commit
pub const DOVE_HASH: &str = git_hash::git_hash_short_as_str!();

/// Version and Tag (v###) for MOVE STDLIB
pub const MOVE_STDLIB_VERSION: &str = "670135efefd82b24d86ad4370ffc65fdfe8e483c";
/// GIT URL for MOVE STDLIB
pub const MOVE_STDLIB_URL: &str = "https://github.com/pontem-network/move-stdlib";

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
/// CLI color
pub mod colorize;
/// Dove execution context.
pub mod context;
/// Export Dove.toml => Move.toml
pub mod export;
/// Transactions.
pub mod tx;
