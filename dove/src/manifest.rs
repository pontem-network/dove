use anyhow::Error;
use std::path::Path;
use std::{fs, fmt};
use toml::Value;
use serde::{Deserializer, Serializer, Serialize, Deserialize};
use serde::{
    de::{Visitor, SeqAccess, Error as DeError},
    ser::{Error as SerError},
};
use libra::prelude::AccountAddress;
use lang::compiler::dialects::dfinance::DFinanceDialect;
use lang::compiler::dialects::Dialect;
use libra::prelude::CORE_CODE_ADDRESS;

/// Dove manifest name.
pub const MANIFEST: &str = "Dove.toml";

/// Movec manifest.
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
    /// Project AccountAddress.
    #[serde(default = "code_code_address")]
    #[serde(deserialize_with = "from_str")]
    pub account_address: AccountAddress,
    /// Authors list.
    #[serde(default)]
    pub authors: Vec<String>,
    /// dnode base url.
    pub blockchain_api: Option<String>,
    /// Dependency list.
    pub dependencies: Option<Dependencies>,
}

impl Default for Package {
    fn default() -> Self {
        Package {
            name: None,
            account_address: CORE_CODE_ADDRESS,
            authors: Default::default(),
            blockchain_api: None,
            dependencies: None,
        }
    }
}

fn module_dir() -> String {
    "modules".to_owned()
}

fn script_dir() -> String {
    "scripts".to_owned()
}

fn tests_dir() -> String {
    "tests".to_owned()
}

fn module_output() -> String {
    "target/modules".to_owned()
}

fn script_output() -> String {
    "target/scripts".to_owned()
}

fn target_deps() -> String {
    "target/.external".to_owned()
}

fn target() -> String {
    "target".to_owned()
}

fn index() -> String {
    ".Dove.man".to_owned()
}

fn code_code_address() -> AccountAddress {
    CORE_CODE_ADDRESS
}

/// Project layout.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Layout {
    /// Directory with module sources.
    #[serde(default = "module_dir")]
    pub module_dir: String,

    /// Directory with script sources.
    #[serde(default = "script_dir")]
    pub script_dir: String,

    /// Directory with tests.
    #[serde(default = "tests_dir")]
    pub tests_dir: String,

    /// Directory with compiled modules.
    #[serde(default = "module_output")]
    pub module_output: String,

    /// Directory with compiled scripts.
    #[serde(default = "script_output")]
    pub script_output: String,

    /// Directory with external dependencies.
    #[serde(default = "target_deps")]
    pub target_deps: String,

    /// Target directory.
    #[serde(default = "target")]
    pub target: String,

    /// Path to index.
    pub index: String,
}

impl Default for Layout {
    fn default() -> Self {
        Layout {
            module_dir: module_dir(),
            script_dir: script_dir(),
            tests_dir: tests_dir(),
            module_output: module_output(),
            script_output: script_output(),
            target_deps: target_deps(),
            target: target(),
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
}

/// Local dependencies path.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct DepPath {
    /// Path to the directory with local dependencies.
    pub path: String,
}

/// Project dependencies.
#[derive(Debug, Clone, PartialEq, Eq)]
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

fn from_str<'de, D>(deserializer: D) -> Result<AccountAddress, D::Error>
where
    D: Deserializer<'de>,
{
    DFinanceDialect::default()
        .normalize_account_address(&String::deserialize(deserializer)?)
        .map(|addr| addr.as_account_address())
        .map_err(D::Error::custom)
}

/// Reads the manifest by path.
pub fn read_manifest(path: &Path) -> Result<DoveToml, Error> {
    Ok(toml::from_str(&fs::read_to_string(path)?)?)
}

#[cfg(test)]
mod test {
    use crate::manifest::{Package, Dependence, Git, Dependencies, DepPath};
    use libra::prelude::CORE_CODE_ADDRESS;

    fn package() -> Package {
        Package {
            name: Some("Foo".to_owned()),
            account_address: CORE_CODE_ADDRESS,
            authors: vec![],
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
                    }),
                    Dependence::Git(Git {
                        git: "https://github.com/dfinance/move-stdlib".to_owned(),
                        branch: Some("master".to_owned()),
                        rev: Some("969442fb28fc162c3e3de20ab0a3afdfa8d0f560".to_owned()),
                    }),
                ],
            }),
        }
    }

    #[test]
    fn parse_deps() {
        let deps = "
                        account_address = \"0x01\"
                        name = \"Foo\"
                        dependencies = [
                            {path = \"/stdlib\"},
                            {git = \"https://github.com/dfinance/move-stdlib\"},
                            {git = \"https://github.com/dfinance/move-stdlib\", \
                            branch = \"master\", rev = \"969442fb28fc162c3e3de20ab0a3afdfa8d0f560\"}
                        ]";
        assert_eq!(package(), toml::from_str::<Package>(deps).unwrap());
    }
}
