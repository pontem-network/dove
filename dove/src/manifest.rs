use std::{fmt, fs};
use std::convert::TryFrom;
use std::path::Path;

use anyhow::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::{
    de::{Error as DeError, SeqAccess, Visitor},
    ser::Error as SerError,
};
use toml::Value;
use move_core_types::language_storage::CORE_CODE_ADDRESS;
use move_lang::shared::Address;
use crate::context::Context;

/// Dove manifest name.
pub const MANIFEST: &str = "Dove.toml";

/// Dove manifest.
#[derive(Deserialize, Serialize, Debug, Clone, Default, PartialEq, Eq)]
pub struct DoveToml {
    /// Project info.
    pub package: Package,
    /// Project layout.
    #[serde(default)]
    pub layout: Layout,
}

/// Project info.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Package {
    /// Project name.
    pub name: Option<String>,
    /// Project account address.
    #[serde(default = "code_code_address")]
    pub account_address: String,
    /// dnode base url.
    pub blockchain_api: Option<String>,
    /// Dependency list.
    pub dependencies: Option<Dependencies>,
    /// Dialect
    #[serde(default = "dialect")]
    pub dialect: Option<String>,
}

impl Default for Package {
    fn default() -> Self {
        Package {
            name: None,
            account_address: code_code_address(),
            blockchain_api: None,
            dependencies: None,
            dialect: None,
        }
    }
}

#[allow(clippy::unnecessary_wraps)]
fn dialect() -> Option<String> {
    Some(default_dialect())
}

fn modules_dir() -> String {
    "modules".to_owned()
}

fn scripts_dir() -> String {
    "scripts".to_owned()
}

fn tests_dir() -> String {
    "tests".to_owned()
}

fn modules_output() -> String {
    "artifacts/modules".to_owned()
}

fn scripts_output() -> String {
    "artifacts/scripts".to_owned()
}

fn transactions_output() -> String {
    "artifacts/transactions".to_owned()
}

fn bundles_output() -> String {
    "artifacts/bundles".to_owned()
}

fn move_prover_output() -> String {
    "artifacts/move_prover".to_owned()
}

fn deps() -> String {
    "artifacts/.external".to_owned()
}

fn artifacts() -> String {
    "artifacts".to_owned()
}

fn index() -> String {
    "artifacts/.Dove.man".to_owned()
}

fn code_code_address() -> String {
    Address::new(CORE_CODE_ADDRESS.to_u8()).to_string()
}

/// Project layout.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Layout {
    /// Directory with module sources.
    #[serde(default = "modules_dir")]
    pub modules_dir: String,

    /// Directory with script sources.
    #[serde(default = "scripts_dir")]
    pub scripts_dir: String,

    /// Directory with tests.
    #[serde(default = "tests_dir")]
    pub tests_dir: String,

    /// Directory with compiled modules.
    #[serde(default = "modules_output")]
    pub modules_output: String,

    /// Directory with module package.
    #[serde(default = "bundles_output")]
    pub bundles_output: String,

    /// Directory with compiled scripts.
    #[serde(default = "scripts_output")]
    pub scripts_output: String,

    /// Directory with transactions.
    #[serde(default = "transactions_output")]
    pub transactions_output: String,

    /// Directory with move-prover intermediate artifacts.
    #[serde(default = "move_prover_output")]
    pub move_prover_output: String,

    /// Directory with external dependencies.
    #[serde(default = "deps")]
    pub deps: String,

    /// Artifacts directory.
    #[serde(default = "artifacts")]
    pub artifacts: String,

    /// Path to index.
    #[serde(default = "index")]
    pub index: String,
}

