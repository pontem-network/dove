/// Dove build version
pub const DOVE_VERSION: &str = git_hash::crate_version!();
/// Dove short hash of the commit
pub const DOVE_HASH: &str = git_hash::git_hash_short_as_str!();

/// Version and Tag (v###) for MOVE STDLIB
pub const MOVE_STDLIB_VERSION: &str = "release-v1.0.0";

/// DIEM version|branch
pub const DIEM_VERSION: &str = git_hash::dependency_branch_from_cargo_lock!("move-cli");
/// DIEM short hash of the commit
pub const DIEM_HASH: &str = git_hash::dependency_git_short_hash_from_cargo_lock!("move-cli");

const ERROR_DESCRIPTIONS: &[u8] = include_bytes!("./error_description.errmap");

#[macro_use]
extern crate anyhow;

use std::fs::create_dir;
use std::path::PathBuf;
use anyhow::Result;

/// Transactions.
pub mod call;
/// Dove cli interface.
pub mod cli;
/// Dove commands handler.
pub mod cmd;
/// Dove execution context.
pub mod context;
/// Native functions.
pub mod natives;
/// To work with stored access keys
pub mod wallet_key;

pub mod publish;

/// Get the location of the ".move" directory.
/// Default: ~/.move/
/// If the directory "~/.move/" does not exist, it will be created.
pub fn dot_move_folder() -> Result<PathBuf> {
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
