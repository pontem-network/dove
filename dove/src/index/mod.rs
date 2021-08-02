use std::collections::HashSet;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use anyhow::Error;
use diem_crypto_derive::{BCSCryptoHash, CryptoHasher};
use serde::{Deserialize, Serialize};

use resolver::git;

use crate::context::Context;
use crate::index::resolver::chain;
use crate::manifest::{Dependence, Git, MANIFEST, read_manifest};

/// Dependency resolver.
pub mod resolver;

/// Modules index.
#[derive(
    Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Default, CryptoHasher, BCSCryptoHash,
)]
pub struct Index {
    /// Package hash.
    pub package_hash: String,
    /// Modules index.
    pub deps_roots: Vec<String>,
}

impl Index {
    /// Load index form disk.
    pub fn load(path: &Path) -> Result<Option<Index>, Error> {
        Ok(if path.exists() {
            let content = fs::read_to_string(path)?;
            if let Ok(index) = toml::from_str::<Index>(&content) {
                Some(index)
            } else {
                fs::remove_file(path)?;
                None
            }
        } else {
            None
        })
    }

    /// Returns dependencies root.
    pub fn into_deps_roots(self) -> Vec<String> {
        self.deps_roots
    }

    /// Build index.
    pub fn build(package_hash: String, ctx: &Context) -> Result<Index, Error> {
        let deps_roots = if let Some(dependencies) = &ctx.manifest.package.dependencies {
            let mut deps_roots = HashSet::new();
            let mut deps_set = HashSet::new();

            Self::load_deps(
                &mut deps_roots,
                &dependencies.deps,
                ctx,
                ctx,
                true,
                &mut deps_set,
            )?;

            deps_roots
                .into_iter()
                .map(|p| p.to_string_lossy().to_string())
                .collect::<Vec<_>>()
        } else {
            vec![]
        };

        Ok(Index {
            package_hash,
            deps_roots,
        })
    }

    fn load_deps(
        roots: &mut HashSet<PathBuf>,
        deps: &[Dependence],
        root_ctx: &Context,
        ctx: &Context,
        is_root: bool,
        deps_set: &mut HashSet<u64>,
    ) -> Result<(), Error> {
        for dep in deps {
            match dep {
                Dependence::Git(git) => {
                    if deps_set.insert(git.id()) {
                        Self::load_git_deps(roots, git, root_ctx, ctx, deps_set)?;
                    }
                }
                Dependence::Path(path) => {
                    let path = PathBuf::from_str(&path.path)?;
                    let path = if path.is_absolute() {
                        path
                    } else {
                        ctx.path_for(path)
                    };

                    let path = path.canonicalize()?;

                    if !is_root && !path.starts_with(ctx.project_dir.canonicalize()?) {
                        return Err(anyhow!("A local dependency in an external package cannot be referenced outside of the package."));
                    }

                    if !path.exists() {
                        return Err(anyhow!("Unresolved dependencies path:{:?}", path));
                    }
                    roots.insert(path);
                }
                Dependence::Chain(chain) => {
                    chain::resolve(root_ctx, &chain.module_id(root_ctx)?)?;
                }
            }
        }

        Ok(())
    }

    fn load_git_deps(
        roots: &mut HashSet<PathBuf>,
        git: &Git,
        root_ctx: &Context,
        ctx: &Context,
        deps_set: &mut HashSet<u64>,
    ) -> Result<HashSet<PathBuf>, Error> {
        let mut result = HashSet::new();
        let path = git::resolve(root_ctx, git)?;

        let manifest = path.join(MANIFEST);
        if manifest.exists() {
            match read_manifest(&manifest) {
                Ok(manifest) => {
                    let ctx = Context {
                        project_dir: path,
                        manifest,
                        dialect: ctx.dialect.copy(),
                    };
                    Self::load_dove_as_deps(roots, root_ctx, &ctx, deps_set)
                        .map_err(|err| anyhow!("Failed to load {}: Err:{}", git.git, err))?;
                }
                Err(err) => {
                    return Err(anyhow!(
                        "Dependency {} has invalid Dove.toml. Err:{}",
                        git.git,
                        err
                    ));
                }
            }
        } else {
            result.insert(path);
        }
        Ok(result)
    }

    fn load_dove_as_deps(
        roots: &mut HashSet<PathBuf>,
        root_ctx: &Context,
        ctx: &Context,
        deps_set: &mut HashSet<u64>,
    ) -> Result<(), Error> {
        if let Some(dependencies) = &ctx.manifest.package.dependencies {
            Self::load_deps(roots, &dependencies.deps, root_ctx, ctx, false, deps_set)?;
        }

        let modules_dir = ctx.path_for(&ctx.manifest.layout.modules_dir);
        if modules_dir.exists() {
            roots.insert(modules_dir);
        }

        Ok(())
    }

    /// Store index to the disk.
    pub fn store(&self, path: &Path) -> Result<(), Error> {
        let value = toml::to_vec(self)?;

        if let Some(dir) = path.parent() {
            if !dir.exists() {
                fs::create_dir_all(dir)?;
            }
        }

        let mut f = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .open(path)?;
        f.set_len(0)?;
        f.write_all(&value)?;

        Ok(())
    }
}