impl Layout {
    /// Returns layout instance with absolute paths.
    pub fn to_absolute(&self, ctx: &Context) -> Result<Layout, Error> {
        Ok(Layout {
            modules_dir: ctx.str_path_for(&self.modules_dir)?,
            scripts_dir: ctx.str_path_for(&self.scripts_dir)?,
            tests_dir: ctx.str_path_for(&self.tests_dir)?,
            modules_output: ctx.str_path_for(&self.modules_output)?,
            bundles_output: ctx.str_path_for(&self.bundles_output)?,
            scripts_output: ctx.str_path_for(&self.scripts_output)?,
            transactions_output: ctx.str_path_for(&self.transactions_output)?,
            move_prover_output: ctx.str_path_for(&self.move_prover_output)?,
            deps: ctx.str_path_for(&self.deps)?,
            artifacts: ctx.str_path_for(&self.artifacts)?,
            index: ctx.str_path_for(&self.index)?,
        })
    }
}

impl Default for Layout {
    fn default() -> Self {
        Layout {
            modules_dir: modules_dir(),
            scripts_dir: scripts_dir(),
            tests_dir: tests_dir(),
            modules_output: modules_output(),
            bundles_output: bundles_output(),
            scripts_output: scripts_output(),
            transactions_output: transactions_output(),
            move_prover_output: move_prover_output(),
            deps: deps(),
            artifacts: artifacts(),
            index: index(),
        }
    }
}

/// Git dependencies.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Git {
    /// Git url.
    pub git: String,
    /// Branch name.
    pub branch: Option<String>,
    /// Commit hash.
    pub rev: Option<String>,
    /// Tag.
    pub tag: Option<String>,
    /// Path.
    pub path: Option<String>,
}

/// Type of git dependency check out.
#[derive(Debug, PartialEq, Eq)]
pub enum CheckoutParams<'a> {
    /// Checkout branch.
    Branch {
        /// Repository git url.
        repo: &'a str,
        /// Branch name to checkout.
        branch: Option<&'a String>,
    },
    /// Checkout revision.
    Rev {
        /// Repository git url.
        repo: &'a str,
        /// Commit hash to checkout.
        rev: &'a str,
    },
    /// Checkout tag.
    Tag {
        /// Repository git url.
        repo: &'a str,
        /// Tag to checkout.
        tag: &'a str,
    },
}

impl CheckoutParams<'_> {
    /// Returns repository url.
    pub fn repo(&self) -> &str {
        match self {
            CheckoutParams::Branch { repo, branch: _ } => repo,
            CheckoutParams::Rev { repo, rev: _ } => repo,
            CheckoutParams::Tag { repo, tag: _ } => repo,
        }
    }
}

impl<'a> TryFrom<&'a Git> for CheckoutParams<'a> {
    type Error = Error;

    fn try_from(dep: &'a Git) -> Result<Self, Self::Error> {
        fn error(git: &str) -> Error {
            anyhow!("dependency ({}) specification is ambiguous. Only one of `branch`, `tag` or `rev` is allowed.", git)
        }

        if let Some(tag) = &dep.tag {
            if dep.branch.is_some() || dep.rev.is_some() {
                Err(error(&dep.git))
            } else {
                Ok(CheckoutParams::Tag {
                    repo: &dep.git,
                    tag,
                })
            }
        } else if let Some(rev) = &dep.rev {
            if dep.branch.is_some() {
                Err(error(&dep.git))
            } else {
                Ok(CheckoutParams::Rev {
                    repo: &dep.git,
                    rev,
                })
            }
        } else {
            Ok(CheckoutParams::Branch {
                repo: &dep.git,
                branch: dep.branch.as_ref(),
            })
        }
    }
}

/// Local dependencies path.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct DepPath {
    /// Path to the directory with local dependencies.
    pub path: String,
}

/// Project dependencies.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Dependencies {
    /// Vector of project dependencies.
    pub deps: Vec<Dependence>,
}

/// External dependencies enum.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Dependence {
    /// Git dependency.
    Git(Git),
    /// Local dependency.
    Path(DepPath),
}

