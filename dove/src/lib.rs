//! Move compiler.
#![deny(missing_docs)]

/// Dove build version
pub const DOVE_VERSION: &str = git_hash::crate_version!();
/// Dove short hash of the commit
pub const DOVE_HASH: &str = git_hash::git_hash_short_as_str!();

/// Version and Tag (v###) for MOVE STDLIB
pub const MOVE_STDLIB_VERSION: &str = "release-v1.0.0";
/// GIT URL for MOVE STDLIB
pub const MOVE_STDLIB_URL: &str = "https://github.com/pontem-network/move-stdlib";

/// DIEM version|branch
pub const DIEM_VERSION: &str = git_hash::dependency_branch_from_cargo_lock!("move-cli");
/// DIEM short hash of the commit
pub const DIEM_HASH: &str = git_hash::dependency_git_short_hash_from_cargo_lock!("move-cli");

#[macro_use]
extern crate anyhow;

use std::fs::create_dir;
use std::path::PathBuf;
use anyhow::Result;

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
/// To work with stored access keys
pub mod secret_phrase;
/// Transactions.
pub mod tx;

/// Get the location of the ".move" directory.
/// Default: ~/.move/
/// If the directory "~/.move/" does not exist, it will be created.
pub fn move_folder() -> Result<PathBuf> {
    let move_home_string = std::env::var("MOVE_HOME").unwrap_or_else(|_| {
        format!(
            "{}/.move",
            std::env::var("HOME").expect("env var 'HOME' must be set")
        )
    });
    let move_home_path = PathBuf::from(move_home_string);
    if !move_home_path.exists() {
        create_dir(&move_home_path)?;
    }
    Ok(move_home_path)
}
