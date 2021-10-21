use std::{fmt, fs};
use std::collections::hash_map::DefaultHasher;
use std::convert::TryFrom;
use std::hash::{Hash, Hasher};
use tiny_keccak::{Sha3, Hasher as tiny_keccak_hasher};
use std::path::{MAIN_SEPARATOR as MS, Path};

use anyhow::Error;
use diem_crypto_derive::{BCSCryptoHash, CryptoHasher};
use move_core_types::identifier::Identifier;
use move_core_types::language_storage::ModuleId;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::{
    de::{Error as DeError, SeqAccess, Visitor},
    ser::Error as SerError,
};
use toml::Value;

use crate::context::Context;
use crate::docs::options::DocgenOptions;
use http::Uri;
use boogie_backend::options::BoogieOptions;

/// Dove manifest name.
pub const MANIFEST: &str = "Dove.toml";

/// Dove manifest.
#[derive(Deserialize, Serialize, Debug, Clone, Default, Eq, PartialEq)]
pub struct DoveToml {
    /// Project info.
    pub package: Package,
    /// Project layout.
    #[serde(default)]
    pub layout: Layout,
    /// Documentation generator operations.
    #[serde(default)]
    pub doc: DocgenOptions,
    /// Boogie Options
    pub boogie_options: Option<BoogieOptions>,
}

/// Project info.
#[derive(Deserialize, Serialize, Debug, Clone, CryptoHasher, BCSCryptoHash, PartialEq, Eq)]
pub struct Package {
    /// Project name.
    pub name: Option<String>,
    /// Project account address.
    #[serde(default = "code_code_address")]
    pub account_address: String,
    /// Minimal dove version.
    pub dove_version: Option<String>,
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
            dove_version: None,
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

fn prover_toml() -> String {
    "prover-env.toml".to_owned()
}

fn modules_output() -> String {
    format!("{}{}{}", artifacts(), MS, "modules")
}

fn scripts_output() -> String {
    format!("{}{}{}", artifacts(), MS, "scripts")
}

fn transactions_output() -> String {
    format!("{}{}{}", artifacts(), MS, "transactions")
}

fn bundles_output() -> String {
    format!("{}{}{}", artifacts(), MS, "bundles")
}

fn move_prover_output() -> String {
    format!("{}{}{}", artifacts(), MS, "move_prover")
}

fn docs_output() -> String {
    "doc".to_owned()
}

fn deps() -> String {
    format!("{}{}{}", artifacts(), MS, ".external")
}

fn chain_deps() -> String {
    format!("{}{}{}", deps(), MS, "chain")
}

fn artifacts() -> String {
    "artifacts".to_owned()
}

fn index() -> String {
    format!("{}{}{}", system_folder(), MS, ".DoveIndex.toml")
}

fn storage_dir() -> String {
    format!("{}{}{}", artifacts(), MS, "storage")
}

fn exe_build_dir() -> String {
    format!("{}{}{}", artifacts(), MS, "exe_build")
}

fn code_code_address() -> String {
    "0x1".to_string()
}

fn system_folder() -> String {
    format!("{}{}{}", artifacts(), MS, ".system")
}

fn project_map() -> String {
    format!("{}{}{}", system_folder(), MS, "project.map")
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

    /// Directory with move documentation.
    #[serde(default = "docs_output")]
    pub docs_output: String,

    /// Directory with external dependencies.
    #[serde(default = "deps")]
    pub deps: String,

    /// Directory with external chain dependencies.
    #[serde(default = "chain_deps")]
    pub chain_deps: String,

    /// Artifacts directory.
    #[serde(default = "artifacts")]
    pub artifacts: String,

    /// Path to index.
    #[serde(default = "index")]
    pub index: String,

    /// Path to executor directory.
    #[serde(default = "storage_dir")]
    pub storage_dir: String,

    /// Path to executor build directory.
    #[serde(default = "exe_build_dir")]
    pub exe_build_dir: String,

    /// Path to pover settings
    #[serde(default = "prover_toml")]
    pub prover_toml: String,

    /// Path t
    #[serde(default = "system_folder")]
    pub system_folder: String,

    /// Path to project map
    #[serde(default = "project_map")]
    pub project_map: String,
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
            docs_output: ctx.str_path_for(&self.docs_output)?,
            deps: ctx.str_path_for(&self.deps)?,
            chain_deps: ctx.str_path_for(&self.chain_deps)?,
            artifacts: ctx.str_path_for(&self.artifacts)?,
            index: ctx.str_path_for(&self.index)?,
            storage_dir: ctx.str_path_for(&self.storage_dir)?,
            exe_build_dir: ctx.str_path_for(&self.exe_build_dir)?,
            prover_toml: ctx.str_path_for(&self.prover_toml)?,
            system_folder: ctx.str_path_for(&self.system_folder)?,
            project_map: ctx.str_path_for(&self.project_map)?,
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
            docs_output: docs_output(),
            deps: deps(),
            chain_deps: chain_deps(),
            artifacts: artifacts(),
            index: index(),
            storage_dir: storage_dir(),
            exe_build_dir: exe_build_dir(),
            prover_toml: prover_toml(),
            system_folder: system_folder(),
            project_map: project_map(),
        }
    }
}

/// Git dependencies.
#[derive(
    Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash, CryptoHasher, BCSCryptoHash,
)]
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