impl<'de> Deserialize<'de> for Dependencies {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        struct DependenciesVisitor();
        impl<'de> Visitor<'de> for DependenciesVisitor {
            type Value = Dependencies;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "vector of dependencies")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut deps = Vec::with_capacity(seq.size_hint().unwrap_or(10));
                while let Some(ele) = seq.next_element::<Value>()? {
                    if let Some(tbl) = ele.as_table() {
                        if tbl.contains_key("git") {
                            deps.push(Dependence::Git(
                                Git::deserialize(ele).map_err(DeError::custom)?,
                            ));
                        } else {
                            deps.push(Dependence::Path(
                                DepPath::deserialize(ele).map_err(DeError::custom)?,
                            ));
                        }
                    } else {
                        return Err(DeError::unknown_variant(ele.type_str(), &["struct"]));
                    }
                }

                Ok(Dependencies { deps })
            }
        }
        deserializer.deserialize_seq(DependenciesVisitor())
    }
}

impl Serialize for Dependencies {
    fn serialize<S>(&self, s: S) -> Result<<S as Serializer>::Ok, S::Error>
    where
        S: Serializer,
    {
        s.collect_seq(
            self.deps
                .iter()
                .map(|dep| match dep {
                    Dependence::Path(path) => Value::try_from(path),
                    Dependence::Git(git) => Value::try_from(git),
                })
                .collect::<Result<Vec<_>, _>>()
                .map_err(SerError::custom)?,
        )
    }
}

/// Reads the manifest by path.
pub fn read_manifest(path: &Path) -> Result<DoveToml, Error> {
    Ok(toml::from_str(&fs::read_to_string(path)?)?)
}

/// Default dialect name (pont).
pub fn default_dialect() -> String {
    "pont".to_owned()
}

#[cfg(test)]
mod test {
    use crate::manifest::{Dependence, Dependencies, DepPath, Git, Package, DoveToml};

    fn package() -> Package {
        Package {
            name: Some("Foo".to_owned()),
            account_address: "0x01".to_owned(),
            blockchain_api: None,
            dependencies: Some(Dependencies {
                deps: vec![
                    Dependence::Path(DepPath {
                        path: "/stdlib".to_owned(),
                    }),
                    Dependence::Git(Git {
                        git: "https://github.com/dfinance/move-stdlib".to_owned(),
                        branch: None,
                        rev: None,
                        tag: None,
                        path: None,
                    }),
                    Dependence::Git(Git {
                        git: "https://github.com/dfinance/move-stdlib".to_owned(),
                        branch: Some("master".to_owned()),
                        rev: Some("969442fb28fc162c3e3de20ab0a3afdfa8d0f560".to_owned()),
                        tag: None,
                        path: Some("/lang".to_owned()),
                    }),
                ],
            }),
            dialect: Some("dfinance".to_owned()),
        }
    }

    #[test]
    fn parse_deps() {
        let deps = r#"
                        account_address = "0x01"
                        name = "Foo"
                        dependencies = [
                            {path = "/stdlib"},
                            {git = "https://github.com/dfinance/move-stdlib"},
                            {git = "https://github.com/dfinance/move-stdlib", branch = "master", rev = "969442fb28fc162c3e3de20ab0a3afdfa8d0f560", path = "/lang"}
                        ]
                        dialect= "dfinance"
                        "#;
        assert_eq!(package(), toml::from_str::<Package>(deps).unwrap());
    }

    #[test]
    fn parse_layout() {
        let dove_toml = r#"
                        [package]
                            name = "test_name"
                            dialect = "pont"
                            dependencies = [
                            ]
                        [layout]
                        tests_dir = "runner_tests"
                        "#;
        let mut expected = DoveToml::default();

        expected.package.name = Some("test_name".to_owned());
        expected.package.dialect = Some("pont".to_owned());
        expected.package.dependencies = Some(Dependencies { deps: vec![] });
        expected.layout.tests_dir = "runner_tests".to_owned();

        assert_eq!(expected, toml::from_str::<DoveToml>(dove_toml).unwrap());
    }
}
