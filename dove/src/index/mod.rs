use std::collections::HashSet;
use std::fs;
use std::fs::{OpenOptions, remove_dir_all};
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
use itertools::Itertools;

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
            roots.insert(path.clone());
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

    /// removing unused external dependencies
    pub fn remove_unused(&self, diff: Vec<Diff>) -> Result<(), Error> {
        for need_delete in diff
            .iter()
            .filter(|diff| diff.is_deleted())
            .map(|diff| diff.to_pathbuf())
            .filter(|path| is_external_git(path))
            .filter_map(|path| {
                path.components()
                    .enumerate()
                    .filter_map(|(num, part)| {
                        part.as_os_str().to_str().and_then(|n| {
                            if n.starts_with("git_") {
                                Some(num)
                            } else {
                                None
                            }
                        })
                    })
                    .last()
                    .map(|num| path.components().take(num + 1).collect::<PathBuf>())
            })
            .filter(|path| path.exists() && path.is_dir())
        {
            remove_dir_all(&need_delete)?
        }

        Ok(())
    }

    /// remove unnecessary elements in dependencies
    /// leave only *.move and *.toml
    pub fn remove_unnecessary_elements_in_dependencies(&self) {
        fn remove(path: PathBuf) {
            let ans = if path.is_file()
                && !path
                    .extension()
                    .and_then(|t| t.to_str())
                    .map(|t| t.to_lowercase())
                    .map_or(false, |ex| vec!["toml", "move"].contains(&ex.as_str()))
            {
                std::fs::remove_file(path)
            } else if path.is_dir() {
                path.read_dir()
                    .map(|r| r.filter_map(|t| t.ok()).map(|t| t.path()).for_each(remove))
            } else {
                Ok(())
            };

            if let Err(err) = ans {
                println!("Warning: {:?}", err);
            }
        }

        self.deps_roots
            .iter()
            .map(PathBuf::from)
            .filter(|path| is_external_git(path))
            .for_each(remove);
    }

    /// difference in indexes
    pub fn diff<'a>(&'a self, index: &'a Index) -> Vec<Diff<'a>> {
        let ours = &self.deps_roots;
        let theirs = &index.deps_roots;

        let mut diff: Vec<Diff> = ours
            .iter()
            .filter_map(|path| {
                if theirs.contains(path) {
                    None
                } else {
                    Some(Diff::Deleted(path))
                }
            })
            .collect();

        diff.extend(theirs.iter().filter_map(|path| {
            if ours.contains(path) {
                None
            } else {
                Some(Diff::Added(path))
            }
        }));

        diff
    }
}

/// difference in indexes
#[derive(Debug, Eq, PartialEq)]
pub enum Diff<'a> {
    /// Added the path to the index
    Added(&'a str),
    /// The path was removed from the index
    Deleted(&'a str),
}

impl<'a> Diff<'a> {
    /// Check if the path to delete is marked
    pub fn is_deleted(&self) -> bool {
        matches!(self, Diff::Deleted(_))
    }

    /// convert to path
    pub fn to_pathbuf(&self) -> PathBuf {
        let path = match self {
            Diff::Deleted(path) => path,
            Diff::Added(path) => path,
        };
        PathBuf::from(path)
    }
}

/// Is the directory of the downloaded repository
fn is_external_git(path: &Path) -> bool {
    let path_components = path
        .components()
        .filter_map(|el| el.as_os_str().to_str())
        .collect::<Vec<&str>>();
    path_components
        .iter()
        .find_position(|el| el == &&".external")
        .and_then(|(pos, _)| path_components.get(pos + 1))
        .map_or(false, |el| el.starts_with("git_"))
}

#[test]
fn test_diff_in_index() {
    for (vec_a, variants) in &[
        (
            vec!["a", "b"],
            vec![
                (Vec::new(), vec![Diff::Deleted("a"), Diff::Deleted("b")]),
                (vec!["a", "b"], vec![]),
                (vec!["b", "a"], vec![]),
                (vec!["b", "a", "c"], vec![Diff::Added("c")]),
                (vec!["a"], vec![Diff::Deleted("b")]),
                (vec!["b"], vec![Diff::Deleted("a")]),
                (vec!["c", "a"], vec![Diff::Deleted("b"), Diff::Added("c")]),
                (vec!["b", "c"], vec![Diff::Deleted("a"), Diff::Added("c")]),
            ],
        ),
        (
            Vec::new(),
            vec![
                (Vec::new(), Vec::new()),
                (vec!["a"], vec![Diff::Added("a")]),
                (vec!["b", "c"], vec![Diff::Added("b"), Diff::Added("c")]),
            ],
        ),
    ] {
        let index_a = Index {
            package_hash: "".to_string(),
            deps_roots: vec_a.iter().map(|t| t.to_string()).collect(),
        };
        for (vec_b, diffs) in variants {
            let index_b = Index {
                package_hash: "".to_string(),
                deps_roots: vec_b.iter().map(|t| t.to_string()).collect(),
            };

            assert_eq!(&index_a.diff(&index_b), diffs);
        }
    }
}