impl Git {
    /// Returns a git dependency identifier.
    /// Now it uses hashing to calculate the ID.
    pub fn id(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }

    /// Returns unique repository name for git repository.
    pub fn local_name(&self) -> Result<String, Error> {
        Ok(format!("git_{}", self.local_hash_repository()?))
    }

    /// Returns unique hash repository for git repository.
    fn local_hash_repository(&self) -> Result<String, Error> {
        let mut digest = Sha3::v256();

        digest.update(self.git_name_normalization_for_hash()?.as_bytes());

        if let Some(branch) = &self.branch {
            digest.update(format!("b_{}", branch).as_bytes());
        }
        if let Some(rev) = &self.rev {
            digest.update(format!("r_{}", rev).as_bytes());
        }
        if let Some(path) = &self.path {
            digest.update(format!("p_{}", path).as_bytes());
        }
        if let Some(tag) = &self.tag {
            digest.update(format!("t_{}", tag).as_bytes());
        }
        let mut output = [0; 32];
        digest.finalize(&mut output);
        Ok(hex::encode(&output))
    }

    fn git_name_normalization_for_hash(&self) -> Result<String, Error> {
        let mut git_str = self.git.trim().to_string();
        if git_str.find("http") == Some(0) {
            let uri = git_str.parse::<Uri>()?;
            git_str = format!(
                "{}{}",
                uri.host().unwrap_or_default(),
                uri.path_and_query()
                    .map(|s| s.to_string())
                    .unwrap_or_default()
            );
        } else if git_str.find("git@") == Some(0) {
            if let Some(pos) = git_str.find(':') {
                git_str.replace_range(pos..pos + 1, "/");
            }
            git_str = git_str[4..].to_string();
        }

        if git_str.len() > 4 && &git_str[git_str.len() - 4..] == ".git" {
            git_str = git_str[..git_str.len() - 4].to_string();
        }
        Ok(git_str)
    }
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
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, CryptoHasher, BCSCryptoHash)]
pub struct DepPath {
    /// Path to the directory with local dependencies.
    pub path: String,
}

/// Chain dependency.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, CryptoHasher, BCSCryptoHash)]
pub struct Chain {
    /// Module full name.
    pub address: String,
    /// Module full name.
    pub name: String,
}

impl Chain {
    /// Return module id.
    pub fn module_id(&self, ctx: &Context) -> Result<ModuleId, Error> {
        Ok(ModuleId::new(
            ctx.dialect.parse_address(&self.address)?,
            Identifier::new(self.name.to_owned())?,
        ))
    }
}

impl AsRef<str> for DepPath {
    fn as_ref(&self) -> &str {
        &self.path
    }
}

/// Project dependencies.
#[derive(Debug, Clone, PartialEq, Eq, Default, CryptoHasher, BCSCryptoHash)]
pub struct Dependencies {
    /// Vector of project dependencies.
    pub deps: Vec<Dependence>,
}

/// External dependencies enum.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, CryptoHasher, BCSCryptoHash)]
pub enum Dependence {
    /// Git dependency.
    Git(Git),
    /// Local dependency.
    Path(DepPath),
    /// Chain dependency.
    Chain(Chain),
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
                        } else if tbl.contains_key("name") {
                            deps.push(Dependence::Chain(
                                Chain::deserialize(ele).map_err(DeError::custom)?,
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
                    Dependence::Chain(chain) => Value::try_from(chain),
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
    use crate::manifest::{Dependence, Dependencies, DepPath, DoveToml, Git, Package};

    fn package() -> Package {
        Package {
            name: Some("Foo".to_owned()),
            account_address: "0x01".to_owned(),
            dove_version: None,
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

    #[test]
    fn git_local_hash() {
        let h1 = Git {
            git: "https://github.com/pontem-network/move-stdlib.git".to_string(),
            path: None,
            branch: None,
            rev: None,
            tag: None,
        }
        .local_hash_repository()
        .unwrap();

        assert_eq!(
            h1,
            Git {
                git: "https://github.com/pontem-network/move-stdlib".to_string(),
                path: None,
                branch: None,
                rev: None,
                tag: None,
            }
            .local_hash_repository()
            .unwrap()
        );

        assert_eq!(
            h1,
            Git {
                git: "http://github.com/pontem-network/move-stdlib".to_string(),
                path: None,
                branch: None,
                rev: None,
                tag: None,
            }
            .local_hash_repository()
            .unwrap()
        );

        assert_eq!(
            h1,
            Git {
                git: "git@github.com:pontem-network/move-stdlib".to_string(),
                path: None,
                branch: None,
                rev: None,
                tag: None,
            }
            .local_hash_repository()
            .unwrap()
        );

        fn get_ob(num: u8) -> String {
            let mut o = Git {
                git: "git@github.com:pontem-network/move-stdlib".to_string(),
                path: None,
                branch: None,
                rev: None,
                tag: None,
            };
            let val = Some("1".to_string());
            match num {
                1 => o.path = val,
                2 => o.branch = val,
                3 => o.rev = val,
                4 => o.tag = val,
                _ => (),
            };
            o.local_hash_repository().unwrap()
        }
        for nh1 in 0..=4 {
            let h1 = get_ob(nh1);
            for nh2 in 0..=4 {
                let h2 = get_ob(nh2);
                if nh1 == nh2 {
                    assert_eq!(h1, h2);
                } else {
                    assert_ne!(h1, h2);
                }
            }
        }
    }
}
