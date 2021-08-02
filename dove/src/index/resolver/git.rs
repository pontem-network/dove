use std::convert::TryFrom;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::Error;
use fs_extra::dir::CopyOptions;
use git2::{Oid, Repository};
use git2::build::RepoBuilder;

use crate::context::Context;
use crate::manifest::{CheckoutParams, Git};
use crate::stdoutln;

/// Git prefix.
pub const PREFIX: &str = "git";

/// Returns module path by its identifier.
/// Downloads a modules from git if it is not in the cache.
pub fn resolve(ctx: &Context, git: &Git) -> Result<PathBuf, Error> {
    let checkout_params = CheckoutParams::try_from(git)?;

    let deps = ctx.path_for(&ctx.manifest.layout.deps);
    let local_name = git.local_name()?;
    let mut repo_path = deps.join(&local_name);

    if !repo_path.exists() {
        if git.path.is_some() {
            repo_path = repo_path.join("._tmp_dove_checkout_dir_");
        }

        checkout(checkout_params, &repo_path).map_err(|err| {
            if repo_path.exists() {
                if let Err(err) = fs::remove_dir_all(&repo_path) {
                    stdoutln!("Warning: {:?} {}", repo_path.display(), err.to_string());
                }
            }
            err
        })?;

        if let Err(err) = fs::remove_dir_all(&repo_path.join(".git")) {
            warn!("Failed to remove .git in repo {:?}. {}", repo_path, err);
        }

        if let Some(path_in_repo) = &git.path {
            let source_path = repo_path
                .join(&path_in_repo)
                .canonicalize()
                .map_err(|err| anyhow!("Invalid path in git repo.{} [{}]", git.git, err))?;

            if !source_path.starts_with(&repo_path) {
                if let Some(repo_path) = repo_path.parent() {
                    fs::remove_dir_all(&repo_path)?;
                }
                return Err(anyhow!(
                    "Invalid path in git repo.{} [Path is output of git directory]",
                    git.git
                ));
            }
            let target_path = deps.join(&local_name);
            if source_path.is_file() {
                fs::copy(source_path, target_path)?;
            } else {
                for entry in fs::read_dir(source_path)? {
                    let source_path = entry?.path();
                    if source_path.is_file() {
                        if let Some(name) = source_path.file_name() {
                            fs::copy(&source_path, target_path.join(name))?;
                        }
                    } else {
                        fs_extra::dir::move_dir(&source_path, &target_path, &CopyOptions::new())?;
                    }
                }
            }
            fs::remove_dir_all(&repo_path)?;
        }
    }

    Ok(repo_path)
}

fn checkout(params: CheckoutParams<'_>, path: &Path) -> Result<(), Error> {
    let to_path: PathBuf;

    #[cfg(target_os = "windows")]
    {
        use rand::random;
        to_path = std::env::temp_dir().join(format!("git_{}", random::<u64>()));
    }

    #[cfg(not(target_os = "windows"))]
    {
        to_path = path.to_path_buf();
    }

    let repo = clone(&params, &to_path)?;

    match params {
        CheckoutParams::Branch { repo: _, branch } => {
            if let Some(branch_name) = branch {
                let refs = format!("refs/remotes/origin/{}", branch_name);

                let head = repo.head()?;
                let oid = head
                    .target()
                    .ok_or_else(|| anyhow!("Failed to take repo {} head.", params.repo()))?;
                let commit = repo.find_commit(oid)?;

                repo.branch(branch_name, &commit, false)?;

                let obj = repo.revparse_single(&refs)?;
                repo.checkout_tree(&obj, None)?;
                repo.set_head(&refs)?;
            }
        }
        CheckoutParams::Rev { repo: _, rev } => {
            let oid = Oid::from_str(rev)?;
            let commit = repo.find_commit(oid)?;

            repo.branch(rev, &commit, false)?;

            let refs = format!("refs/heads/{}", rev);

            let obj = repo.revparse_single(&refs)?;
            repo.checkout_tree(&obj, None)?;
            repo.set_head(&refs)?;
        }
        CheckoutParams::Tag {
            repo: _,
            tag: tg_name,
        } => {
            let references = repo.references()?;
            let refs = format!("refs/tags/{}", tg_name);

            let mut finded = false;
            for reference in references.flatten() {
                if reference.is_tag() {
                    if let Some(tag_ref) = reference.name() {
                        if tag_ref == refs {
                            let commit = reference.peel_to_commit()?;

                            repo.branch(tg_name, &commit, false)?;

                            let obj = repo.revparse_single(&refs)?;
                            repo.checkout_tree(&obj, None)?;
                            repo.set_head(&refs)?;
                            finded = true;
                            break;
                        }
                    }
                }
            }
            if !finded {
                return Err(anyhow!("Tag {} not found.", tg_name));
            }
        }
    }

    #[cfg(target_family = "windows")]
    {
        if !path.exists() {
            fs::create_dir_all(&path)?;
        }
        copy_dir_all(&to_path, &path)?;
        readonly_false(&path)?;
    }

    Ok(())
}

fn clone(git: &CheckoutParams, path: &Path) -> Result<Repository, Error> {
    stdoutln!("Download:[{}]", git.repo());

    RepoBuilder::new()
        .clone(git.repo(), path)
        .map_err(|err| anyhow!("Failed to clone repository :[{}]:{}", git.repo(), err))
}

#[cfg(target_os = "windows")]
fn readonly_false(dirpath: &Path) -> std::io::Result<()> {
    let mut p = dirpath.metadata()?.permissions();
    p.set_readonly(false);
    fs::set_permissions(dirpath, p)?;
    if !dirpath.is_dir() {
        return Ok(());
    }

    for npatn in dirpath.read_dir()? {
        readonly_false(npatn?.path().as_path())?;
    }
    Ok(())
}

#[cfg(target_os = "windows")]
fn copy_dir_all(src: &Path, dst: &Path) -> std::io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(&entry.path(), &dst.join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), &dst.join(entry.file_name()))?;
        }
    }
    Ok(())
}
