use std::path::{PathBuf, Path};
use crate::manifest::{Git, MANIFEST, read_manifest};
use crate::context::Context;
use git2::build::RepoBuilder;
use tiny_keccak::{Sha3, Hasher};
use anyhow::Error;
use git2::{Repository, Oid};
use crate::index::move_dir_iter;
use libra::account::AccountAddress;
use crate::index::meta::{source_meta, FileMeta};

/// Git prefix.
pub const PREFIX: &str = "git";

/// Returns module path by its identifier.
/// Downloads a modules from git if it is not in the cache.
pub fn resolve(ctx: &Context, git: &Git) -> Result<PathBuf, Error> {
    let deps = ctx.path_for(&ctx.manifest.layout.target_deps);
    let repo_path = deps.join(make_local_name(&git));
    if !repo_path.exists() {
        let repo = clone(&git, &repo_path)?;
        if let Some(branch_name) = &git.branch {
            let head = repo.head()?;
            let oid = head
                .target()
                .ok_or_else(|| anyhow!("Failed to take repo {} head.", git.git))?;
            let commit = repo.find_commit(oid)?;

            repo.branch(branch_name, &commit, false)?;
            let obj = repo.revparse_single(&("refs/heads/".to_owned() + branch_name))?;
            repo.checkout_tree(&obj, None)?;
            repo.set_head(&("refs/heads/".to_owned() + branch_name))?;
        } else if let Some(rev) = &git.rev {
            let oid = Oid::from_str(rev)?;
            let commit = repo.find_commit(oid)?;

            repo.branch(rev, &commit, false)?;

            let obj = repo.revparse_single(&("refs/heads/".to_owned() + rev))?;
            repo.checkout_tree(&obj, None)?;
            repo.set_head(&("refs/heads/".to_owned() + rev))?;
        }
    }
    Ok(repo_path)
}

/// Index of git dependencies.
pub struct GitIndex<'a> {
    ctx: &'a Context,
    path: &'a Path,
}

impl<'a> GitIndex<'a> {
    /// Create a new `GitIndex` instance.
    pub fn new(ctx: &'a Context, path: &'a Path) -> GitIndex<'a> {
        GitIndex { ctx, path }
    }

    /// Returns all metadata of this `ChainIndex`.
    pub fn meta(&self) -> Result<Vec<FileMeta>, Error> {
        let dep_address = get_dep_address(self.path)?;

        move_dir_iter(self.path)
            .map(|path| source_meta(path.path(), dep_address, self.ctx.dialect.as_ref()))
            .collect()
    }
}

fn get_dep_address(path: &Path) -> Result<Option<AccountAddress>, Error> {
    let manifest = path.join(MANIFEST);
    if manifest.exists() {
        let manifest = read_manifest(&manifest)?;
        Ok(Some(manifest.package.account_address))
    } else {
        Ok(None)
    }
}

fn make_local_name(git: &Git) -> String {
    let mut digest = Sha3::v256();
    digest.update(git.git.as_bytes());
    if let Some(branch) = &git.branch {
        digest.update(branch.as_bytes());
    }
    if let Some(rev) = &git.rev {
        digest.update(rev.as_bytes());
    }
    let mut output = [0; 32];
    digest.finalize(&mut output);
    format!("{}_{}", PREFIX, hex::encode(&output))
}

fn clone(git: &Git, path: &Path) -> Result<Repository, Error> {
    println!("Download:[{}]", git.git);
    RepoBuilder::new()
        .clone(&git.git, path)
        .map_err(|err| anyhow!("Failed to clone repository :[{}]:{}", git.git, err))
}
