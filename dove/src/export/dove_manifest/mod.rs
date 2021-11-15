use std::{fmt, fs};
use std::path::{MAIN_SEPARATOR as MS, Path};
use serde::{Deserialize, Deserializer};
use serde::de::{Error as DeError, SeqAccess, Visitor};
use toml::Value;
use anyhow::Error;
use boogie_backend::options::BoogieOptions;
use docgen_options::DocgenOptions;

pub mod docgen_options;

/// Dove manifest.
#[derive(Deserialize, Debug, Default)]
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
#[derive(Deserialize, Debug)]
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

/// Project layout.
#[derive(Deserialize, Debug)]
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

    /// Path to prover settings
    #[serde(default = "prover_toml")]
    pub prover_toml: String,

    /// Path to the project's system folder
    #[serde(default = "system_folder")]
    pub system_folder: String,

    /// Path to project map
    #[serde(default = "project_map")]
    pub project_map: String,
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

/// Git dependencies.
#[derive(Deserialize, Debug)]
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

/// Local dependencies path.
#[derive(Deserialize, Debug)]
pub struct DepPath {
    /// Path to the directory with local dependencies.
    pub path: String,
}

/// Chain dependency.
#[derive(Deserialize, Debug)]
pub struct Chain {
    /// Module full name.
    pub address: String,
    /// Module full name.
    pub name: String,
}

/// Project dependencies.
#[derive(Debug, Default)]
pub struct Dependencies {
    /// Vector of project dependencies.
    pub deps: Vec<Dependence>,
}

/// External dependencies enum.
#[derive(Deserialize, Debug)]
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

/// Reads the manifest by path.
pub fn read_manifest(path: &Path) -> Result<DoveToml, Error> {
    Ok(toml::from_str(&fs::read_to_string(path)?)?)
}

/// Default dialect name (pont).
pub fn default_dialect() -> String {
    "pont".to_owned()
}
