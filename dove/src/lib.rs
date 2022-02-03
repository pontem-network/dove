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

const ERROR_DESCRIPTIONS: &[u8] = include_bytes!("./error_description.errmap");

#[macro_use]
extern crate anyhow;

/// Dove cli interface.
pub mod cli;
/// Dove commands handler.
pub mod cmd;
/// Dove execution context.
pub mod context;

/// Native functions.
pub mod natives;

/// Transactions.
pub mod call;
